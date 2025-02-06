use crate::adapters::spi::http::report::models::ReportApiModel;
use crate::application::mappers::http_mapper::HttpMapper;
use crate::domain::report_entity::ReportEntity;

pub struct ReportHttpMapper {}

impl HttpMapper<ReportEntity, ReportApiModel> for ReportHttpMapper {
    fn to_http(entity: ReportEntity) -> ReportApiModel {
        ReportApiModel {
            content: serde_json::to_string(&entity.content).unwrap(),
            created: entity.created,
        }
    }

    fn to_entity(http_obj: ReportApiModel) -> ReportEntity {
        panic!("not implemented");
    }
}
