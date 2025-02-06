use crate::application::mappers::api_mapper::ApiMapper;
use crate::application::usecases::get_report_list::GetReportListUseCase;
use crate::application::usecases::interfaces::AbstractUseCase;
use crate::domain::info_provider_entity::ProviderInfo;
use crate::{
    adapters::api::{
        report::report_mapper::ReportPresenterMapper,
        shared::{app_state::AppState, error_presenter::ErrorReponse},
    },
    application::usecases::add_report::AddReportUseCase,
};
use actix_web::{get, post, web, HttpResponse};
use std::collections::HashMap;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(add_report).service(list_report);
}

#[post("/add/{name}")]
async fn add_report(req: web::Json<HashMap<String, Vec<String>>>, data: web::Data<AppState>, path: web::Path<(String,)>) -> Result<HttpResponse, ErrorReponse> {
    let query = ProviderInfo { query: req.clone() };

    let usecase = AddReportUseCase::new(&data.report_repo, &path.0, &query);
    let report = usecase.execute().await;

    report.map_err(ErrorReponse::map_io_error).map(|r| HttpResponse::Ok().json(ReportPresenterMapper::to_api(r)))
}

#[get("/list/{name}")]
async fn list_report(data: web::Data<AppState>, path: web::Path<(String,)>) -> Result<HttpResponse, ErrorReponse> {
    let usecase = GetReportListUseCase::new(&data.report_repo, &path.0);
    let reports = usecase.execute().await;

    reports
        .map_err(ErrorReponse::map_io_error)
        .map(|rs| HttpResponse::Ok().json(rs.into_iter().map(ReportPresenterMapper::to_api).collect::<Vec<_>>()))
}
