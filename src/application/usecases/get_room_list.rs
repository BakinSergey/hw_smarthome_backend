use async_trait::async_trait;

use crate::{
    application::{repositories::room_repo_abstract::RoomRepoAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling::ErrorHandling},
    domain::{error::ApiError, room_entity::RoomEntity},
};

pub struct GetRoomListUseCase<'a> {
    repo: &'a dyn RoomRepoAbstract,
    home: &'a str,
}

impl<'a> GetRoomListUseCase<'a> {
    pub fn new(repo: &'a dyn RoomRepoAbstract, home: &'a str) -> Self {
        GetRoomListUseCase { repo, home }
    }
}

#[async_trait(?Send)]
impl AbstractUseCase<Vec<RoomEntity>> for GetRoomListUseCase<'_> {
    async fn execute(&self) -> Result<Vec<RoomEntity>, ApiError> {
        let room_list = self.repo.list(self.home).await;

        match room_list {
            Ok(rl) => Ok(rl),
            Err(e) => Err(ErrorHandling::application_error("Cannot get room list", Some(e))),
        }
    }
}
