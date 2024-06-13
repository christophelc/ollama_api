mod ollama_caller;

use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
    let rt = Runtime::new().unwrap();
    match rt.block_on(ollama_caller::call_ollama()) {
        Ok(_) => println!("Ollama called successfully"),
        Err(e) => eprintln!("Error calling Ollama: {}", e),
    }
}
