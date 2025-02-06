use crate::adapters::spi::db::device::models::Device;
use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::device_entity::DeviceEntity;

pub struct DeviceDbMapper {}

impl DbMapper<DeviceEntity, Device> for DeviceDbMapper {
    fn to_db(_entity: DeviceEntity) -> Device {
        panic!("not implemented");
    }

    fn to_entity(model: Device) -> DeviceEntity {
        DeviceEntity {
            id: model.id,
            name: model.name,
            data: model.data,
        }
    }
}
