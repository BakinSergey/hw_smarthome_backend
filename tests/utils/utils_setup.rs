use std::{env, net::TcpListener};
use uuid::Uuid;

use super::test_context::TestContextPostgreSQL;

pub fn spawn_app(db_name: &str) -> String {
    // Let the OS assign a port (:0)
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let server = smarthome_api::run(listener, db_name).expect("Failed to bind address");

    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

pub fn setup() -> TestContextPostgreSQL {
    dotenv::from_filename(format!(".env.{}", env::var("ENV").expect("ENV must be set"))).ok();

    TestContextPostgreSQL::new(&dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set"), format!("test_{}", Uuid::new_v4().as_simple()).as_str())
}
