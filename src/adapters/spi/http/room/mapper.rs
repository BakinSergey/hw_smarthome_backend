use crate::adapters::spi::http::room::models::RoomApiModel;
use crate::application::mappers::http_mapper::HttpMapper;
use crate::domain::room_entity::RoomEntity;

pub struct RoomHttpMapper {}

impl HttpMapper<RoomEntity, RoomApiModel> for RoomHttpMapper {
    fn to_http(entity: RoomEntity) -> RoomApiModel {
        RoomApiModel {
            name: entity.name,
        }
    }

    fn to_entity(http_obj: RoomApiModel) -> RoomEntity {
        panic!("not implemented");
    }
}
