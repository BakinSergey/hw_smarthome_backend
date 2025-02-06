use async_trait::async_trait;

use crate::{
    application::{repositories::device_repo_abstract::DeviceRepoAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling::ErrorHandling},
    domain::{device_entity::DeviceEntity, error::ApiError},
};

pub struct GetRoomDeviceListUseCase<'a> {
    repo: &'a dyn DeviceRepoAbstract,
    home: &'a str,
    room: &'a str,
}

impl<'a> GetRoomDeviceListUseCase<'a> {
    pub fn new(repo: &'a dyn DeviceRepoAbstract, home: &'a str, room: &'a str) -> Self {
        Self { repo, home, room }
    }
}

#[async_trait(?Send)]
impl AbstractUseCase<Vec<DeviceEntity>> for GetRoomDeviceListUseCase<'_> {
    async fn execute(&self) -> Result<Vec<DeviceEntity>, ApiError> {
        let device_list = self.repo.list(self.home, self.room).await;

        match device_list {
            Ok(dev_list) => Ok(dev_list),
            Err(e) => Err(ErrorHandling::application_error("Cannot get device list", Some(e))),
        }
    }
}
