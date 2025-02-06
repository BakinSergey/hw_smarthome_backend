use crate::adapters::spi::db::home::models::Home;
use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::home_entity::HomeEntity;

pub struct HomeDbMapper {}

impl DbMapper<HomeEntity, Home> for HomeDbMapper {
    fn to_db(_entity: HomeEntity) -> Home {
        panic!("not implemented");
    }

    fn to_entity(model: Home) -> HomeEntity {
        HomeEntity { id: model.id, name: model.name }
    }
}
