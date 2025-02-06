use crate::utils::utils_setup::{setup, spawn_app};
use smarthome_api::adapters::api::device::device_presenter::DevicePresenter;

#[actix_rt::test]
async fn test_add_device_to_room_uc() {
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    let response = _ctx.client.post(format!("{}/api/v1/device/add/MyHome/Room3/NewDevice", &api_address)).send().await.unwrap();

    assert!(response.status().is_success());

    let content_json = response.json::<DevicePresenter>().await.unwrap();

    assert_eq!(content_json.name, "NewDevice");
}

#[actix_rt::test]
async fn test_get_room_device_list_uc() {
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    let response = _ctx.client.get(format!("{}/api/v1/device/list/MyHome/Room1", &api_address)).send().await.unwrap();

    assert!(response.status().is_success());

    let content_json = response.json::<Vec<DevicePresenter>>().await.unwrap();

    assert_eq!(content_json.len(), 2);
}

#[actix_rt::test]
async fn test_del_device_uc() {
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    let response = _ctx.client.delete(format!("{}/api/v1/device/del/MyHome/Room1/Device1.2", &api_address)).send().await.unwrap();

    assert!(response.status().is_success());

    let content_json = response.json::<DevicePresenter>().await.unwrap();

    assert_eq!(content_json.name, "Device1.2");
}
