use async_trait::async_trait;

use crate::{
    application::{repositories::device_repo_abstract::DeviceRepoAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling::ErrorHandling},
    domain::{device_entity::DeviceEntity, error::ApiError},
};

pub struct DelDeviceFromRoomUseCase<'a> {
    repo: &'a dyn DeviceRepoAbstract,
    home: &'a str,
    room: &'a str,
    name: &'a str,
}

impl<'a> DelDeviceFromRoomUseCase<'a> {
    pub fn new(repo: &'a dyn DeviceRepoAbstract, home: &'a str, room: &'a str, name: &'a str) -> Self {
        Self { repo, home, room, name }
    }
}

#[async_trait(?Send)]
impl AbstractUseCase<DeviceEntity> for DelDeviceFromRoomUseCase<'_> {
    async fn execute(&self) -> Result<DeviceEntity, ApiError> {
        let device = self.repo.delete(self.home, self.room, self.name).await;

        match device {
            Ok(dev) => Ok(dev),
            Err(e) => Err(ErrorHandling::application_error("Cannot del device", Some(e))),
        }
    }
}
