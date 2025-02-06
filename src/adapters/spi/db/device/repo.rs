use crate::adapters::spi::db::{db_connection::DbConnection, device::models::NewDevice};
use crate::application::mappers::db_mapper::DbMapper;
use crate::application::repositories::device_repo_abstract::DeviceRepoAbstract;
use crate::domain::device_entity::DeviceEntity;

use crate::adapters::spi::db::device::mapper::DeviceDbMapper;
use crate::adapters::spi::db::device::models::Device;
use crate::adapters::spi::db::home::models::Home;
use crate::adapters::spi::db::room::models::Room;
use async_trait::async_trait;
use diesel::prelude::*;
use serde_json::{json, Value};
use std::error::Error;
use std::sync::Arc;

pub struct DeviceRepo {
    pub db: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl DeviceRepoAbstract for DeviceRepo {
    async fn add(&self, home: &str, room: &str, name: &str) -> Result<DeviceEntity, Box<dyn Error>> {
        use crate::adapters::spi::db::schema::devices::dsl::devices;
        use crate::adapters::spi::db::schema::homes::dsl::{homes, name as hname};
        use crate::adapters::spi::db::schema::rooms::dsl::{home_id, name as rname, rooms};

        let mut conn = self.db.get_pool().get()?;

        let home_result: Option<Home> = homes.filter(hname.eq(home)).first(&mut conn).optional()?;
        match home_result {
            Some(_) => {}
            None => return Err(Box::new(diesel::result::Error::NotFound)),
        }
        let hr: Home = home_result.unwrap();

        let room_result: Option<Room> = rooms.filter(rname.eq(room)).filter(home_id.eq(hr.id)).first(&mut conn).optional()?;

        match room_result {
            Some(_) => {}
            None => return Err(Box::new(diesel::result::Error::NotFound)),
        }
        let rr: Room = room_result.unwrap();

        let device_data: Value = json!({
            "name": name,
            "created": chrono::offset::Utc::now().to_string()
        });

        let new_device = NewDevice { name, data: device_data, room_id: &rr.id };

        let result = diesel::insert_into(devices).values(&new_device).get_result(&mut conn);

        match result {
            Ok(model) => Ok(DeviceDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn delete(&self, home: &str, room: &str, name: &str) -> Result<DeviceEntity, Box<dyn Error>> {
        use crate::adapters::spi::db::schema::devices::dsl::{devices, name as dname, room_id};
        use crate::adapters::spi::db::schema::homes::dsl::{homes, name as hname};
        use crate::adapters::spi::db::schema::rooms::dsl::{home_id, name as rname, rooms};

        let mut conn = self.db.get_pool().get()?;

        let home_result: Option<Home> = homes.filter(hname.eq(home)).first(&mut conn).optional()?;

        match home_result {
            Some(_) => {}
            None => return Err(Box::new(diesel::result::Error::NotFound)),
        };
        let hr: Home = home_result.unwrap();

        let room_result: Option<Room> = rooms.filter(rname.eq(room)).filter(home_id.eq(hr.id)).first(&mut conn).optional()?;

        match room_result {
            Some(_) => {}
            None => return Err(Box::new(diesel::result::Error::NotFound)),
        };
        let rr: Room = room_result.unwrap();

        let device_result = diesel::delete(devices.filter(room_id.eq(rr.id)).filter(dname.eq(name))).get_result::<Device>(&mut conn);

        match device_result {
            Ok(model) => Ok(DeviceDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn list(&self, home: &str, room: &str) -> Result<Vec<DeviceEntity>, Box<dyn Error>> {
        use crate::adapters::spi::db::schema::devices::dsl::{devices, room_id};
        use crate::adapters::spi::db::schema::homes::dsl::{homes, name as hname};
        use crate::adapters::spi::db::schema::rooms::dsl::{home_id, name as rname, rooms};

        let mut conn = self.db.get_pool().get()?;

        let home_result: Option<Home> = homes.filter(hname.eq(home)).first(&mut conn).optional()?;

        match home_result {
            Some(_) => {}
            None => return Err(Box::new(diesel::result::Error::NotFound)),
        };
        let hr: Home = home_result.unwrap();

        let room_result: Option<Room> = rooms.filter(rname.eq(room)).filter(home_id.eq(hr.id)).first(&mut conn).optional()?;

        match room_result {
            Some(_) => {}
            None => return Err(Box::new(diesel::result::Error::NotFound)),
        };
        let rr: Room = room_result.unwrap();

        let result = devices.filter(room_id.eq(rr.id)).get_results::<Device>(&mut conn);

        match result {
            Ok(models) => Ok(models.into_iter().map(DeviceDbMapper::to_entity).collect::<Vec<_>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
