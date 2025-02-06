use crate::utils::utils_setup::{setup, spawn_app};
use smarthome_api::adapters::api::report::report_presenter::ReportPresenter;
use std::collections::HashMap;

#[actix_rt::test]
async fn test_add_report_uc() {
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    let mut q: HashMap<&str, Vec<&str>> = HashMap::new();
    q.insert("Room1", vec!["Device1.1", "Device1.2"]);
    q.insert("Room2", vec!["Device2.1", "Device2.2"]); // 2.2 - will NOT FOUND
    q.insert("Room3", vec!["Device3.1"]);

    let response = _ctx.client.post(format!("{}/api/v1/report/add/MyHome", &api_address)).json(&q).send().await.unwrap();

    assert!(response.status().is_success());

    let content_json = response.json::<ReportPresenter>().await.unwrap();

    assert_eq!(content_json.home_id.to_string(), "0efe3792-74e1-4202-8a11-5620b2a2b896".to_string());

    assert_eq!(content_json.content.data["Room2"]["Device2.2"].name, "NOT FOUND".to_string());
}

#[actix_rt::test]
async fn test_get_report_list_uc() {
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    let response = _ctx.client.get(format!("{}/api/v1/report/list/MyHome", &api_address)).send().await.unwrap();

    assert!(response.status().is_success());

    let content_json = response.json::<Vec<ReportPresenter>>().await.unwrap();

    assert_eq!(content_json.len(), 0);
}
