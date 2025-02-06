use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceEntity {
    pub id: Uuid,
    pub name: String,
    pub data: Value,
}

impl DeviceEntity {
    pub fn new(id: Uuid, name: String, data: Value) -> Self {
        Self { id, name, data }
    }
}
