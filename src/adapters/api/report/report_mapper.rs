use crate::adapters::api::report::report_payload::ReportPayload;
use crate::adapters::api::report::report_presenter::ReportPresenter;
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::report_entity::ReportEntity;

pub struct ReportPresenterMapper {}

impl ApiMapper<ReportEntity, ReportPresenter, ReportPayload> for ReportPresenterMapper {
    fn to_api(entity: ReportEntity) -> ReportPresenter {
        ReportPresenter {
            id: entity.id,
            query: entity.query,
            home_id: entity.home_id,
            content: entity.content,
            created: entity.created,
        }
    }

    fn to_entity(_payload: ReportPayload) -> ReportEntity {
        panic!("not implemented");
    }
}
