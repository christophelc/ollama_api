use std::fmt;
use serde::{Deserialize, Serialize};
use warp::reject::Reject;

#[derive(Debug)]
pub struct CustomError {
    message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Reject for CustomError {}


#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
    stream: bool
}

#[derive(Deserialize)]
struct ApiResponse {
    model: String,
    created_at: String,
    message: Message,
    done: bool,
    done_reason: String,
}

pub async fn call_ollama(messages: Vec<Message>) -> Result<String, CustomError> {
    let client = reqwest::Client::new();
    let body = RequestBody {
        model: "codestral".to_string(),
        messages,
        stream: false, // Set this to true or false as needed
    };

    // Serialize the body to JSON and print it
    let body_json = serde_json::to_string(&body).map_err(|e| CustomError {
        message: e.to_string(),
    })?;
    println!("Request body: {}", body_json);

    let res = client
        .post("http://localhost:11434/api/chat")
        .json(&body)
        .send()
        .await
        .map_err(|e| CustomError { message: e.to_string() })?;

    let status = res.status();
    let res_text = res.text().await.map_err(|e| CustomError {
        message: e.to_string(),
    })?;
    println!("Response body: {}", res_text);

    if status.is_success() {
        // Deserialize the response JSON from the text
        let api_response: ApiResponse = serde_json::from_str(&res_text).map_err(|e| CustomError {
            message: e.to_string(),
        })?;
        Ok(api_response.message.content)        
    } else {
        Err(CustomError {
            message: format!("Request failed with status: {}", status),
        })
    }
}
