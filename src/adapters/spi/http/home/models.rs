use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct HomeApiModel {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HomeListApiModel {
    pub current_page: i32,
    pub data: Vec<HomeApiModel>,
}


