use async_trait::async_trait;

use crate::domain::room_entity::RoomEntity;

use std::error::Error;

#[async_trait(?Send)]
pub trait RoomRepoAbstract {
    async fn add(&self, home: &str, name: &str) -> Result<RoomEntity, Box<dyn Error>>;
    async fn delete(&self, home: &str, name: &str) -> Result<RoomEntity, Box<dyn Error>>;
    async fn list(&self, home: &str) -> Result<Vec<RoomEntity>, Box<dyn Error>>;
}
