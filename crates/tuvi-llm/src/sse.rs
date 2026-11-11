//! A small parser for server-sent events, enough for chat-completion streams.

/// Collects bytes and returns the `data:` payload of each complete event.
#[derive(Debug, Default)]
pub(crate) struct SseParser {
    buffer: Vec<u8>,
}

impl SseParser {
    /// Adds a chunk and returns the payloads of the events that are now
    /// complete. An event ends at a blank line. Its `data:` lines are joined
    /// with `\n`. Comments (`:` lines) and other fields are skipped.
    pub(crate) fn push(&mut self, chunk: &[u8]) -> Vec<String> {
        self.buffer.extend_from_slice(chunk);
        let mut payloads = Vec::new();
        while let Some((end, separator)) = find_event_end(&self.buffer) {
            let event = String::from_utf8_lossy(&self.buffer[..end]).into_owned();
            self.buffer.drain(..end + separator);
            let data = event
                .lines()
                .filter_map(|line| line.strip_prefix("data:"))
                .map(|data| data.strip_prefix(' ').unwrap_or(data))
                .collect::<Vec<_>>();
            if !data.is_empty() {
                payloads.push(data.join("\n"));
            }
        }
        payloads
    }
}

/// The end of the first event and the length of its blank-line separator.
fn find_event_end(buffer: &[u8]) -> Option<(usize, usize)> {
    let lf = buffer.windows(2).position(|w| w == b"\n\n").map(|i| (i, 2));
    let crlf = buffer
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .map(|i| (i, 4));
    match (lf, crlf) {
        (Some(a), Some(b)) => Some(if a.0 <= b.0 { a } else { b }),
        (a, b) => a.or(b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_split_across_chunks() {
        let mut parser = SseParser::default();
        assert!(parser.push(b"data: {\"a\"").is_empty());
        assert_eq!(
            parser.push(b":1}\n\ndata: [DONE]\n\n"),
            ["{\"a\":1}", "[DONE]"]
        );
    }

    #[test]
    fn crlf_comments_and_multiline_data() {
        let mut parser = SseParser::default();
        let payloads =
            parser.push(b": keep-alive\r\n\r\nevent: x\r\ndata: one\r\ndata: two\r\n\r\n");
        assert_eq!(payloads, ["one\ntwo"]);
    }
}
