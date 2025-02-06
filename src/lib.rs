// #[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate log;
extern crate r2d2;
use actix_web::dev::Server;
use std::net::TcpListener;

pub mod adapters;
pub mod application;
pub mod domain;
pub mod infra;

pub fn run(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    infra::server(listener, db_name)
}
