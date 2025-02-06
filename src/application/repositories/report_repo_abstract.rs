use async_trait::async_trait;

use crate::domain::report_entity::ReportEntity;

use crate::domain::info_provider_entity::ProviderInfo;
use std::error::Error;

#[async_trait(?Send)]
pub trait ReportRepoAbstract {
    async fn add(&self, home: &str, query: &ProviderInfo) -> Result<ReportEntity, Box<dyn Error>>;
    async fn list(&self, home: &str) -> Result<Vec<ReportEntity>, Box<dyn Error>>;
}
