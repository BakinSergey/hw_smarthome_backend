use crate::adapters::{
    self,
    api::shared::app_state::AppState,
    spi::db::{db_connection::DbConnection, device::repo::DeviceRepo, home::repo::HomeRepo, report::repo::ReportRepo, room::repo::RoomRepo},
};
use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use std::{env, net::TcpListener};

pub fn server(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    let _ = env_logger::try_init();

    let db_connection = Arc::new(DbConnection { db_name: db_name.to_string() });

    let data = web::Data::new(AppState {
        app_name: String::from("SmartHomeAPI"),
        home_repo: HomeRepo { db: db_connection.clone() },
        room_repo: RoomRepo { db: db_connection.clone() },
        device_repo: DeviceRepo { db: db_connection.clone() },
        report_repo: ReportRepo { db: db_connection.clone() },
    });

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || App::new().app_data(data.clone()).wrap(Logger::default()).configure(adapters::api::shared::routes::routes))
        .listen(listener)?
        .run();

    println!("Server running on port {}, db_name {}", port, db_name);

    Ok(server)
}
