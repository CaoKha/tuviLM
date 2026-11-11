//! The chat messages table.

use topcoat::{Result, context::Cx};

use crate::db::db;

/// `ChatMessage::role` of a question.
pub const USER: &str = "user";
/// `ChatMessage::role` of an answer.
pub const ASSISTANT: &str = "assistant";

/// One message of a chart's chat. Messages are in `id` order.
#[derive(Debug, toasty::Model)]
pub struct ChatMessage {
    #[key]
    #[auto]
    pub id: u64,

    #[index]
    pub chart_id: u64,

    /// [`USER`] or [`ASSISTANT`].
    pub role: String,

    /// Plain text for a question, markdown for an answer.
    pub content: String,
}

impl ChatMessage {
    pub fn is_user(&self) -> bool {
        self.role == USER
    }
}

/// All messages of a chart, oldest first.
pub async fn history(cx: &Cx, chart_id: u64) -> Result<Vec<ChatMessage>> {
    Ok(ChatMessage::filter_by_chart_id(chart_id)
        .order_by(ChatMessage::fields().id().asc())
        .exec(&mut db(cx))
        .await?)
}

/// Saves one message. `role` is [`USER`] or [`ASSISTANT`].
pub async fn append(cx: &Cx, chart_id: u64, role: &str, content: &str) -> Result<()> {
    toasty::create!(ChatMessage {
        chart_id: chart_id,
        role: role,
        content: content,
    })
    .exec(&mut db(cx))
    .await?;
    Ok(())
}
