use async_trait::async_trait;

use crate::domain::home_entity::HomeEntity;

use std::error::Error;

#[async_trait(?Send)]
pub trait HomeRepoAbstract {
    async fn add(&self, name: &str) -> Result<HomeEntity, Box<dyn Error>>;
    async fn delete(&self, name: &str) -> Result<HomeEntity, Box<dyn Error>>;
    async fn list(&self) -> Result<Vec<HomeEntity>, Box<dyn Error>>;
}
