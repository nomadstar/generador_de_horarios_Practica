use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub async fn send_get_request(url: &str) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();
    let resp = client.get(url).send().await?.json::<Value>().await?;
    Ok(resp)
}

pub async fn send_post_request(url: &str, body: &Value) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();
    let resp = client.post(url)
        .json(body)
        .send()
        .await?
        .json::<Value>()
        .await?;
    Ok(resp)
}

// Ejemplo de uso:
// #[tokio::main]
// async fn main() {
//     let url = "https://jsonplaceholder.typicode.com/posts/1";
//     match send_get_request(url).await {
//         Ok(json) => println!("{:#?}", json),
//         Err(e) => eprintln!("Error: {}", e),
//     }
// }