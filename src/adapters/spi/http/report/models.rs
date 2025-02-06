use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportApiModel {
    pub content: String,
    pub created: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceListApiModel {
    pub current_page: i32,
    pub data: Vec<ReportApiModel>,
}


