pub mod openai_compat;

use std::time::Duration;

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct RequestOptions {
    pub model: String,
    pub streaming: bool,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct StreamChunk {
    pub content: Option<String>,
    pub finish_reason: Option<String>,
    pub usage: Option<TokenUsage>,
}

#[derive(Debug, Clone)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone)]
pub struct StreamingResponse {
    pub chunks: Vec<StreamChunk>,
    pub final_usage: Option<TokenUsage>,
    pub model: String,
    pub full_content: String,
}

#[derive(Debug, Clone)]
pub struct CompleteResponse {
    pub content: String,
    pub model: String,
    pub usage: TokenUsage,
}

pub trait Provider: Send + Sync {
    fn chat_completions_streaming(
        &self,
        messages: &[Message],
        options: &RequestOptions,
    ) -> Result<StreamingResponse, Error>;

    fn chat_completions_blocking(
        &self,
        messages: &[Message],
        options: &RequestOptions,
    ) -> Result<CompleteResponse, Error>;
}
