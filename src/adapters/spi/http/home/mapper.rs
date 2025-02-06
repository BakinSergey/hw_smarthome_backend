use crate::adapters::spi::http::home::models::HomeApiModel;
use crate::application::mappers::http_mapper::HttpMapper;
use crate::domain::home_entity::HomeEntity;

pub struct HomeHttpMapper {}

impl HttpMapper<HomeEntity, HomeApiModel> for HomeHttpMapper {
    fn to_http(entity: HomeEntity) -> HomeApiModel {
        HomeApiModel {
            id: entity.id,
            name: entity.name,
        }
    }

    fn to_entity(http_obj: HomeApiModel) -> HomeEntity {
        HomeEntity {
            id: http_obj.id,
            name: http_obj.name,
        }
    }
}
