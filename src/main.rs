mod ollama_caller;

use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Deserialize)]
struct ChatInput {
    message: String,
}

#[derive(Serialize)]
struct ChatResponse {
    response: String,
}

async fn handle_chat(
    input: ChatInput,
) -> Result<
    impl warp::Reply,
    warp::Rejection,
> {
    let messages =
        vec![ollama_caller::Message {
            role: "user".to_string(),
            content: input.message,
        }];

    match ollama_caller::call_ollama(
        messages,
    )
    .await
    {
        Ok(response_message) => {
            let response =
                ChatResponse {
                    response:
                        response_message,
                };
            Ok(warp::reply::json(
                &response,
            ))
        },
        Err(e) => {
            Err(warp::reject::custom(e))
        },
    }
}

#[tokio::main]
async fn main() {
    // GET / -> index.html
    let index_route = warp::path::end()
        .map(|| {
            warp::reply::html(
                INDEX_HTML,
            )
        });

    // POST /chat -> handle chat messages
    let chat_route = warp::path("chat")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_chat);

    // Combine routes
    let routes =
        index_route.or(chat_route);

    // Start the server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

static INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Chatbot</title>
</head>
<body>
    <h1>Chatbot</h1>
    <form id="chat-form">
        <input type="text" id="message" placeholder="Type your message" required>
        <button type="submit">Send</button>
    </form>
    <div id="chat-log"></div>
    <script>
        const form = document.getElementById('chat-form');
        form.addEventListener('submit', async (e) => {
            e.preventDefault();
            const message = document.getElementById('message').value;
            const response = await fetch('/chat', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ message }),
            });
            const result = await response.json();
            const chatLog = document.getElementById('chat-log');
            chatLog.innerHTML += `<p><strong>You:</strong> ${message}</p>`;
            chatLog.innerHTML += `<p><strong>Bot:</strong> ${result.response}</p>`;
            form.reset();
        });
    </script>
</body>
</html>
"#;
