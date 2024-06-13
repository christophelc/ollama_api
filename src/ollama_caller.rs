use reqwest::Client;

pub async fn call_ollama() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({ "model": "codestral", "prompt": "Write a hello world program in Rust." }))
        .send()
        .await?;
    println!("Response: {:?}", res.text().await?);
    Ok(())
}
