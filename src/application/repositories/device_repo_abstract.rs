use async_trait::async_trait;

use crate::domain::device_entity::DeviceEntity;

use std::error::Error;

#[async_trait(?Send)]
pub trait DeviceRepoAbstract {
    async fn add(&self, home: &str, room: &str, name: &str) -> Result<DeviceEntity, Box<dyn Error>>;
    async fn delete(&self, home: &str, room: &str, name: &str) -> Result<DeviceEntity, Box<dyn Error>>;
    async fn list(&self, home: &str, room: &str) -> Result<Vec<DeviceEntity>, Box<dyn Error>>;
}
