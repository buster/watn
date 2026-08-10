use crate::config::types::ModelPricing;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct ModelEntry {
    pub id: String,
    pub name: Option<String>,
    pub context_length: Option<u64>,
    pub pricing: Option<ModelPricing>,
    pub supported_features: Vec<String>,
    pub reasoning: Option<ModelReasoning>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelReasoning {
    pub default_effort: Option<String>,
    pub default_enabled: bool,
    pub mandatory: bool,
    pub supported_efforts: Vec<String>,
}

pub fn models_url(endpoint: &str) -> String {
    let endpoint = crate::provider::transport::endpoint(endpoint);
    format!("{}/models", endpoint.trim_end_matches('/'))
}

pub fn models_search_url(endpoint: &str, query: &str) -> String {
    let endpoint = crate::provider::transport::endpoint(endpoint);
    format!("{}/models?search={}", endpoint.trim_end_matches('/'), query)
}

pub fn models_page_url(endpoint: &str, page: u32, limit: u32) -> String {
    let endpoint = crate::provider::transport::endpoint(endpoint);
    format!(
        "{}/models?page={}&limit={}",
        endpoint.trim_end_matches('/'),
        page,
        limit
    )
}

pub fn fetch_models(endpoint: &str, api_key: Option<&str>) -> Result<Vec<ModelEntry>, Error> {
    let url = models_url(endpoint);

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let mut req = client.get(&url);
    if let Some(key) = api_key {
        req = req.header("Authorization", format!("Bearer {}", key));
    }

    let response = req.send().map_err(|e| {
        if e.is_timeout() || e.is_connect() {
            Error::NetworkError(e.to_string())
        } else if let Some(status) = e.status() {
            Error::ApiError {
                status: status.as_u16(),
                message: e.to_string(),
            }
        } else {
            Error::NetworkError(e.to_string())
        }
    })?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().unwrap_or_default();
        return Err(Error::ApiError {
            status: status.as_u16(),
            message: body,
        });
    }

    let body: serde_json::Value = response.json().map_err(|e| Error::ApiError {
        status: 0,
        message: format!("failed to parse response: {}", e),
    })?;

    let data = body["data"].as_array().ok_or_else(|| Error::ApiError {
        status: 0,
        message: "response missing 'data' array".to_string(),
    })?;

    let models: Vec<ModelEntry> = data
        .iter()
        .map(|item| {
            let id = item["id"].as_str().unwrap_or("").to_string();

            let name = item["name"].as_str().map(|s| s.to_string());

            let context_length = item["context_length"]
                .as_u64()
                .or_else(|| item["context_length"].as_str().and_then(|s| s.parse().ok()));

            let pricing = item["pricing"].as_object().map(|p| {
                let prompt = parse_pricing_value(p.get("prompt"));
                let completion = parse_pricing_value(p.get("completion"));
                ModelPricing {
                    input: prompt,
                    output: completion,
                }
            });

            let supported_features: Vec<String> = item["supported_features"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();

            ModelEntry {
                id,
                name,
                context_length,
                pricing,
                supported_features,
                reasoning: parse_reasoning(item.get("reasoning")),
            }
        })
        .collect();

    Ok(models)
}

/// Per-word, order-independent match against a model id: split the query on
/// whitespace and require every word to be contained (case-insensitive)
/// anywhere in the id, in any order. "dee flash" matches "DeepSeek V4 Flash"
/// because each word is matched separately anywhere in the identifier.
pub fn word_matches(id: &str, query: &str) -> bool {
    let id_lower = id.to_lowercase();
    query
        .to_lowercase()
        .split_whitespace()
        .all(|word| id_lower.contains(word))
}

pub fn search_models(
    endpoint: &str,
    query: &str,
    api_key: Option<&str>,
) -> Result<Vec<ModelEntry>, Error> {
    let url = models_search_url(endpoint, query);

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let mut req = client.get(&url);
    if let Some(key) = api_key {
        req = req.header("Authorization", format!("Bearer {}", key));
    }

    let response = req.send().map_err(|e| {
        if e.is_timeout() || e.is_connect() {
            Error::NetworkError(e.to_string())
        } else if let Some(status) = e.status() {
            Error::ApiError {
                status: status.as_u16(),
                message: e.to_string(),
            }
        } else {
            Error::NetworkError(e.to_string())
        }
    })?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().unwrap_or_default();
        return Err(Error::ApiError {
            status: status.as_u16(),
            message: body,
        });
    }

    let body: serde_json::Value = response.json().map_err(|e| Error::ApiError {
        status: 0,
        message: format!("failed to parse response: {}", e),
    })?;

    let data = body["data"].as_array().ok_or_else(|| Error::ApiError {
        status: 0,
        message: "response missing 'data' array".to_string(),
    })?;

    let models: Vec<ModelEntry> = parse_model_data(data);

    let filtered: Vec<ModelEntry> = models
        .into_iter()
        .filter(|m| word_matches(&m.id, query))
        .collect();

    Ok(filtered)
}

pub fn fetch_models_page(
    endpoint: &str,
    page: u32,
    limit: u32,
    api_key: Option<&str>,
) -> Result<Vec<ModelEntry>, Error> {
    let url = models_page_url(endpoint, page, limit);

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let mut req = client.get(&url);
    if let Some(key) = api_key {
        req = req.header("Authorization", format!("Bearer {}", key));
    }

    let response = req.send().map_err(|e| {
        if e.is_timeout() || e.is_connect() {
            Error::NetworkError(e.to_string())
        } else if let Some(status) = e.status() {
            Error::ApiError {
                status: status.as_u16(),
                message: e.to_string(),
            }
        } else {
            Error::NetworkError(e.to_string())
        }
    })?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().unwrap_or_default();
        return Err(Error::ApiError {
            status: status.as_u16(),
            message: body,
        });
    }

    let body: serde_json::Value = response.json().map_err(|e| Error::ApiError {
        status: 0,
        message: format!("failed to parse response: {}", e),
    })?;

    let data = body["data"].as_array().ok_or_else(|| Error::ApiError {
        status: 0,
        message: "response missing 'data' array".to_string(),
    })?;

    let models: Vec<ModelEntry> = parse_model_data(data);
    Ok(models)
}

fn parse_model_data(data: &[serde_json::Value]) -> Vec<ModelEntry> {
    data.iter()
        .map(|item| {
            let id = item["id"].as_str().unwrap_or("").to_string();
            let name = item["name"].as_str().map(|s| s.to_string());
            let context_length = item["context_length"]
                .as_u64()
                .or_else(|| item["context_length"].as_str().and_then(|s| s.parse().ok()));
            let pricing = item["pricing"].as_object().map(|p| ModelPricing {
                input: parse_pricing_value(p.get("prompt")),
                output: parse_pricing_value(p.get("completion")),
            });
            let supported_features: Vec<String> = item["supported_features"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();

            ModelEntry {
                id,
                name,
                context_length,
                pricing,
                supported_features,
                reasoning: parse_reasoning(item.get("reasoning")),
            }
        })
        .collect()
}

fn parse_pricing_value(val: Option<&serde_json::Value>) -> f64 {
    match val {
        Some(v) => v
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .or_else(|| v.as_f64())
            .unwrap_or(0.0),
        None => 0.0,
    }
}

fn parse_reasoning(value: Option<&serde_json::Value>) -> Option<ModelReasoning> {
    let object = value?.as_object()?;
    let supported_efforts = object
        .get("supported_efforts")
        .and_then(serde_json::Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(|value| value.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    Some(ModelReasoning {
        default_effort: object
            .get("default_effort")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string),
        default_enabled: object
            .get("default_enabled")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        mandatory: object
            .get("mandatory")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        supported_efforts,
    })
}
