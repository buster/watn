use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

use crate::error::Error;
use crate::provider::{Message, Provider, RequestOptions, StreamEvent, StreamingResponse, TokenUsage};

pub struct OpenAICompatibleProvider {
    pub endpoint: String,
    pub api_key: String,
    client: reqwest::blocking::Client,
}

pub fn chat_completions_url(endpoint: &str) -> String {
    let endpoint = crate::provider::transport::endpoint(endpoint);
    format!("{}/chat/completions", endpoint.trim_end_matches('/'))
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
        sink: &mut dyn FnMut(StreamEvent) -> Result<(), Error>,
    ) -> Result<StreamingResponse, Error> {
        let url = chat_completions_url(&self.endpoint);

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

        let mut reader = BufReader::new(response);
        let mut line = String::new();
        let mut first_event_at = None;
        let mut completed = false;

        loop {
            line.clear();
            let bytes_read = reader
                .read_line(&mut line)
                .map_err(|e| Error::NetworkError(e.to_string()))?;
            if bytes_read == 0 {
                break;
            }

            let line = line.trim_end_matches(['\r', '\n']);
            let Some(data) = line.strip_prefix("data:") else {
                continue;
            };
            let data = data.strip_prefix(' ').unwrap_or(data).trim();
            if data == "[DONE]" {
                completed = true;
                break;
            }

            first_event_at.get_or_insert_with(Instant::now);

            let Ok(chunk) = serde_json::from_str::<serde_json::Value>(data) else {
                continue;
            };

            if let Some(model) = chunk["model"].as_str() {
                response_model = model.to_string();
            }

            if let Some(usage) = chunk["usage"].as_object() {
                final_usage = Some(TokenUsage {
                    prompt_tokens: usage["prompt_tokens"].as_u64().unwrap_or(0) as u32,
                    completion_tokens: usage["completion_tokens"].as_u64().unwrap_or(0) as u32,
                });
            }

            if let Some(choices) = chunk["choices"].as_array() {
                for choice in choices {
                    let delta = &choice["delta"];
                    if let Some(content) = delta["content"].as_str() {
                        if !content.is_empty() {
                            full_content.push_str(content);
                            sink(StreamEvent::Content(content.to_string()))?;
                        }
                    }

                    if let Some(reasoning) = delta["reasoning"]
                        .as_str()
                        .or_else(|| delta["reasoning_content"].as_str())
                    {
                        reasoning_content.push_str(reasoning);
                    }
                }
            }
        }

        if !completed {
            return Err(Error::NetworkError(
                "stream ended before [DONE]".to_string(),
            ));
        }

        let elapsed_secs = first_event_at
            .map(|started| started.elapsed().as_secs_f64())
            .unwrap_or(0.0);

        Ok(StreamingResponse {
            final_usage,
            model: response_model,
            full_content,
            elapsed_secs,
            reasoning_content: if reasoning_content.is_empty() {
                None
            } else {
                Some(reasoning_content)
            },
        })
    }
}
