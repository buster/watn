use std::collections::HashMap;

use crate::error::Error;
use crate::provider::Provider;

pub struct ProviderRegistry {
    providers: HashMap<String, Box<dyn Provider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, provider: Box<dyn Provider>) {
        self.providers.insert(name, provider);
    }

    pub fn get(&self, name: &str) -> Result<&dyn Provider, Error> {
        self.providers
            .get(name)
            .map(|p| p.as_ref())
            .ok_or_else(|| Error::ProviderNotFound(name.to_string()))
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}