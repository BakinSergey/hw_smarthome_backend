use diesel::{insert_into, RunQueryDsl};
use smarthome_api::adapters::spi::db::db_connection::DbConnection;
use smarthome_api::adapters::spi::db::schema::{devices::dsl::devices, homes::dsl::homes, rooms::dsl::rooms};

use crate::{
    integration::fixtures::fixtures_struct::{DeviceJson, HomeJson, RoomJson},
    utils::utils_file::read_from_file,
};

pub fn execute_imports(conn: &DbConnection) {
    load_homes(conn);
    load_rooms(conn);
    load_devices(conn);
}

fn load_homes(conn: &DbConnection) {
    let json = read_from_file::<Vec<HomeJson>>("tests/integration/fixtures/home.json").unwrap();

    let mut conn = conn.get_pool().get().expect("couldn't get db connection from pool");
    insert_into(homes).values(&json).execute(&mut conn).unwrap();
}

fn load_rooms(conn: &DbConnection) {
    let json = read_from_file::<Vec<RoomJson>>("tests/integration/fixtures/room.json").unwrap();

    let mut conn = conn.get_pool().get().expect("couldn't get db connection from pool");
    insert_into(rooms).values(&json).execute(&mut conn).unwrap();
}

fn load_devices(conn: &DbConnection) {
    let json = read_from_file::<Vec<DeviceJson>>("tests/integration/fixtures/device.json").unwrap();

    let mut conn = conn.get_pool().get().expect("couldn't get db connection from pool");
    insert_into(devices).values(&json).execute(&mut conn).unwrap();
}
