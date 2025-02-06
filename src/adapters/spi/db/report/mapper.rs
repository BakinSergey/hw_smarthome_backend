use crate::adapters::spi::db::report::models::Report;
use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::info_provider_entity::ProviderInfo;
use crate::domain::report_entity::{ReportContent, ReportEntity, RoomData};
use std::collections::HashMap;

pub struct ReportDbMapper {}

impl DbMapper<ReportEntity, Report> for ReportDbMapper {
    fn to_db(_entity: ReportEntity) -> Report {
        panic!("not implemented yet")
    }

    fn to_entity(model: Report) -> ReportEntity {
        println!("{:?}", serde_json::to_string(&model.content).unwrap());
        let content_v: ReportContent = {
            let data: HashMap<String, RoomData> = serde_json::from_value(model.content).unwrap();
            ReportContent { data }
        };

        let query_v: ProviderInfo = serde_json::from_value(model.query).unwrap();

        ReportEntity {
            id: model.id,
            query: query_v,
            home_id: model.home_id,
            content: content_v,
            created: model.created,
        }
    }
}
