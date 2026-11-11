//! The LLM client of the app, and the "one answer per chart" guard.

use std::{collections::HashSet, sync::Mutex};

use topcoat::context::{Cx, context};
use tuvi_llm::{LlmClient, LlmConfig};

/// Shared by every request. Add it to the router with `.with(...)`.
#[derive(Debug)]
pub struct ChatService {
    client: Result<LlmClient, String>,
    /// Charts with an answer in progress.
    answering: Mutex<HashSet<u64>>,
}

impl ChatService {
    /// Reads the LLM settings from the environment (see `.env.example`). A
    /// missing key does not stop the app: the chat page shows the reason.
    pub fn from_env() -> Self {
        let client = LlmConfig::from_env()
            .map_err(|error| error.to_string())
            .and_then(|config| LlmClient::new(config).map_err(|error| error.to_string()));
        Self {
            client,
            answering: Mutex::new(HashSet::new()),
        }
    }

    /// The client, or why the chat is off.
    pub fn client(&self) -> Result<&LlmClient, &str> {
        self.client.as_ref().map_err(String::as_str)
    }

    /// Marks an answer for `chart_id` as started. `None` when one is already
    /// running, for example after a page reload. The mark goes away when the
    /// guard drops.
    pub fn begin(&self, chart_id: u64) -> Option<AnswerGuard<'_>> {
        let mut answering = self
            .answering
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        answering.insert(chart_id).then_some(AnswerGuard {
            service: self,
            chart_id,
        })
    }
}

/// Ends the "answer in progress" mark of one chart when dropped.
pub struct AnswerGuard<'a> {
    service: &'a ChatService,
    chart_id: u64,
}

impl Drop for AnswerGuard<'_> {
    fn drop(&mut self) {
        let mut answering = self
            .service
            .answering
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        answering.remove(&self.chart_id);
    }
}

pub fn chat_service(cx: &Cx) -> &ChatService {
    context::<ChatService>(cx)
}
