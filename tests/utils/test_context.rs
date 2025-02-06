use crate::integration::fixtures::fixtures_run;
use diesel::RunQueryDsl;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use fixtures_run::execute_imports;
use smarthome_api::adapters::spi::db::db_connection::DbConnection;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct TestContextPostgreSQL {
    pub base_url: String,
    pub db_name: String,
    pub client: reqwest::Client,
}
type DB = diesel::pg::Pg;

impl TestContextPostgreSQL {
    pub fn new(base_url: &str, db_name: &str) -> Self {
        let db_connection_postgres_db = DbConnection { db_name: "postgres".to_string() };
        let mut conn_postgres_db = db_connection_postgres_db.get_pool().get().expect("couldn't get db connection from pool");

        let query = diesel::sql_query(format!("CREATE DATABASE {};", db_name).as_str());
        query.execute(&mut conn_postgres_db).unwrap_or_else(|_| panic!("couldn't create database {}", db_name));

        let db_connection_test_db = DbConnection { db_name: db_name.to_string() };
        let mut conn_test_db = db_connection_test_db.get_pool().get().expect("couldn't get db connection from pool");

        // create data model
        TestContextPostgreSQL::run_db_migrations(&mut conn_test_db);

        execute_imports(&db_connection_test_db);

        Self {
            base_url: base_url.to_string(),
            db_name: db_name.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn run_db_migrations(conn: &mut impl MigrationHarness<DB>) {
        conn.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");
    }
}

impl Drop for TestContextPostgreSQL {
    fn drop(&mut self) {
        let db_connection_postgres_db = DbConnection { db_name: "postgres".to_string() };
        let mut conn_postgres_db = db_connection_postgres_db.get_pool().get().expect("couldn't get db connection from pool");

        let disconnect_users = format!("SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';", self.db_name);
        diesel::sql_query(disconnect_users.as_str()).execute(&mut conn_postgres_db).unwrap();

        let query = diesel::sql_query(format!("DROP DATABASE {};", self.db_name).as_str());
        query.execute(&mut conn_postgres_db).unwrap_or_else(|_| panic!("couldn't drop test database {}", self.db_name));
    }
}
