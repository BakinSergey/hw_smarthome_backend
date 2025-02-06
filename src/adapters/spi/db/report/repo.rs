use crate::adapters::spi::db::{
    db_connection::DbConnection,
    home::models::Home,
    report::models::{NewReport, Report},
    room::models::Room,
};
use std::collections::HashMap;

use crate::application::{mappers::db_mapper::DbMapper, repositories::report_repo_abstract::ReportRepoAbstract};

use crate::adapters::spi::db::device::models::Device;
use crate::adapters::spi::db::report::mapper::ReportDbMapper;

use crate::domain::info_provider_entity::ProviderInfo;

use crate::domain::report_entity::{DeviceData, ReportContent, ReportEntity, RoomData};

use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use serde_json::json;
use std::error::Error;
use std::sync::Arc;

pub struct ReportRepo {
    pub db: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl ReportRepoAbstract for ReportRepo {
    async fn add(&self, home: &str, query: &ProviderInfo) -> Result<ReportEntity, Box<dyn Error>> {
        use crate::adapters::spi::db::schema::devices::dsl::{devices, name as dname, room_id};
        use crate::adapters::spi::db::schema::homes::dsl::{homes, name as hname};
        use crate::adapters::spi::db::schema::reports::dsl::reports;
        use crate::adapters::spi::db::schema::rooms::dsl::{home_id as rhome_id, name as rname, rooms};

        let mut conn = self.db.get_pool().get()?;

        let home_result: Option<Home> = homes.filter(hname.eq(home)).first(&mut conn).optional()?;

        match home_result {
            Some(_) => {}
            None => return Err(Box::new(diesel::result::Error::NotFound)),
        }

        let hr: Home = home_result.unwrap();
        let queried_rooms = query.query.keys().clone();

        let h_rooms: Vec<Room> = rooms.filter(rname.eq_any(queried_rooms)).filter(rhome_id.eq(hr.id)).get_results(&mut conn)?;

        let mut new_report = NewReport {
            home_id: &hr.id,
            query: &serde_json::to_value(query).unwrap(),
            content: json!({}),
            created: &Utc::now().naive_utc(),
        };

        // let mut content: HashMap<String, HashMap<String, Value>> = HashMap::new();
        let mut content = ReportContent::default();

        for room in h_rooms {
            let room_devices: Vec<Device> = match query.query.get(&room.name) {
                Some(room_names) => devices.filter(room_id.eq(room.id)).filter(dname.eq_any(room_names)).get_results(&mut conn)?,
                None => vec![],
            };

            // let mut dev_content: HashMap<String, Value> = HashMap::new();
            let mut room_content: RoomData = HashMap::new();
            let mut q_devices = query.query.get(&room.name).unwrap().clone();
            let mut processed: Vec<String> = vec![];

            for device in room_devices {
                if q_devices.is_empty() {
                    break;
                }
                if q_devices.contains(&device.name) {
                    // добавляем в отчет
                    let device_data: DeviceData = serde_json::from_value(device.data).unwrap();
                    room_content.insert(device.name.clone(), device_data);
                    processed.push(device.name.clone());
                }
            }

            q_devices.retain(|x| !processed.contains(x));
            // остались устройства к-х нет в комнате - по ним отвечаем NOT FOUND
            for device in q_devices {
                room_content.insert(
                    device,
                    DeviceData {
                        name: "NOT FOUND".to_string(),
                        created: "NOT_FOUND".to_string(),
                    },
                );
            }
            content.data.insert(room.name.clone(), room_content);
        }

        new_report.content = serde_json::to_value(content.data).unwrap();

        let result = diesel::insert_into(reports).values(&new_report).get_result(&mut conn);

        match result {
            Ok(model) => Ok(ReportDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn list(&self, home_v: &str) -> Result<Vec<ReportEntity>, Box<dyn Error>> {
        use crate::adapters::spi::db::schema::homes::dsl::{homes, name};
        use crate::adapters::spi::db::schema::reports::dsl::{home_id, reports};
        let mut conn = self.db.get_pool().get()?;

        let home_result: Option<Home> = homes.filter(name.eq(home_v)).first(&mut conn).optional()?;

        match home_result {
            Some(_) => {}
            None => return Err(Box::new(diesel::result::Error::NotFound)),
        }
        let hr = home_result.unwrap();

        let result = reports.filter(home_id.eq(&hr.id)).get_results::<Report>(&mut conn);

        match result {
            Ok(models) => Ok(models.into_iter().map(ReportDbMapper::to_entity).collect::<Vec<_>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
