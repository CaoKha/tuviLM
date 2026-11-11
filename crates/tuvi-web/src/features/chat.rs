//! "Hỏi Thiên Hạc": the AI chat about one chart.
//!
//! - `store`: the saved messages of each chart
//! - `prompt`: the system prompt, with the whole chart as text
//! - `markdown`: safe HTML from the model's markdown
//! - `service`: the LLM client, and a guard so one chart has one answer at a time
//!
//! The whole chart goes into the prompt (about 2 KB), so the model needs no
//! tools to read it. Cách cục and the book Tử Vi Tân Biên are not in the
//! prompt yet.

mod markdown;
mod prompt;
mod service;
mod store;

pub use markdown::to_html;
pub use prompt::system_prompt;
pub use service::{ChatService, chat_service};
pub use store::{ASSISTANT, ChatMessage, USER, append, history};
