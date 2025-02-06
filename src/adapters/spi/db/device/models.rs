use crate::adapters::spi::db::{room::models::Room, schema::*};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, QueryableByName, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Room))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = devices)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub data: serde_json::Value,
    pub room_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = devices)]
pub struct NewDevice<'a> {
    pub name: &'a str,
    pub data: serde_json::Value,
    pub room_id: &'a Uuid,
}
