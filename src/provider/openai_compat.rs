use std::io::Read;
use std::time::{Duration, Instant};

use crate::error::Error;
use crate::provider::{
    Message, Provider, RequestOptions, StreamingResponse,
    TokenUsage,
};

pub struct OpenAICompatibleProvider {
    pub endpoint: String,
    pub api_key: String,
    client: reqwest::blocking::Client,
}

impl OpenAICompatibleProvider {
    pub fn new(endpoint: String, api_key: String) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap();
        Self {
            endpoint,
            api_key,
            client,
        }
    }
}

impl Provider for OpenAICompatibleProvider {
    fn chat_completions_streaming(
        &self,
        messages: &[Message],
        options: &RequestOptions,
    ) -> Result<StreamingResponse, Error> {
        let start = Instant::now();
        let endpoint = crate::provider::transport::endpoint(&self.endpoint);
        let url = format!("{}/chat/completions", endpoint.trim_end_matches('/'));

        let mut body = serde_json::json!({
            "model": options.model,
            "messages": messages.iter().map(|m| serde_json::json!({
                "role": m.role,
                "content": m.content,
            })).collect::<Vec<_>>(),
            "stream": true,
            "temperature": options.temperature.unwrap_or(0.7),
            "max_tokens": options.max_tokens.unwrap_or(1024),
        });

        if let Some(effort) = &options.reasoning_effort {
            body["reasoning_effort"] = serde_json::json!(effort);
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .map_err(|e| {
                if e.is_timeout() || e.is_connect() {
                    Error::NetworkError(e.to_string())
                } else if let Some(status) = e.status() {
                    if status.as_u16() == 401 {
                        Error::AuthError("authentication failed".to_string())
                    } else {
                        Error::ApiError {
                            status: status.as_u16(),
                            message: e.to_string(),
                        }
                    }
                } else {
                    Error::NetworkError(e.to_string())
                }
            })?;

        let status = response.status();
        if !status.is_success() {
            let body_text = response.text().unwrap_or_default();
            return if status.as_u16() == 401 {
                Err(Error::AuthError("authentication failed".to_string()))
            } else {
                Err(Error::ApiError {
                    status: status.as_u16(),
                    message: body_text,
                })
            };
        }

        let mut full_content = String::new();
        let mut reasoning_content = String::new();
        let mut final_usage = None;
        let mut response_model = options.model.clone();

        let mut buf = Vec::new();
        response
            .bytes()
            .map_err(|e| Error::NetworkError(e.to_string()))?
            .as_ref()
            .read_to_end(&mut buf)
            .map_err(|e| Error::NetworkError(e.to_string()))?;

        let text = String::from_utf8_lossy(&buf);
        for line in text.lines() {
            let line = line.trim();
            if !line.starts_with("data: ") {
                continue;
            }
            let data = &line[6..];
            if data == "[DONE]" {
                continue;
            }
            if let Ok(chunk) = serde_json::from_str::<serde_json::Value>(data) {
                if let Some(choices) = chunk["choices"].as_array() {
                    for choice in choices {
                        let delta = &choice["delta"];
                        if let Some(content) = delta["content"].as_str() {
                            full_content.push_str(content);
                        }

                        if let Some(reasoning) = delta["reasoning"].as_str() {
                            reasoning_content.push_str(reasoning);
                        }

                        if let Some(model) = chunk["model"].as_str() {
                            response_model = model.to_string();
                        }

                        if let Some(usage) = chunk["usage"].as_object() {
                            final_usage = Some(TokenUsage {
                                prompt_tokens: usage["prompt_tokens"].as_u64().unwrap_or(0) as u32,
                                completion_tokens: usage["completion_tokens"]
                                    .as_u64()
                                    .unwrap_or(0)
                                    as u32,
                                total_tokens: usage["total_tokens"].as_u64().unwrap_or(0) as u32,
                            });
                        }
                    }
                }
            }
        }

        let elapsed_secs = start.elapsed().as_secs_f64();

        Ok(StreamingResponse {
            chunks: Vec::new(),
            final_usage,
            model: response_model,
            full_content,
            elapsed_secs,
            reasoning_content: if reasoning_content.is_empty() { None } else { Some(reasoning_content) },
        })
    }
}
