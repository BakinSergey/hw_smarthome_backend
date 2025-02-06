use crate::adapters::spi::db::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Queryable, QueryableByName, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = reports)]
pub struct Report {
    pub id: Uuid,
    pub home_id: Uuid,
    pub query: Value,
    pub content: Value,
    pub created: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = reports)]
pub struct NewReport<'a> {
    pub home_id: &'a Uuid,
    pub query: &'a Value,
    pub content: Value,
    pub created: &'a NaiveDateTime,
}
