// mod shared;

use assert_cmd::prelude::*;
use serde_json::Value;
use std::{
    collections::HashMap,
    process::Command,
    sync::{Arc, Mutex, Once},
};

use dotenv::dotenv;
use std::env;

#[derive(Debug, Default)]
struct TestContext {
    ids: Mutex<HashMap<String, String>>,
}

impl TestContext {
    fn new() -> Self {
        Self {
            ids: Mutex::new(HashMap::new()),
        }
    }

    fn set_id(&self, key: &str, value: &str) {
        let mut ids = self.ids.lock().unwrap();
        ids.insert(key.to_string(), value.to_string());
    }

    fn get_id(&self, key: &str) -> Option<String> {
        let ids = self.ids.lock().unwrap();
        ids.get(key).cloned()
    }
}

static INIT: Once = Once::new();
static mut CONTEXT: Option<Arc<TestContext>> = None;

fn setup() -> Arc<TestContext> {
    INIT.call_once(|| {
        let context = Arc::new(TestContext::new());
        unsafe { CONTEXT = Some(context.clone()) };
    });
    unsafe { CONTEXT.clone().unwrap() }
}

fn execute_command(args: &[&str]) -> String {
    let mut cmd = Command::cargo_bin("konnectctl").unwrap();
    let output = cmd
        .args(args)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    String::from_utf8(output).unwrap()
}

fn handle_json_response_commands(output_str: String) -> Value {
    serde_json::from_str(&output_str).unwrap()
}

fn assert_delete_response(value: &str) {
    let out = value.trim_matches('\n').trim_matches('"');
    match out.parse::<i32>() {
        Ok(n) => {
            assert_eq!(n, 204)
        }
        Err(e) => {
            println!("Failed to parse the string: {}", e);
        }
    }
}

fn assert_missing_fields(json_value: &Value, fields: &[&str]) {
    for field in fields {
        assert!(
            json_value.get(field).is_some(),
            "{} field is missing",
            field
        );
    }
}

#[test]
fn test_11_create_api_product() {
    let context = setup();
    let cmd = [
        "create",
        "api-product",
        "--name",
        "Test API Product",
        "--description",
        "Test API Product",
    ];
    let json_value = handle_json_response_commands(execute_command(&cmd));
    let fields = ["id", "name", "description"];
    assert_missing_fields(&json_value, &fields);
    let idout = json_value.get("id").unwrap().to_string();
    let id = idout.trim_matches('"');

    context.set_id("API_PRODUCT_ID", id);
}

#[test]
fn test_12_patch_api_product() {
    // patch API product
    let context = setup();
    let id = context.get_id("API_PRODUCT_ID");
    // println!("Testing Patching Api product");
    match id {
        Some(pid) => {
            let fields = ["id", "name", "description"];

            let json_value = handle_json_response_commands(execute_command(&[
                "patch",
                "api-product",
                "--id",
                &pid,
                "-n",
                "New API Product",
            ]));
            assert_missing_fields(&json_value, &fields);
            assert_eq!(json_value["name"].as_str().unwrap(), "New API Product");
            // println!("Finish testing Patching Api product and fields\n");
        }
        None => assert_eq!(1, 2),
    }
}

#[test]
fn test_13_get_api_product() {
    // println!("Testing Get API product and fields");
    //
    let fields = ["id", "name", "description"];
    let context = setup();
    let id = context.get_id("API_PRODUCT_ID").unwrap();
    let json_value =
        handle_json_response_commands(execute_command(&["get", "api-product", "--id", &id]));
    assert_missing_fields(&json_value, &fields);
    assert_eq!(json_value["id"].as_str().unwrap(), &id);
}
#[test]
fn test_14_create_api_product_version_without_gatewayservice() {
    let context = setup();
    let id = context.get_id("API_PRODUCT_ID").unwrap();
    let fields = ["id", "name"];
    let v_json_value = handle_json_response_commands(execute_command(&[
        "create",
        "api-product-version",
        "-p",
        &id,
        "-n",
        "v1",
        "-s",
        "unpublished",
    ]));
    assert_missing_fields(&v_json_value, &fields);
    let pidout = v_json_value.get("id").unwrap().to_string();
    let pid = pidout.trim_matches('"');
    context.set_id("API_PRODUCT_VERSION_ID", pid);
}

#[test]
fn test_15_get_api_product_version_without_gatewayservice() {
    // println!("Testing Get API product and fields");
    //
    let fields = ["id", "name"];
    let context = setup();
    let pid = context.get_id("API_PRODUCT_ID").unwrap();
    let id = context.get_id("API_PRODUCT_VERSION_ID").unwrap();

    let json_value = handle_json_response_commands(execute_command(&[
        "get",
        "api-product-version",
        "-p",
        &pid,
        "--id",
        &id,
    ]));
    assert_missing_fields(&json_value, &fields);
    assert_eq!(json_value["id"].as_str().unwrap(), &id);
}

#[test]
fn test_16_create_api_product_version_spec() {
    let context = setup();
    let id = context.get_id("API_PRODUCT_ID").unwrap();
    let vid = context.get_id("API_PRODUCT_VERSION_ID").unwrap();
    let fields = ["id", "name"];
    let v_json_value = handle_json_response_commands(execute_command(&[
        "create",
        "api-product-version-specification",
        "-p",
        &id,
        "-v",
        &vid,
        "-f",
        "./tests/spec/spec-file.yaml",
    ]));
    assert_missing_fields(&v_json_value, &fields);
    let pidout = v_json_value.get("id").unwrap().to_string();
    let sid = pidout.trim_matches('"');
    context.set_id("API_PRODUCT_VERSION_SPEC_ID", sid);
}

#[test]
fn test_17_patch_api_product_version_spec() {
    let context = setup();
    let id = context.get_id("API_PRODUCT_ID").unwrap();
    let vid = context.get_id("API_PRODUCT_VERSION_ID").unwrap();
    let sid = context.get_id("API_PRODUCT_VERSION_SPEC_ID").unwrap();
    let fields = ["id", "name"];
    let v_json_value = handle_json_response_commands(execute_command(&[
        "patch",
        "api-product-version-specification",
        "-p",
        &id,
        "-v",
        &vid,
        "-i",
        &sid,
        "-f",
        "./tests/spec/another_spec.yaml",
    ]));
    assert_missing_fields(&v_json_value, &fields);
    let pidout = v_json_value.get("id").unwrap().to_string();
    let sid = pidout.trim_matches('"');
    context.set_id("API_PRODUCT_VERSION_SPEC_ID", sid);
}

#[test]
fn test_18_delete_api_product_version_specification() {
    // Delete API Product
    let context = setup();
    let pid = context.get_id("API_PRODUCT_ID").unwrap();
    let vid = context.get_id("API_PRODUCT_VERSION_ID").unwrap();
    let sid = context.get_id("API_PRODUCT_VERSION_SPEC_ID").unwrap();
    assert_delete_response(&execute_command(&[
        "delete",
        "api-product-version-specification",
        "--id",
        &sid,
        "-p",
        &pid,
        "-v",
        &vid,
    ]));
}

#[test]
fn test_19_delete_api_product_version() {
    // Delete API Product
    let context = setup();
    let id = context.get_id("API_PRODUCT_ID").unwrap();
    let vid = context.get_id("API_PRODUCT_VERSION_ID").unwrap();
    assert_delete_response(&execute_command(&[
        "delete",
        "api-product-version",
        "--id",
        &vid,
        "-p",
        &id,
    ]));
}

#[test]
fn test_20_create_api_product_version_with_gatewayservice() {
    dotenv().ok();

    let cpid = env::var("TEST_CONTROL_PLANE_ID").expect("Control Plane Id must be set");
    let gsid = env::var("TEST_GATEWAY_SERVICE_ID").expect("Control Plane Id must be set");
    let context = setup();
    let id = context.get_id("API_PRODUCT_ID").unwrap();
    let fields = ["id", "name"];
    let v_json_value = handle_json_response_commands(execute_command(&[
        "create",
        "api-product-version",
        "-p",
        &id,
        "-c",
        &cpid,
        "-g",
        &gsid,
        "-n",
        "v1",
        "-s",
        "unpublished",
    ]));
    assert_missing_fields(&v_json_value, &fields);
    let pidout = v_json_value.get("id").unwrap().to_string();
    let pid = pidout.trim_matches('"');
    context.set_id("API_PRODUCT_VERSION_ID", pid);
}

#[test]
fn test_21_get_api_product_version_with_gatewayservice() {
    // println!("Testing Get API product and fields");
    //
    let fields = ["id", "name"];
    let context = setup();
    let pid = context.get_id("API_PRODUCT_ID").unwrap();
    let id = context.get_id("API_PRODUCT_VERSION_ID").unwrap();

    let json_value = handle_json_response_commands(execute_command(&[
        "get",
        "api-product-version",
        "-p",
        &pid,
        "--id",
        &id,
    ]));
    assert_missing_fields(&json_value, &fields);
    assert_eq!(json_value["id"].as_str().unwrap(), &id);
}

#[test]
fn test_22_delete_api_product() {
    let context = setup();
    let id = context.get_id("API_PRODUCT_ID").unwrap();
    assert_delete_response(&execute_command(&["delete", "api-product", "--id", &id]));
}
// Delete API Product
