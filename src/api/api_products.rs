use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use crate::cli;

use super::api_utils;
use super::api_utils::Executable;
use super::common_structs::Meta;

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchAPIProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    labels: Option<HashMap<String, String>>,
    portal_ids: Option<Vec<String>>,
}
impl PatchAPIProductRequest {
    pub fn new(api_product: &cli::api_products_cli::PatchApiProduct) -> Self {
        let name = &api_product.name;
        let description = &api_product.description;
        let hs = match &api_product.labels {
            Some(lbs) => Some(api_utils::string_to_hashmap(lbs.to_string())),
            None => Some(HashMap::new()),
        };
        let pids = match &api_product.portal_ids {
            Some(ps) => Some(ps.split(',').map(|s| s.to_string()).collect()),
            None => Some(Vec::new()),
        };
        PatchAPIProductRequest {
            name: name.clone(),
            description: description.clone(),
            labels: hs,
            portal_ids: pids,
        }
    }

    // pub fn to_filtered_json(&self) -> Value {
    //     let json = serde_json::to_value(self).unwrap();
    //     Self::filter_empty_fields(json)
    // }
    //
    // pub fn filter_empty_fields(value: Value) -> Value {
    //     match value {
    //         Value::Object(map) => {
    //             let filtered_map = map
    //                 .into_iter()
    //                 .filter_map(|(k, v)| {
    //                     if k == "id" {
    //                         return None;
    //                     }
    //                     match &v {
    //                         Value::String(s) if s.is_empty() => None,
    //                         Value::Array(arr) if arr.is_empty() => None,
    //                         Value::Object(obj) if obj.is_empty() => None,
    //                         Value::Null => None,
    //                         _ => Some((k, Self::filter_empty_fields(v))),
    //                     }
    //                 })
    //                 .collect();
    //             Value::Object(filtered_map)
    //         }
    //         _ => value,
    //     }
    // }
}

impl Executable for PatchAPIProductRequest {
    type Response = APIProductResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAPIProductRequest {
    pub id: String,
}

impl Executable for DeleteAPIProductRequest {
    type Response = DeleteAPIProductResponse;
}

impl DeleteAPIProductRequest {
    pub fn new(delp: cli::api_products_cli::DeleteApiProduct) -> Self {
        let id = delp.id;
        DeleteAPIProductRequest { id }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAPIProductResponse {
    response_code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIProductError {
    status: u16,
    title: String,
    instance: String,
    detail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIProductRequest {
    name: String,
    description: String,
    labels: Option<HashMap<String, String>>,
    portal_ids: Option<Vec<String>>,
}
impl APIProductRequest {
    pub fn new(api_product: cli::api_products_cli::CreateApiProduct) -> Self {
        let hs = match api_product.labels {
            Some(lbs) => Some(api_utils::string_to_hashmap(lbs)),
            None => Some(HashMap::new()),
        };
        let pids = match api_product.portal_ids {
            Some(ps) => Some(ps.split(',').map(|s| s.to_string()).collect()),
            None => Some(Vec::new()),
        };
        APIProductRequest {
            name: api_product.name,
            description: api_product.description,
            labels: hs,
            portal_ids: pids,
        }
    }
}

impl Executable for APIProductRequest {
    type Response = APIProductResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAPIProductRequest {
    pub id: String,
    pub name: String,
}

impl GetAPIProductRequest {
    pub fn new(gap: cli::api_products_cli::GetApiProduct) -> Self {
        GetAPIProductRequest {
            id: gap.id.unwrap_or("".to_string()),
            name: gap.name.unwrap_or("".to_string()),
        }
    }
}

impl Executable for GetAPIProductRequest {
    type Response = APIProductResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAPIProductsArrayRequest {}

impl Executable for GetAPIProductsArrayRequest {
    type Response = APIProductsArrayResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIProductsArrayResponse {
    data: Vec<APIProductResponse>,
    pub meta: Meta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIProductResponse {
    pub id: String,
    name: String,
    description: Option<String>,
    portal_ids: Vec<String>,
    created_at: String,
    updated_at: String,
    labels: HashMap<String, String>,
}

// pub struct APIProduct {
//     path: String,
// }
