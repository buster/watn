pub mod openai_compat;
pub mod registry;
pub mod setup;
pub mod transport;

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct RequestOptions {
    pub model: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub reasoning_effort: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
}

#[derive(Debug, Clone)]
pub struct StreamingResponse {
    pub final_usage: Option<TokenUsage>,
    pub model: String,
    pub full_content: String,
    pub elapsed_secs: f64,
    pub reasoning_content: Option<String>,
}

pub trait Provider: Send + Sync {
    fn chat_completions_streaming(
        &self,
        messages: &[Message],
        options: &RequestOptions,
    ) -> Result<StreamingResponse, Error>;
}
