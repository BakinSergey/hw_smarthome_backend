use crate::adapters::api::device::device_payload::DevicePayload;
use crate::adapters::api::device::device_presenter::DevicePresenter;
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::device_entity::DeviceEntity;

pub struct DevicePresenterMapper {}

impl ApiMapper<DeviceEntity, DevicePresenter, DevicePayload> for DevicePresenterMapper {
    fn to_api(entity: DeviceEntity) -> DevicePresenter {
        DevicePresenter {
            id: entity.id,
            name: entity.name,
            data: entity.data,
        }
    }

    fn to_entity(_payload: DevicePayload) -> DeviceEntity {
        panic!("not implemented");
    }
}
