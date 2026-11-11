//! Chat requests in the OpenAI chat-completions format.

use std::{
    collections::VecDeque,
    fmt,
    sync::atomic::{AtomicU64, Ordering::Relaxed},
    time::{SystemTime, UNIX_EPOCH},
};

use futures_util::{Stream, StreamExt, TryStreamExt, stream};
use serde::{Deserialize, Serialize};

use crate::{LlmConfig, Provider, sse::SseParser};

/// The User-Agent of every request. OpenCode Go asks clients to name themselves.
const USER_AGENT: &str = concat!("tuvilm/", env!("CARGO_PKG_VERSION"));

/// Max characters of an error body kept in [`LlmError::Status`].
const ERROR_BODY_MAX_CHARS: usize = 500;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}

/// One conversation. OpenCode Go needs the same id on every request of a
/// conversation (`x-opencode-session`); OpenRouter ignores it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session(String);

impl Session {
    /// A new id, unique within this machine. It is not a secret.
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        let count = COUNTER.fetch_add(1, Relaxed);
        Self(format!("tuvilm-{nanos:x}-{:x}-{count}", std::process::id()))
    }

    /// Uses an id you already have, for example a saved chat id.
    pub fn from_id(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn id(&self) -> &str {
        &self.0
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum LlmError {
    /// The request did not complete: network, TLS or a broken stream.
    Http(reqwest::Error),
    /// The provider answered with an error status.
    Status { status: u16, body: String },
    /// The answer was not in the expected format.
    Format(String),
}

impl fmt::Display for LlmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(error) => write!(f, "LLM request failed: {error}"),
            Self::Status { status, body } => write!(f, "LLM provider returned {status}: {body}"),
            Self::Format(message) => write!(f, "unexpected LLM answer: {message}"),
        }
    }
}

impl std::error::Error for LlmError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Http(error) => Some(error),
            Self::Status { .. } | Self::Format(_) => None,
        }
    }
}

impl From<reqwest::Error> for LlmError {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error)
    }
}

/// Sends chats to the configured provider. Cheap to clone: the clone shares
/// the connection pool.
#[derive(Debug, Clone)]
pub struct LlmClient {
    http: reqwest::Client,
    config: LlmConfig,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: &'a [Message],
    stream: bool,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    #[serde(default)]
    message: Option<Content>,
    #[serde(default)]
    delta: Option<Content>,
}

#[derive(Deserialize)]
struct Content {
    #[serde(default)]
    content: Option<String>,
}

impl LlmClient {
    pub fn new(config: LlmConfig) -> Result<Self, LlmError> {
        let http = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
        Ok(Self { http, config })
    }

    pub fn config(&self) -> &LlmConfig {
        &self.config
    }

    /// Sends the chat and waits for the whole answer.
    pub async fn complete(
        &self,
        session: &Session,
        messages: &[Message],
    ) -> Result<String, LlmError> {
        let response = self.send(session, messages, false).await?;
        let body: ChatResponse = response.json().await?;
        body.choices
            .into_iter()
            .next()
            .and_then(|choice| choice.message?.content)
            .ok_or_else(|| LlmError::Format("no message content in the first choice".to_owned()))
    }

    /// Sends the chat and returns the answer as a stream of text pieces.
    pub async fn stream(
        &self,
        session: &Session,
        messages: &[Message],
    ) -> Result<impl Stream<Item = Result<String, LlmError>> + Send + Unpin + use<>, LlmError> {
        let response = self.send(session, messages, true).await?;
        let bytes = response.bytes_stream().map_err(LlmError::from);

        // State: the byte stream, the event parser, payloads not yet handled,
        // and whether `[DONE]` arrived.
        let state = (
            Box::pin(bytes),
            SseParser::default(),
            VecDeque::<String>::new(),
            false,
        );
        // Boxed so callers can poll it with `.next()` without pinning it.
        Ok(Box::pin(stream::try_unfold(
            state,
            |(mut bytes, mut parser, mut pending, mut done)| async move {
                loop {
                    while let Some(payload) = pending.pop_front() {
                        if payload == "[DONE]" {
                            done = true;
                            pending.clear();
                            break;
                        }
                        if let Some(text) = delta_text(&payload)? {
                            return Ok(Some((text, (bytes, parser, pending, done))));
                        }
                    }
                    if done {
                        return Ok(None);
                    }
                    let Some(chunk) = bytes.next().await.transpose()? else {
                        return Ok(None);
                    };
                    pending.extend(parser.push(&chunk));
                }
            },
        )))
    }

    async fn send(
        &self,
        session: &Session,
        messages: &[Message],
        stream: bool,
    ) -> Result<reqwest::Response, LlmError> {
        let request = ChatRequest {
            model: &self.config.model,
            messages,
            stream,
        };
        let mut builder = self
            .http
            .post(format!("{}/chat/completions", self.config.base_url))
            .bearer_auth(&self.config.api_key)
            .json(&request);
        builder = match self.config.provider {
            Provider::OpenCodeGo => builder.header("x-opencode-session", session.id()),
            // Optional app attribution on openrouter.ai.
            Provider::OpenRouter => builder.header("X-Title", "TuviLM"),
        };

        let response = builder.send().await?;
        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }
        let body = response.text().await.unwrap_or_default();
        Err(LlmError::Status {
            status: status.as_u16(),
            body: body.chars().take(ERROR_BODY_MAX_CHARS).collect(),
        })
    }
}

/// The text in one stream event, `None` when the event has no text (for
/// example the first role-only event). An `error` object becomes an error.
fn delta_text(payload: &str) -> Result<Option<String>, LlmError> {
    let value: serde_json::Value = serde_json::from_str(payload)
        .map_err(|error| LlmError::Format(format!("{error}: {payload}")))?;
    if let Some(error) = value.get("error") {
        return Err(LlmError::Format(format!("stream error: {error}")));
    }
    let chunk: ChatResponse =
        serde_json::from_value(value).map_err(|error| LlmError::Format(error.to_string()))?;
    Ok(chunk
        .choices
        .into_iter()
        .next()
        .and_then(|choice| choice.delta?.content)
        .filter(|text| !text.is_empty()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_text_cases() {
        let text = r#"{"choices":[{"delta":{"content":"Xin chào"}}]}"#;
        assert_eq!(delta_text(text).ok().flatten().as_deref(), Some("Xin chào"));
        let role_only = r#"{"choices":[{"delta":{"role":"assistant"}}]}"#;
        assert_eq!(delta_text(role_only).ok().flatten(), None);
        assert!(delta_text(r#"{"error":{"message":"rate limit"}}"#).is_err());
        assert!(delta_text("not json").is_err());
    }

    #[test]
    fn sessions_are_unique() {
        assert_ne!(Session::new(), Session::new());
    }

    #[test]
    fn request_json() {
        let messages = [Message::system("s"), Message::user("u")];
        let request = ChatRequest {
            model: "m",
            messages: &messages,
            stream: true,
        };
        let Ok(json) = serde_json::to_string(&request) else {
            panic!()
        };
        assert_eq!(
            json,
            r#"{"model":"m","messages":[{"role":"system","content":"s"},{"role":"user","content":"u"}],"stream":true}"#
        );
    }
}
