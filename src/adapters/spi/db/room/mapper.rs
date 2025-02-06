use crate::adapters::spi::db::room::models::Room;
use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::room_entity::RoomEntity;

pub struct RoomDbMapper {}

impl DbMapper<RoomEntity, Room> for RoomDbMapper {
    fn to_db(_entity: RoomEntity) -> Room {
        panic!("not implemented");
    }

    fn to_entity(model: Room) -> RoomEntity {
        RoomEntity {
            id: model.id,
            name: model.name,
            home_id: model.home_id,
        }
    }
}
