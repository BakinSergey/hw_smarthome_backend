use actix_web::web;

use crate::adapters::api::{device::device_controller, home::home_controller, report::report_controller, room::room_controller};

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/api/v1/home").configure(home_controller::routes))
        .service(web::scope("/api/v1/room").configure(room_controller::routes))
        .service(web::scope("/api/v1/device").configure(device_controller::routes))
        .service(web::scope("/api/v1/report").configure(report_controller::routes));
}
