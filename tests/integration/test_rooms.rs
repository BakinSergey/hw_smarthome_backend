use crate::utils::utils_setup::{setup, spawn_app};
use smarthome_api::adapters::api::room::room_presenter::RoomPresenter;

#[actix_rt::test]
async fn test_add_room_to_home_uc() {
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    let response = _ctx.client.post(format!("{}/api/v1/room/add/MyHome/NewRoom", &api_address)).send().await.unwrap();

    assert!(response.status().is_success());

    let content_json = response.json::<RoomPresenter>().await.unwrap();

    assert_eq!(content_json.name, "NewRoom");
}

#[actix_rt::test]
async fn test_get_room_list_uc() {
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    let response = _ctx.client.get(format!("{}/api/v1/room/list/MyHome", &api_address)).send().await.unwrap();

    assert!(response.status().is_success());

    let content_json = response.json::<Vec<RoomPresenter>>().await.unwrap();

    assert_eq!(content_json.len(), 3);
}

#[actix_rt::test]
async fn test_del_room_uc() {
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    let response = _ctx.client.delete(format!("{}/api/v1/room/del/MyHome/Room1", &api_address)).send().await.unwrap();

    assert!(response.status().is_success());

    let content_json = response.json::<RoomPresenter>().await.unwrap();

    assert_eq!(content_json.name, "Room1");
}
