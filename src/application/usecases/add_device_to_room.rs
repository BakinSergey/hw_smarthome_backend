use async_trait::async_trait;

use crate::{
    application::{repositories::device_repo_abstract::DeviceRepoAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling::ErrorHandling},
    domain::{device_entity::DeviceEntity, error::ApiError},
};

pub struct AddDeviceToRoomUseCase<'a> {
    repo: &'a dyn DeviceRepoAbstract,
    home: &'a str,
    room: &'a str,
    name: &'a str,
}

impl<'a> AddDeviceToRoomUseCase<'a> {
    pub fn new(repo: &'a dyn DeviceRepoAbstract, home: &'a str, room: &'a str, name: &'a str) -> Self {
        Self { repo, home, room, name }
    }
}

#[async_trait(?Send)]
impl AbstractUseCase<DeviceEntity> for AddDeviceToRoomUseCase<'_> {
    async fn execute(&self) -> Result<DeviceEntity, ApiError> {
        let device = self.repo.add(self.home, self.room, self.name).await;

        match device {
            Ok(dev) => Ok(dev),
            Err(e) => Err(ErrorHandling::application_error("Cannot add device", Some(e))),
        }
    }
}
