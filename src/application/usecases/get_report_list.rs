use async_trait::async_trait;

use crate::{
    application::{repositories::report_repo_abstract::ReportRepoAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling::ErrorHandling},
    domain::{error::ApiError, report_entity::ReportEntity},
};

pub struct GetReportListUseCase<'a> {
    repo: &'a dyn ReportRepoAbstract,
    home: &'a str,
}

impl<'a> GetReportListUseCase<'a> {
    pub fn new(repo: &'a dyn ReportRepoAbstract, home: &'a str) -> Self {
        GetReportListUseCase { repo, home }
    }
}

#[async_trait(?Send)]
impl AbstractUseCase<Vec<ReportEntity>> for GetReportListUseCase<'_> {
    async fn execute(&self) -> Result<Vec<ReportEntity>, ApiError> {
        let report_list = self.repo.list(self.home).await;

        match report_list {
            Ok(rl) => Ok(rl),
            Err(e) => Err(ErrorHandling::application_error("Cannot get home report list", Some(e))),
        }
    }
}
