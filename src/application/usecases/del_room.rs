use async_trait::async_trait;

use crate::{
    application::{repositories::room_repo_abstract::RoomRepoAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling::ErrorHandling},
    domain::{error::ApiError, room_entity::RoomEntity},
};

pub struct DelRoomUseCase<'a> {
    repo: &'a dyn RoomRepoAbstract,
    home: &'a str,
    name: &'a str,
}

impl<'a> DelRoomUseCase<'a> {
    pub fn new(repo: &'a dyn RoomRepoAbstract, home: &'a str, name: &'a str) -> Self {
        DelRoomUseCase { repo, home, name }
    }
}

#[async_trait(?Send)]
impl AbstractUseCase<RoomEntity> for DelRoomUseCase<'_> {
    async fn execute(&self) -> Result<RoomEntity, ApiError> {
        let room = self.repo.delete(self.home, self.name).await;

        match room {
            Ok(r) => Ok(r),
            Err(e) => Err(ErrorHandling::application_error("Cannot del room", Some(e))),
        }
    }
}
