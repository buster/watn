use crate::config::types::ModelPricing;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct ModelEntry {
    pub id: String,
    pub name: Option<String>,
    pub context_length: Option<u64>,
    pub pricing: Option<ModelPricing>,
    pub supported_features: Vec<String>,
}

pub fn fetch_models(endpoint: &str, api_key: Option<&str>) -> Result<Vec<ModelEntry>, Error> {
    let url = format!("{}/models", endpoint.trim_end_matches('/'));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let mut req = client.get(&url);
    if let Some(key) = api_key {
        req = req.header("Authorization", format!("Bearer {}", key));
    }

    let response = req
        .send()
        .map_err(|e| {
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

    let body: serde_json::Value = response
        .json()
        .map_err(|e| Error::ApiError {
            status: 0,
            message: format!("failed to parse response: {}", e),
        })?;

    let data = body["data"]
        .as_array()
        .ok_or_else(|| Error::ApiError {
            status: 0,
            message: "response missing 'data' array".to_string(),
        })?;

    let models: Vec<ModelEntry> = data
        .iter()
        .map(|item| {
            let id = item["id"].as_str().unwrap_or("").to_string();

            let name = item["name"].as_str().map(|s| s.to_string());

            let context_length = item["context_length"].as_u64().or_else(|| {
                item["context_length"].as_str().and_then(|s| s.parse().ok())
            });

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
            }
        })
        .collect();

    Ok(models)
}

pub fn search_models(
    endpoint: &str,
    query: &str,
    api_key: Option<&str>,
) -> Result<Vec<ModelEntry>, Error> {
    let base = endpoint.trim_end_matches('/');
    let url = format!("{}/models?search={}", base, query);

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let mut req = client.get(&url);
    if let Some(key) = api_key {
        req = req.header("Authorization", format!("Bearer {}", key));
    }

    let response = req
        .send()
        .map_err(|e| {
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

    let body: serde_json::Value = response
        .json()
        .map_err(|e| Error::ApiError {
            status: 0,
            message: format!("failed to parse response: {}", e),
        })?;

    let data = body["data"]
        .as_array()
        .ok_or_else(|| Error::ApiError {
            status: 0,
            message: "response missing 'data' array".to_string(),
        })?;

    let models: Vec<ModelEntry> = parse_model_data(data);
    let query_lower = query.to_lowercase();

    let filtered: Vec<ModelEntry> = models
        .into_iter()
        .filter(|m| m.id.to_lowercase().contains(&query_lower))
        .collect();

    Ok(filtered)
}

pub fn fetch_models_page(
    endpoint: &str,
    page: u32,
    limit: u32,
    api_key: Option<&str>,
) -> Result<Vec<ModelEntry>, Error> {
    let base = endpoint.trim_end_matches('/');
    let url = format!("{}/models?page={}&limit={}", base, page, limit);

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let mut req = client.get(&url);
    if let Some(key) = api_key {
        req = req.header("Authorization", format!("Bearer {}", key));
    }

    let response = req
        .send()
        .map_err(|e| {
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

    let body: serde_json::Value = response
        .json()
        .map_err(|e| Error::ApiError {
            status: 0,
            message: format!("failed to parse response: {}", e),
        })?;

    let data = body["data"]
        .as_array()
        .ok_or_else(|| Error::ApiError {
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
            let context_length = item["context_length"].as_u64().or_else(|| {
                item["context_length"].as_str().and_then(|s| s.parse().ok())
            });
            let pricing = item["pricing"].as_object().map(|p| {
                ModelPricing {
                    input: parse_pricing_value(p.get("prompt")),
                    output: parse_pricing_value(p.get("completion")),
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
            }
        })
        .collect()
}

fn parse_pricing_value(val: Option<&serde_json::Value>) -> f64 {
    match val {
        Some(v) => v.as_str().and_then(|s| s.parse::<f64>().ok())
            .or_else(|| v.as_f64())
            .unwrap_or(0.0),
        None => 0.0,
    }
}
