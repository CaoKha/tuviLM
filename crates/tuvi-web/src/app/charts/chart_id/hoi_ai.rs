//! `/charts/{chart_id}/hoi-ai`: the AI chat about one chart.
//!
//! A question is a form POST: it saves the question and redirects back here.
//! When the last message is a question, this page streams the answer into a
//! `live!` region over the same HTTP response, then saves it.

use std::time::{Duration, Instant};

use futures_util::StreamExt;
use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{
        content::Form,
        error::{SeeOther, see_other},
        href, page, route,
    },
    view::{Unescaped, View, emit, live, view},
};
use tuvi_core::Chart;
use tuvi_llm::{Message, Session};

use super::chart_id;
use crate::features::{
    charts::{accent_style, load},
    chat::{ASSISTANT, ChatMessage, USER, append, chat_service, history, system_prompt, to_html},
};

/// Max characters of one question.
const QUESTION_MAX_CHARS: usize = 2000;

/// Messages sent to the model besides the system prompt: the newest ones.
const HISTORY_LIMIT: usize = 20;

/// The streamed answer is drawn again at most this often.
const EMIT_EVERY: Duration = Duration::from_millis(80);

#[page]
pub async fn show(cx: &Cx) -> Result<impl View> {
    let id = *chart_id(cx)?;
    let (saved, chart) = load(cx, id).await?;
    let messages = history(cx, id).await?;
    let waiting = messages.last().is_some_and(ChatMessage::is_user);
    let prompt = if waiting {
        model_prompt(&saved.name, &chart, &messages)
    } else {
        Vec::new()
    };

    Ok(view! {
        <main
            class="mx-auto flex max-w-2xl flex-col px-4 pb-8"
            style=(accent_style(&chart))
        >
            <a
                href=(href!(super::show, chart_id: id))
                class="text-[10px] font-bold tracking-[0.26em] text-accent uppercase"
            >
                "← Lá số của "
                (saved.name.as_str())
            </a>
            <h1 class="mt-1 text-3xl font-semibold">"Hỏi Thiên Hạc"</h1>
            <section aria-live="polite" class="mt-6 flex flex-col gap-3">
                if messages.is_empty() {
                    ai_bubble(
                        html: format!(
                            "<p>Chào bạn ✦ Mình đã đọc lá số của {} rồi. Bạn muốn hỏi về cung nào?</p>",
                            escape(&saved.name),
                        )
                    )
                }
                for message in &messages {
                    if message.is_user() {
                        user_bubble(text: message.content.as_str())
                    } else {
                        ai_bubble(html: to_html(&message.content))
                    }
                }
                if waiting {
                    answer(chart_id: id, prompt: &prompt)
                }
            </section>
            <form
                id="cuoi"
                method="post"
                action=(href!(ask, chart_id: id))
                class="sticky bottom-4 mt-6 flex items-center gap-2.5 rounded-full border border-[rgba(243,239,250,0.18)] bg-[rgba(48,29,80,0.97)] py-2 pr-2 pl-5 shadow-[0_8px_24px_rgba(8,4,20,0.5)]"
            >
                <input
                    name="question"
                    required=""
                    maxlength=(QUESTION_MAX_CHARS)
                    autocomplete="off"
                    autofocus=(!waiting)
                    placeholder="Hỏi về một cung, một sao, một năm…"
                    aria-label="Câu hỏi"
                    class="min-w-0 flex-1 bg-transparent text-[15px] text-ink outline-none placeholder:text-muted"
                >
                <button
                    type="submit"
                    aria-label="Gửi"
                    class="pill-primary grid size-10.5 shrink-0 cursor-pointer place-items-center rounded-full text-lg"
                >
                    "↑"
                </button>
            </form>
        </main>
    })
}

#[derive(Deserialize)]
pub struct Question {
    question: String,
}

/// Saves a question, then goes back to the chat, which streams the answer.
#[route(POST)]
pub async fn ask(cx: &Cx, Form(form): Form<Question>) -> Result<SeeOther> {
    let id = *chart_id(cx)?;
    let question = form.question.trim();
    if !question.is_empty() && question.chars().count() <= QUESTION_MAX_CHARS {
        // `load` turns an unknown chart into a 404 before anything is saved.
        load(cx, id).await?;
        append(cx, id, USER, question).await?;
    }
    let url = href!(show, chart_id: id).resolve(cx);
    Ok(see_other(format!("{url}#cuoi")))
}

/// What the model gets: the system prompt, then the newest messages.
fn model_prompt(name: &str, chart: &Chart, messages: &[ChatMessage]) -> Vec<Message> {
    let skip = messages.len().saturating_sub(HISTORY_LIMIT);
    std::iter::once(Message::system(system_prompt(name, chart)))
        .chain(messages[skip..].iter().map(|message| {
            if message.is_user() {
                Message::user(message.content.as_str())
            } else {
                Message::assistant(message.content.as_str())
            }
        }))
        .collect()
}

/// The answer to the last question, streamed as it arrives, then saved.
#[topcoat::view::component]
async fn answer(cx: &Cx, chart_id: u64, prompt: &[Message]) -> Result<impl View> {
    Ok(view! {
        (live! {
            emit! {
                ai_bubble(
                    html: "<p class=\"animate-pulse text-muted\">Thiên Hạc đang xem lá số…</p>".to_owned(

                    )
                )
            }?;

            let service = chat_service(cx);
            let client = match service.client() {
                Ok(client) => client,
                Err(reason) => return emit! {
                    error_bubble(
                        text: format!("Chưa bật trò chuyện AI: {reason}.")
                    )
                },
            };
            let Some(_guard) = service.begin(chart_id) else {
                return emit! {
                    ai_bubble(
                        html: "<p class=\"text-muted\">Thiên Hạc đang trả lời câu này ở một cửa sổ khác. Hãy tải lại trang sau ít phút.</p>".to_owned(

                        )
                    )
                };
            };

            // One OpenCode Go session per chart chat.
            let session = Session::from_id(format!("tuvilm-chart-{chart_id}"));
            let mut stream = match client.stream(&session, prompt).await {
                Ok(stream) => stream,
                Err(error) => {
                    eprintln!("chat {chart_id}: {error}");
                    return emit! {
                        error_bubble(
                            text: format!(
                                "Thiên Hạc chưa trả lời được: {error}",
                            )
                        )
                    };
                }
            };

            let mut text = String::new();
            let mut last_emit = Instant::now();
            while let Some(piece) = stream.next().await {
                match piece {
                    Ok(piece) => text.push_str(&piece),
                    Err(error) => {
                        eprintln!("chat {chart_id}: {error}");
                        return emit! {
                            error_bubble(
                                text: format!("Câu trả lời bị ngắt: {error}")
                            )
                        };
                    }
                }
                if last_emit.elapsed() >= EMIT_EVERY {
                    emit! { ai_bubble(html: to_html(&text)) }?;
                    last_emit = Instant::now();
                }
            }
            if text.trim().is_empty() {
                return emit! {
                    error_bubble(
                        text: "Thiên Hạc không trả lời. Hãy thử lại.".to_owned(

                        )
                    )
                };
            }

            append(cx, chart_id, ASSISTANT, &text).await?;
            emit! { ai_bubble(html: to_html(&text)) }
        })
    })
}

/// A message from Thiên Hạc. `html` must be safe: from [`to_html`] or escaped.
#[topcoat::view::component]
async fn ai_bubble(cx: &Cx, html: String) -> Result<impl View> {
    Ok(view! {
        <article
            class="glass max-w-[90%] self-start rounded-[20px] rounded-bl-md px-4 py-3.5 text-[14.5px] leading-relaxed sm:max-w-[80%] [&_ol]:list-decimal [&_ol]:pl-5 [&_p+*]:mt-2 [&_strong]:text-accent [&_ul]:list-disc [&_ul]:pl-5 [&_h3]:mt-3 [&_h3]:text-base [&_h2]:mt-3 [&_h2]:text-lg"
        >
            <p class="mb-1 text-[11px] font-semibold tracking-[0.16em] text-accent">
                "THIÊN HẠC"
            </p>
            (Unescaped::new_unchecked(html))
        </article>
    })
}

/// A question from the user.
#[topcoat::view::component]
async fn user_bubble(cx: &Cx, text: &str) -> Result<impl View> {
    Ok(view! {
        <p
            class="pill-primary max-w-[85%] self-end rounded-[20px] rounded-br-md px-4 py-3 text-[14.5px] leading-normal font-medium whitespace-pre-wrap"
        >
            (text)
        </p>
    })
}

/// An answer that failed. The question stays, so a reload tries again.
#[topcoat::view::component]
async fn error_bubble(cx: &Cx, text: String) -> Result<impl View> {
    Ok(view! {
        <div
            role="alert"
            class="glass max-w-[90%] self-start rounded-[20px] rounded-bl-md border-fire/50 px-4 py-3.5 text-sm"
        >
            <p class="text-fire">(text)</p>
            <a
                href=""
                class="mt-2 inline-block text-xs font-semibold text-accent underline"
            >
                "Thử lại"
            </a>
        </div>
    })
}

/// Escapes text for the few places that build HTML by hand.
fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
