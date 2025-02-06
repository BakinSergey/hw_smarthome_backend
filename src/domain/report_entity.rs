use crate::domain::info_provider_entity::ProviderInfo;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeviceData {
    pub created: String,
    pub name: String,
}

pub type RoomData = HashMap<String, DeviceData>;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ReportContent {
    pub data: HashMap<String, HashMap<String, DeviceData>>,
}

#[derive(Debug, Clone, Default)]
pub struct ReportEntity {
    // отчет о состоянии дома, будем хранить без реляции с Home,
    // в запросе на удаление дома, будет флаг with_report,
    // который зацепит если надо и отчеты для данного дома
    pub id: Uuid,
    pub query: ProviderInfo,
    pub home_id: Uuid,
    pub content: ReportContent,
    pub created: NaiveDateTime,
}

impl ReportEntity {
    pub fn new(id: Uuid, query: ProviderInfo, home_id: Uuid, content: ReportContent) -> Self {
        // let query = serde_json::Value::from(&query);
        // let content = serde_json::Value::from(&content);
        ReportEntity {
            id,
            query,
            home_id,
            content,
            created: Utc::now().naive_utc(),
        }
    }
}
