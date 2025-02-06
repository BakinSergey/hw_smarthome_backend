use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct DevicePresenter {
    pub id: Uuid,
    pub name: String,
    pub data: Value,
}
