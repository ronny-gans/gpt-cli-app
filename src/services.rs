use reqwest::Client;
use serde_json;
use std::env;
use dotenv::dotenv;
use crate::models::ApiResponse;



pub async fn make_connection(prompt:String)-> Result <ApiResponse,Box <dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("invalid api key");
    let url="https://api.openai.com/v1/chat/completions";
    let client =Client::new();

    let body = serde_json::json! ({
        "model":"gpt-4.1",
        "messages": [
            {"role":"user","content":prompt}]
    });
    let res = client
        .post(url)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?;
        let text=res.text().await?;
        let parsed=serde_json::from_str(&text)?;
    
    Ok(parsed)

}
    