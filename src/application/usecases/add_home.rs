use async_trait::async_trait;

use crate::{
    application::{repositories::home_repo_abstract::HomeRepoAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling::ErrorHandling},
    domain::{error::ApiError, home_entity::HomeEntity},
};

pub struct AddHomeUseCase<'a> {
    repo: &'a dyn HomeRepoAbstract,
    name: &'a str,
}

impl<'a> AddHomeUseCase<'a> {
    pub fn new(repo: &'a dyn HomeRepoAbstract, name: &'a str) -> Self {
        AddHomeUseCase { repo, name }
    }
}

#[async_trait(?Send)]
impl AbstractUseCase<HomeEntity> for AddHomeUseCase<'_> {
    async fn execute(&self) -> Result<HomeEntity, ApiError> {
        let home = self.repo.add(self.name).await;

        match home {
            Ok(h) => Ok(h),
            Err(e) => Err(ErrorHandling::application_error("Cannot add home", Some(e))),
        }
    }
}
