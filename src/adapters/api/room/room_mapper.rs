use crate::adapters::api::room::room_payload::RoomPayload;
use crate::adapters::api::room::room_presenter::RoomPresenter;
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::room_entity::RoomEntity;

pub struct RoomPresenterMapper {}

impl ApiMapper<RoomEntity, RoomPresenter, RoomPayload> for RoomPresenterMapper {
    fn to_api(entity: RoomEntity) -> RoomPresenter {
        RoomPresenter {
            id: entity.id,
            name: entity.name,
            home_id: entity.home_id.to_string(),
        }
    }

    fn to_entity(_payload: RoomPayload) -> RoomEntity {
        panic!("not implemented");
    }
}
