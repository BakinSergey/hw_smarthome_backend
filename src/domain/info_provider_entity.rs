use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ProviderInfo {
    pub query: HashMap<String, Vec<String>>,
}
