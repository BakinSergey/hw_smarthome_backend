use crate::adapters::api::home::home_payload::HomePayload;
use crate::adapters::api::home::home_presenter::HomePresenter;
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::home_entity::HomeEntity;

pub struct HomePresenterMapper {}

impl ApiMapper<HomeEntity, HomePresenter, HomePayload> for HomePresenterMapper {
    fn to_api(entity: HomeEntity) -> HomePresenter {
        HomePresenter { id: entity.id, name: entity.name }
    }

    fn to_entity(_payload: HomePayload) -> HomeEntity {
        panic!("not implemented");
    }
}
