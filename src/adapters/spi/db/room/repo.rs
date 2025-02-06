use crate::adapters::spi::db::{
    db_connection::DbConnection,
    home::models::Home,
    room::mapper::RoomDbMapper,
    room::models::{NewRoom, Room},
};

use crate::application::{mappers::db_mapper::DbMapper, repositories::room_repo_abstract::RoomRepoAbstract};

use crate::domain::room_entity::RoomEntity;
use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;
use std::sync::Arc;

pub struct RoomRepo {
    pub db: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl RoomRepoAbstract for RoomRepo {
    async fn add(&self, home: &str, name: &str) -> Result<RoomEntity, Box<dyn Error>> {
        use crate::adapters::spi::db::schema::homes::dsl::{homes, name as hname};
        use crate::adapters::spi::db::schema::rooms::dsl::rooms;

        let mut conn = self.db.get_pool().get()?;

        let home_result: Option<Home> = homes.filter(hname.eq(home)).first(&mut conn).optional()?;

        match home_result {
            Some(_) => {}
            None => return Err(Box::new(diesel::result::Error::NotFound)),
        }
        let home_result: Home = home_result.unwrap();
        let new_room = NewRoom { name, home_id: home_result.id };

        let result = diesel::insert_into(rooms).values(&new_room).get_result::<Room>(&mut conn);

        match result {
            Ok(model) => Ok(RoomDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn delete(&self, home: &str, name: &str) -> Result<RoomEntity, Box<dyn Error>> {
        use crate::adapters::spi::db::schema::homes::dsl::{homes, name as hname};
        use crate::adapters::spi::db::schema::rooms::dsl::{home_id, name as rname, rooms};

        let mut conn = self.db.get_pool().get()?;

        let home_result: Option<Home> = homes.filter(hname.eq(home)).first(&mut conn).optional()?;

        match home_result {
            Some(_) => {}
            None => return Err(Box::new(diesel::result::Error::NotFound)),
        };
        let hr: Home = home_result.unwrap();
        let room_result = diesel::delete(rooms.filter(home_id.eq(hr.id)).filter(rname.eq(name))).get_result::<Room>(&mut conn);

        match room_result {
            Ok(model) => Ok(RoomDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn list(&self, home_v: &str) -> Result<Vec<RoomEntity>, Box<dyn Error>> {
        use crate::adapters::spi::db::schema::homes::dsl::{homes, name as hname};
        use crate::adapters::spi::db::schema::rooms::dsl::{home_id, rooms};
        let mut conn = self.db.get_pool().get()?;

        let home_result: Option<Home> = homes.filter(hname.eq(home_v)).first(&mut conn).optional()?;

        match home_result {
            Some(_) => {}
            None => return Err(Box::new(diesel::result::Error::NotFound)),
        }

        let hr = home_result.unwrap();

        let result = rooms.filter(home_id.eq(hr.id)).get_results::<Room>(&mut conn);

        match result {
            Ok(models) => Ok(models.into_iter().map(RoomDbMapper::to_entity).collect::<Vec<_>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
