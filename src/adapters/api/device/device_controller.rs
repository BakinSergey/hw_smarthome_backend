use crate::application::mappers::api_mapper::ApiMapper;
use crate::application::usecases::get_room_device_list::GetRoomDeviceListUseCase;
use crate::application::usecases::interfaces::AbstractUseCase;
use crate::{
    adapters::api::{
        device::device_mapper::DevicePresenterMapper,
        shared::{app_state::AppState, error_presenter::ErrorReponse},
    },
    application::usecases::{add_device_to_room::AddDeviceToRoomUseCase, del_device::DelDeviceFromRoomUseCase},
};
use actix_web::{delete, get, post, web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(add).service(del).service(list);
}

#[post("/add/{home}/{room}/{name}")]
async fn add(data: web::Data<AppState>, path: web::Path<(String, String, String)>) -> Result<HttpResponse, ErrorReponse> {
    let usecase = AddDeviceToRoomUseCase::new(&data.device_repo, &path.0, &path.1, &path.2);
    let device = usecase.execute().await;

    device.map_err(ErrorReponse::map_io_error).map(|d| HttpResponse::Ok().json(DevicePresenterMapper::to_api(d)))
}

#[delete("/del/{home}/{room}/{name}")]
async fn del(data: web::Data<AppState>, path: web::Path<(String, String, String)>) -> Result<HttpResponse, ErrorReponse> {
    let usecase = DelDeviceFromRoomUseCase::new(&data.device_repo, &path.0, &path.1, &path.2);
    let device = usecase.execute().await;

    device.map_err(ErrorReponse::map_io_error).map(|d| HttpResponse::Ok().json(DevicePresenterMapper::to_api(d)))
}

#[get("/list/{home}/{room}")]
async fn list(data: web::Data<AppState>, path: web::Path<(String, String)>) -> Result<HttpResponse, ErrorReponse> {
    let usecase = GetRoomDeviceListUseCase::new(&data.device_repo, &path.0, &path.1);
    let devices = usecase.execute().await;

    devices
        .map_err(ErrorReponse::map_io_error)
        .map(|ds| HttpResponse::Ok().json(ds.into_iter().map(DevicePresenterMapper::to_api).collect::<Vec<_>>()))
}
