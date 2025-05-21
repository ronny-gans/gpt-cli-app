use serde::{Serialize,Deserialize};
use serde_json;



// build based on the response from API look at the documentation from each API

#[derive(Serialize,Deserialize)]
pub struct ApiResponse {
    pub choices:Vec<Choice>,
    pub created:i64,
    pub id:String,
    pub model: String,
    pub object: String,
    pub service_tier: Option<String>,
    pub system_fingerprint: Option<String>,
    pub usage:Usage
}


#[derive(Serialize,Deserialize,Debug)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
    pub index:i32,
    pub logprobs: Option<serde_json::Value>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Message {
    pub annotations: Option<serde_json::Value>,
    pub content:String,
    pub refusal: Option<serde_json::Value>,
    pub role:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Usage {
    pub completion_tokens:i32,
    pub completion_tokens_details: CompletionToken,
    pub prompt_tokens:i32,
    pub prompt_tokens_details: CompletionToken,
    pub total_tokens:i32
}


#[derive(Serialize,Deserialize,Debug)]
pub struct CompletionToken {
    pub accepted_prediction_tokens: Option<i32>,
    pub audio_tokens:Option<i32>,
    pub reasoning_tokens:Option<i32>,
    pub rejected_prediction_tokens: Option<i32>,
    #[serde(default)]
    pub cached_tokens: Option<i32>,
}





    