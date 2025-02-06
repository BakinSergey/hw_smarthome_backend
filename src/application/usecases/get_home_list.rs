use async_trait::async_trait;

use crate::{
    application::{repositories::home_repo_abstract::HomeRepoAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling::ErrorHandling},
    domain::{error::ApiError, home_entity::HomeEntity},
};

pub struct GetHomeListUseCase<'a> {
    repo: &'a dyn HomeRepoAbstract,
}

impl<'a> GetHomeListUseCase<'a> {
    pub fn new(repo: &'a dyn HomeRepoAbstract) -> Self {
        GetHomeListUseCase { repo }
    }
}

#[async_trait(?Send)]
impl AbstractUseCase<Vec<HomeEntity>> for GetHomeListUseCase<'_> {
    async fn execute(&self) -> Result<Vec<HomeEntity>, ApiError> {
        let home_list = self.repo.list().await;

        match home_list {
            Ok(hl) => Ok(hl),
            Err(e) => Err(ErrorHandling::application_error("Cannot get home list", Some(e))),
        }
    }
}
