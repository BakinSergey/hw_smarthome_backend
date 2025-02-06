use crate::adapters::spi::http::device::models::DeviceApiModel;
use crate::application::mappers::http_mapper::HttpMapper;
use crate::domain::device_entity::DeviceEntity;

pub struct DeviceHttpMapper {}

impl HttpMapper<DeviceEntity, DeviceApiModel> for DeviceHttpMapper {
    fn to_http(entity: DeviceEntity) -> DeviceApiModel {
        DeviceApiModel {
            id: entity.id,
            name: entity.name,
            data: serde_json::to_string(&entity.data).unwrap(),
        }
    }

    fn to_entity(http_obj: DeviceApiModel) -> DeviceEntity {
        DeviceEntity {
            id: http_obj.id,
            name: http_obj.name,
            data: serde_json::Value::from(http_obj.data),
        }
    }
}
