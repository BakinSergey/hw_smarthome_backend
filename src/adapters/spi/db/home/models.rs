use crate::adapters::spi::db::schema::*;
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Queryable, Insertable, QueryableByName, Selectable, PartialEq, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = homes)]
pub struct Home {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = homes)]
pub struct NewHome<'a> {
    pub name: &'a str,
}
