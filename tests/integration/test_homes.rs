use crate::utils::utils_setup::{setup, spawn_app};
use smarthome_api::adapters::api::home::home_presenter::HomePresenter;

#[actix_rt::test]
async fn test_add_home_uc() {
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    let response = _ctx.client.post(format!("{}/api/v1/home/add/SuperHome", &api_address)).send().await.unwrap();

    assert!(response.status().is_success());

    let content_json = response.json::<HomePresenter>().await.unwrap();

    assert_eq!(content_json.name, "SuperHome");
}

#[actix_rt::test]
async fn test_get_home_list_uc() {
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    let response = _ctx.client.get(format!("{}/api/v1/home/list", &api_address)).send().await.unwrap();

    assert!(response.status().is_success());

    let content_json = response.json::<Vec<HomePresenter>>().await.unwrap();

    assert_eq!(content_json.len(), 3);
    assert_eq!(content_json[0].name, "MyHome");
}

#[actix_rt::test]
async fn test_del_home_uc() {
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    let response = _ctx.client.delete(format!("{}/api/v1/home/del/ScoobeHome", &api_address)).send().await.unwrap();

    assert!(response.status().is_success());

    let content_json = response.json::<HomePresenter>().await.unwrap();

    assert_eq!(content_json.name, "ScoobeHome");
}
