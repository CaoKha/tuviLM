//! LLM client for TuviLM.
//!
//! Two providers, both through the OpenAI chat-completions format:
//! - [OpenRouter](https://openrouter.ai): `OPENROUTER_API_KEY`
//! - [OpenCode Go](https://opencode.ai/docs/go): `OPENCODE_API_KEY`
//!
//! [`LlmConfig::from_env`] picks the provider from the environment.
//! [`LlmClient`] sends the chat, as one answer or as a stream of text pieces.

mod client;
mod config;
mod sse;

pub use client::{LlmClient, LlmError, Message, Role, Session};
pub use config::{ConfigError, LlmConfig, Provider};
