//! End-to-end tests against a fake chat-completions server on localhost.

use futures_util::StreamExt;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tuvi_llm::{LlmClient, LlmConfig, Message, Provider, Session};

/// Serves one request with `body` and returns the raw request text.
async fn serve_once(
    body: &'static str,
    content_type: &'static str,
) -> (String, tokio::task::JoinHandle<String>) {
    let Ok(listener) = TcpListener::bind("127.0.0.1:0").await else {
        panic!("bind")
    };
    let Ok(address) = listener.local_addr() else {
        panic!("addr")
    };
    let handle = tokio::spawn(async move {
        let Ok((mut socket, _)) = listener.accept().await else {
            panic!("accept")
        };
        let mut request = Vec::new();
        let mut buffer = [0u8; 4096];
        // Read until the JSON body is complete (it ends with `}`).
        loop {
            let Ok(read) = socket.read(&mut buffer).await else {
                panic!("read")
            };
            request.extend_from_slice(&buffer[..read]);
            if read == 0 || request.ends_with(b"}") {
                break;
            }
        }
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-type: {content_type}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
            body.len()
        );
        let _ = socket.write_all(response.as_bytes()).await;
        String::from_utf8_lossy(&request).into_owned()
    });
    (format!("http://{address}/v1"), handle)
}

fn config(provider: Provider, base_url: String) -> LlmConfig {
    LlmConfig {
        provider,
        api_key: "test-key".to_owned(),
        model: "glm-5.3".to_owned(),
        base_url,
    }
}

#[tokio::test]
async fn opencode_go_stream_sends_session_and_joins_text() {
    let body = "data: {\"choices\":[{\"delta\":{\"role\":\"assistant\"}}]}\n\n\
                data: {\"choices\":[{\"delta\":{\"content\":\"Lá \"}}]}\n\n\
                data: {\"choices\":[{\"delta\":{\"content\":\"số\"}}]}\n\n\
                data: [DONE]\n\n";
    let (base_url, server) = serve_once(body, "text/event-stream").await;
    let Ok(client) = LlmClient::new(config(Provider::OpenCodeGo, base_url)) else {
        panic!()
    };
    let session = Session::from_id("chat-42");

    let Ok(stream) = client.stream(&session, &[Message::user("hi")]).await else {
        panic!("send")
    };
    let pieces: Vec<_> = stream.collect().await;
    let text: String = pieces
        .into_iter()
        .map(|piece| piece.unwrap_or_default())
        .collect();
    assert_eq!(text, "Lá số");

    let Ok(request) = server.await else {
        panic!("server")
    };
    let request = request.to_lowercase();
    assert!(request.starts_with("post /v1/chat/completions "));
    assert!(request.contains("authorization: bearer test-key"));
    assert!(request.contains("x-opencode-session: chat-42"));
    assert!(request.contains("user-agent: tuvilm/"));
    assert!(request.contains("\"stream\":true"));
}

#[tokio::test]
async fn openrouter_complete_has_no_session_header() {
    let body = r#"{"choices":[{"message":{"role":"assistant","content":"Xin chào"}}]}"#;
    let (base_url, server) = serve_once(body, "application/json").await;
    let Ok(client) = LlmClient::new(config(Provider::OpenRouter, base_url)) else {
        panic!()
    };

    let answer = client
        .complete(&Session::new(), &[Message::user("hi")])
        .await;
    assert_eq!(answer.ok().as_deref(), Some("Xin chào"));

    let Ok(request) = server.await else {
        panic!("server")
    };
    assert!(!request.to_lowercase().contains("x-opencode-session"));
}
