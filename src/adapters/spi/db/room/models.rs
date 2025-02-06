use crate::adapters::spi::db::{home::models::Home, schema::*};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, QueryableByName, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Home))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = rooms)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub home_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = rooms)]
pub struct NewRoom<'a> {
    pub home_id: Uuid,
    pub name: &'a str,
}
