use async_trait::async_trait;

use crate::domain::info_provider_entity::ProviderInfo;
use crate::{
    application::{repositories::report_repo_abstract::ReportRepoAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling::ErrorHandling},
    domain::{error::ApiError, report_entity::ReportEntity},
};

pub struct AddReportUseCase<'a> {
    repo: &'a dyn ReportRepoAbstract,
    home: &'a str,
    query: &'a ProviderInfo,
}

impl<'a> AddReportUseCase<'a> {
    pub fn new(repo: &'a dyn ReportRepoAbstract, home: &'a str, query: &'a ProviderInfo) -> Self {
        AddReportUseCase { repo, home, query }
    }
}

#[async_trait(?Send)]
impl AbstractUseCase<ReportEntity> for AddReportUseCase<'_> {
    async fn execute(&self) -> Result<ReportEntity, ApiError> {
        let report = self.repo.add(self.home, self.query).await;

        match report {
            Ok(r) => Ok(r),
            Err(e) => Err(ErrorHandling::application_error("Cannot create home report", Some(e))),
        }
    }
}
