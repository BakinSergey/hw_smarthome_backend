use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceApiModel {
    pub name: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceListApiModel {
    pub current_page: i32,
    pub data: Vec<DeviceApiModel>,
}


