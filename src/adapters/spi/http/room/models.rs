use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomApiModel {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomListApiModel {
    pub current_page: i32,
    pub data: Vec<RoomApiModel>,
}


