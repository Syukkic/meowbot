#![allow(non_snake_case)]

use dotenv;
use reqwest::header::HeaderValue;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Content {
    i: String,
    visibility: String,
    text: String,
    fileIds: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().unwrap();
    let api_key = std::env::var("ACCESS_TOKEN").expect("Can't find Misskey TOKEN");
    let instance_url = std::env::var("INSTANCE_URL").expect("Can't find instance URL");
    let endpoint = format!("{}/api/notes/create", instance_url);
    let visibility = "public".to_string();

    let post_data = Content {
        i: api_key,
        visibility: visibility,
        text: "Test Pics x2".to_string(),
        fileIds: vec!["9j11sg9h30".to_string(), "9j11sg9o31".to_string()],
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&endpoint)
        .json(&post_data)
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/116.0",
        )
        .header("Content-Type", HeaderValue::from_static("application/json"))
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let post_info: serde_json::Value = response.json().await.unwrap();
        println!("Post created: {:?}", post_info);
    } else {
        println!("Error: {:?}", response.text().await.unwrap())
    }

    Ok(())
}
