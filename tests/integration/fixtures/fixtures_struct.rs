use diesel::Insertable;
use serde::Deserialize;
use smarthome_api::adapters::spi::db::schema::*;
use uuid::Uuid;

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = homes)]
pub struct HomeJson {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = rooms)]
pub struct RoomJson {
    pub id: Uuid,
    pub name: String,
    pub home_id: Uuid,
}

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = devices)]
pub struct DeviceJson {
    pub id: Uuid,
    pub name: String,
    pub data: serde_json::Value,
    pub room_id: Uuid,
}
