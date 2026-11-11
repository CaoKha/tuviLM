//! Markdown from the model to HTML that is safe to put in the page.

use pulldown_cmark::{Event, Options, Parser, Tag, html};

/// Renders markdown to HTML. Raw HTML in the input is shown as text, images
/// are dropped (their alt text stays), and links keep only `http`, `https`
/// and `mailto` targets.
pub fn to_html(markdown: &str) -> String {
    let options =
        Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES | Options::ENABLE_TASKLISTS;
    let mut in_image = 0usize;
    let events = Parser::new_ext(markdown, options).filter_map(|event| match event {
        Event::Html(text) | Event::InlineHtml(text) => Some(Event::Text(text)),
        Event::Start(Tag::Image { .. }) => {
            in_image += 1;
            None
        }
        Event::End(pulldown_cmark::TagEnd::Image) => {
            in_image = in_image.saturating_sub(1);
            None
        }
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            let safe = ["http://", "https://", "mailto:"]
                .iter()
                .any(|scheme| dest_url.to_ascii_lowercase().starts_with(scheme));
            let dest_url = if safe { dest_url } else { "#".into() };
            Some(Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }))
        }
        other => Some(other),
    });
    let mut out = String::with_capacity(markdown.len() * 3 / 2);
    html::push_html(&mut out, events);
    out
}

#[cfg(test)]
mod tests {
    use super::to_html;

    #[test]
    fn formats_markdown() {
        assert_eq!(
            to_html("**Mệnh** có _Thiên Cơ_"),
            "<p><strong>Mệnh</strong> có <em>Thiên Cơ</em></p>\n"
        );
    }

    #[test]
    fn escapes_raw_html() {
        let html = to_html("<script>alert(1)</script> <b onclick=x>hi</b>");
        assert!(!html.contains("<script"));
        assert!(!html.contains("<b "));
        assert!(html.contains("&lt;script&gt;"));
    }

    #[test]
    fn blocks_unsafe_links_and_images() {
        let html = to_html("[a](javascript:alert(1)) [b](https://x.vn) ![img](https://x.vn/a.png)");
        assert!(!html.contains("javascript:"));
        assert!(html.contains("href=\"https://x.vn\""));
        assert!(!html.contains("<img"));
    }
}
