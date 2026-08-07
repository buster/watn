pub mod openai_compat;

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct RequestOptions {
    pub model: String,
    #[allow(dead_code)]
    pub streaming: bool,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub reasoning_effort: Option<String>,
    #[allow(dead_code)]
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub struct StreamChunk {
    #[allow(dead_code)]
    pub content: Option<String>,
    #[allow(dead_code)]
    pub finish_reason: Option<String>,
    #[allow(dead_code)]
    pub usage: Option<TokenUsage>,
}

#[derive(Debug, Clone)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    #[allow(dead_code)]
    pub total_tokens: u32,
}

#[derive(Debug, Clone)]
pub struct StreamingResponse {
    #[allow(dead_code)]
    pub chunks: Vec<StreamChunk>,
    pub final_usage: Option<TokenUsage>,
    pub model: String,
    pub full_content: String,
    pub elapsed_secs: f64,
    pub reasoning_content: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
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

    #[allow(dead_code)]
    fn chat_completions_blocking(
        &self,
        messages: &[Message],
        options: &RequestOptions,
    ) -> Result<CompleteResponse, Error>;
}
