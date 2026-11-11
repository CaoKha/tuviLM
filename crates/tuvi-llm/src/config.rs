//! Which provider, key, model and URL to use.

use std::fmt;

/// An LLM provider. Both speak the OpenAI chat-completions format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    OpenRouter,
    OpenCodeGo,
}

impl Provider {
    /// The value of `TUVI_LLM_PROVIDER` that selects this provider.
    pub const fn id(self) -> &'static str {
        match self {
            Self::OpenRouter => "openrouter",
            Self::OpenCodeGo => "opencode-go",
        }
    }

    const fn default_base_url(self) -> &'static str {
        match self {
            Self::OpenRouter => "https://openrouter.ai/api/v1",
            Self::OpenCodeGo => "https://opencode.ai/inference/go/openai/v1",
        }
    }

    /// Used when `TUVI_LLM_MODEL` is not set. OpenRouter keeps the model of the
    /// old Python backend. For OpenCode Go, the model must use the
    /// chat-completions format (see `GET <base_url>/models`).
    const fn default_model(self) -> &'static str {
        match self {
            Self::OpenRouter => "qwen/qwen3.7-max",
            Self::OpenCodeGo => "glm-5.3",
        }
    }

    /// The environment variables that can hold this provider's key, first wins.
    const fn key_vars(self) -> &'static [&'static str] {
        match self {
            Self::OpenRouter => &["OPENROUTER_API_KEY"],
            Self::OpenCodeGo => &["OPENCODE_API_KEY", "OPENCODE_GO_API_KEY"],
        }
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::OpenRouter => "OpenRouter",
            Self::OpenCodeGo => "OpenCode Go",
        })
    }
}

/// The settings for [`crate::LlmClient`].
#[derive(Clone, PartialEq, Eq)]
pub struct LlmConfig {
    pub provider: Provider,
    pub api_key: String,
    pub model: String,
    /// Without a trailing `/`, for example `https://openrouter.ai/api/v1`.
    pub base_url: String,
}

/// Keeps the key out of logs and panic messages.
impl fmt::Debug for LlmConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LlmConfig")
            .field("provider", &self.provider)
            .field("api_key", &"<hidden>")
            .field("model", &self.model)
            .field("base_url", &self.base_url)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    /// No key for any provider.
    NoKey,
    /// `TUVI_LLM_PROVIDER` names an unknown provider.
    UnknownProvider(String),
    /// `TUVI_LLM_PROVIDER` names a provider whose key is not set.
    MissingKey(Provider),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoKey => f.write_str(
                "no LLM key: set OPENCODE_API_KEY (OpenCode Go) or OPENROUTER_API_KEY (OpenRouter)",
            ),
            Self::UnknownProvider(value) => write!(
                f,
                "TUVI_LLM_PROVIDER={value:?} is not known: use \"opencode-go\" or \"openrouter\""
            ),
            Self::MissingKey(provider) => write!(
                f,
                "TUVI_LLM_PROVIDER={} needs {}",
                provider.id(),
                provider.key_vars().join(" or ")
            ),
        }
    }
}

impl std::error::Error for ConfigError {}

impl LlmConfig {
    /// Reads the settings from the process environment:
    ///
    /// | Variable | Meaning |
    /// | --- | --- |
    /// | `OPENCODE_API_KEY` (or `OPENCODE_GO_API_KEY`) | OpenCode Go key |
    /// | `OPENROUTER_API_KEY` | OpenRouter key |
    /// | `TUVI_LLM_PROVIDER` | `opencode-go` or `openrouter`. Optional. |
    /// | `TUVI_LLM_MODEL` | Model id. Optional. |
    /// | `TUVI_LLM_BASE_URL` | API base URL. Optional. |
    ///
    /// Without `TUVI_LLM_PROVIDER`, the provider whose key is set wins. When
    /// both keys are set, OpenCode Go wins.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_vars(|name| std::env::var(name).ok())
    }

    /// [`Self::from_env`] with a custom lookup, so tests need not touch the
    /// process environment.
    pub fn from_vars(var: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let var = |name: &str| var(name).filter(|value| !value.trim().is_empty());
        let key_of = |provider: Provider| provider.key_vars().iter().find_map(|name| var(name));

        let (provider, api_key) = match var("TUVI_LLM_PROVIDER") {
            Some(id) => {
                let provider = [Provider::OpenCodeGo, Provider::OpenRouter]
                    .into_iter()
                    .find(|provider| provider.id() == id.trim())
                    .ok_or(ConfigError::UnknownProvider(id))?;
                let key = key_of(provider).ok_or(ConfigError::MissingKey(provider))?;
                (provider, key)
            }
            None => [Provider::OpenCodeGo, Provider::OpenRouter]
                .into_iter()
                .find_map(|provider| Some((provider, key_of(provider)?)))
                .ok_or(ConfigError::NoKey)?,
        };

        Ok(Self {
            provider,
            api_key: api_key.trim().to_owned(),
            model: var("TUVI_LLM_MODEL").unwrap_or_else(|| provider.default_model().to_owned()),
            base_url: var("TUVI_LLM_BASE_URL")
                .unwrap_or_else(|| provider.default_base_url().to_owned())
                .trim_end_matches('/')
                .to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(vars: &[(&str, &str)]) -> Result<LlmConfig, ConfigError> {
        LlmConfig::from_vars(|name| {
            vars.iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| (*value).to_owned())
        })
    }

    #[test]
    fn no_key() {
        assert_eq!(config(&[]), Err(ConfigError::NoKey));
        assert_eq!(
            config(&[("OPENROUTER_API_KEY", "  ")]),
            Err(ConfigError::NoKey)
        );
    }

    #[test]
    fn opencode_go_from_its_key() {
        let Ok(config) = config(&[("OPENCODE_API_KEY", "go-key")]) else {
            panic!()
        };
        assert_eq!(config.provider, Provider::OpenCodeGo);
        assert_eq!(config.api_key, "go-key");
        assert_eq!(config.model, "glm-5.3");
        assert_eq!(
            config.base_url,
            "https://opencode.ai/inference/go/openai/v1"
        );
    }

    #[test]
    fn opencode_go_alternative_key_name() {
        let Ok(config) = config(&[("OPENCODE_GO_API_KEY", "go-key")]) else {
            panic!()
        };
        assert_eq!(config.provider, Provider::OpenCodeGo);
    }

    #[test]
    fn openrouter_from_its_key() {
        let Ok(config) = config(&[("OPENROUTER_API_KEY", "or-key")]) else {
            panic!()
        };
        assert_eq!(config.provider, Provider::OpenRouter);
        assert_eq!(config.model, "qwen/qwen3.7-max");
        assert_eq!(config.base_url, "https://openrouter.ai/api/v1");
    }

    #[test]
    fn opencode_go_wins_when_both_are_set() {
        let both = [
            ("OPENROUTER_API_KEY", "or-key"),
            ("OPENCODE_API_KEY", "go-key"),
        ];
        let Ok(config) = config(&both) else { panic!() };
        assert_eq!(config.provider, Provider::OpenCodeGo);
    }

    #[test]
    fn explicit_provider() {
        let vars = [
            ("OPENROUTER_API_KEY", "or-key"),
            ("OPENCODE_API_KEY", "go-key"),
            ("TUVI_LLM_PROVIDER", "openrouter"),
            ("TUVI_LLM_MODEL", "x/y"),
            ("TUVI_LLM_BASE_URL", "http://localhost:4000/v1/"),
        ];
        let Ok(config) = config(&vars) else { panic!() };
        assert_eq!(config.provider, Provider::OpenRouter);
        assert_eq!(config.api_key, "or-key");
        assert_eq!(config.model, "x/y");
        assert_eq!(config.base_url, "http://localhost:4000/v1");
    }

    #[test]
    fn explicit_provider_errors() {
        assert_eq!(
            config(&[("TUVI_LLM_PROVIDER", "openai"), ("OPENROUTER_API_KEY", "k")]),
            Err(ConfigError::UnknownProvider("openai".to_owned()))
        );
        assert_eq!(
            config(&[
                ("TUVI_LLM_PROVIDER", "openrouter"),
                ("OPENCODE_API_KEY", "k")
            ]),
            Err(ConfigError::MissingKey(Provider::OpenRouter))
        );
    }

    #[test]
    fn debug_hides_the_key() {
        let Ok(config) = config(&[("OPENCODE_API_KEY", "secret-value")]) else {
            panic!()
        };
        assert!(!format!("{config:?}").contains("secret-value"));
    }
}
