use crate::application::mappers::api_mapper::ApiMapper;
use crate::application::usecases::get_home_list::GetHomeListUseCase;
use crate::application::usecases::interfaces::AbstractUseCase;
use crate::{
    adapters::api::{
        home::home_mapper::HomePresenterMapper,
        shared::{app_state::AppState, error_presenter::ErrorReponse},
    },
    application::usecases::{add_home::AddHomeUseCase, del_home::DelHomeUseCase},
};
use actix_web::{delete, get, post, web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(add).service(del).service(list);
}

#[post("/add/{name}")]
async fn add(data: web::Data<AppState>, path: web::Path<(String,)>) -> Result<HttpResponse, ErrorReponse> {
    let usecase = AddHomeUseCase::new(&data.home_repo, &path.0);
    let home = usecase.execute().await;

    home.map_err(ErrorReponse::map_io_error).map(|h| HttpResponse::Ok().json(HomePresenterMapper::to_api(h)))
}

#[delete("/del/{name}")]
async fn del(data: web::Data<AppState>, path: web::Path<(String,)>) -> Result<HttpResponse, ErrorReponse> {
    let usecase = DelHomeUseCase::new(&data.home_repo, &path.0);
    let home = usecase.execute().await;

    home.map_err(ErrorReponse::map_io_error).map(|h| HttpResponse::Ok().json(HomePresenterMapper::to_api(h)))
}

#[get("/list")]
async fn list(data: web::Data<AppState>) -> Result<HttpResponse, ErrorReponse> {
    let usecase = GetHomeListUseCase::new(&data.home_repo);
    let homes = usecase.execute().await;

    homes
        .map_err(ErrorReponse::map_io_error)
        .map(|hs| HttpResponse::Ok().json(hs.into_iter().map(HomePresenterMapper::to_api).collect::<Vec<_>>()))
}
