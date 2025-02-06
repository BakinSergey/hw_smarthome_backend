use crate::adapters::spi::{db::device::repo::DeviceRepo, db::home::repo::HomeRepo, db::report::repo::ReportRepo, db::room::repo::RoomRepo};

pub struct AppState {
    pub app_name: String,
    pub home_repo: HomeRepo,
    pub room_repo: RoomRepo,
    pub device_repo: DeviceRepo,
    pub report_repo: ReportRepo,
}
