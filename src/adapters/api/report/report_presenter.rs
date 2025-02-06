use crate::domain::info_provider_entity::ProviderInfo;
use crate::domain::report_entity::ReportContent;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportPresenter {
    pub id: Uuid,
    pub query: ProviderInfo,
    pub home_id: Uuid,
    pub content: ReportContent,
    pub created: NaiveDateTime,
}
