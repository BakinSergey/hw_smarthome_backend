use crate::application::mappers::api_mapper::ApiMapper;
use crate::application::usecases::get_room_list::GetRoomListUseCase;
use crate::application::usecases::interfaces::AbstractUseCase;
use crate::{
    adapters::api::{
        room::room_mapper::RoomPresenterMapper,
        shared::{app_state::AppState, error_presenter::ErrorReponse},
    },
    application::usecases::{add_room_to_home::AddRoomToHomeUseCase, del_room::DelRoomUseCase},
};
use actix_web::{delete, get, post, web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(add).service(del).service(list);
}

#[post("/add/{home}/{name}")]
async fn add(data: web::Data<AppState>, path: web::Path<(String, String)>) -> Result<HttpResponse, ErrorReponse> {
    let usecase = AddRoomToHomeUseCase::new(&data.room_repo, &path.0, &path.1);
    let room = usecase.execute().await;

    room.map_err(ErrorReponse::map_io_error).map(|r| HttpResponse::Ok().json(RoomPresenterMapper::to_api(r)))
}

#[delete("/del/{home}/{name}")]
async fn del(data: web::Data<AppState>, path: web::Path<(String, String)>) -> Result<HttpResponse, ErrorReponse> {
    let usecase = DelRoomUseCase::new(&data.room_repo, &path.0, &path.1);
    let room = usecase.execute().await;

    room.map_err(ErrorReponse::map_io_error).map(|r| HttpResponse::Ok().json(RoomPresenterMapper::to_api(r)))
}

#[get("/list/{home}")]
async fn list(data: web::Data<AppState>, path: web::Path<(String,)>) -> Result<HttpResponse, ErrorReponse> {
    let usecase = GetRoomListUseCase::new(&data.room_repo, &path.0);
    let rooms = usecase.execute().await;

    rooms
        .map_err(ErrorReponse::map_io_error)
        .map(|rs| HttpResponse::Ok().json(rs.into_iter().map(RoomPresenterMapper::to_api).collect::<Vec<_>>()))
}
