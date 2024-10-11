use std::fs;

use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::cli::api_product_version_spec::{
    CreateApiProductVersionSpec, DeleteApiProductVersionSpec, PatchApiProductVersionSpec,
};

use super::{
    api_utils::{get_filename_from_path, string_to_base64, Executable},
    common_structs::Meta,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchAPIProductVersionSpecRequest {
    pub name: String,
    pub content: String,
}

impl Executable for PatchAPIProductVersionSpecRequest {
    type Response = APIProductVersionSpecResponse;
}

impl PatchAPIProductVersionSpecRequest {
    pub fn new(spec_cli: &PatchApiProductVersionSpec) -> Self {
        let path = &spec_cli.path_to_spec_file;
        let name = get_filename_from_path(path).expect("Please pass a valid file name");
        let file_content = fs::read_to_string(path);
        let content = match file_content {
            Ok(c) => {
                info!("Content: {}", &c);
                string_to_base64(c)
            }
            Err(er) => {
                error!("Error reading file {}", er);
                "NO_CONTENT".to_string()
            }
        };

        PatchAPIProductVersionSpecRequest {
            name: name.to_string(),
            content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAPIProductVersionSpecRequest {
    pub id: String,
    pub product_id: String,
    pub product_version_id: String,
}

impl Executable for DeleteAPIProductVersionSpecRequest {
    type Response = DeleteResponse;
}

impl DeleteAPIProductVersionSpecRequest {
    pub fn new(spec_cli: &DeleteApiProductVersionSpec) -> Self {
        let id = &spec_cli.id;
        let product_id = &spec_cli.product_id;
        let product_version_id = &spec_cli.product_version_id;
        DeleteAPIProductVersionSpecRequest {
            id: id.to_string(),
            product_id: product_id.to_string(),
            product_version_id: product_version_id.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    respons_code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAPIProductVersionSpecRequest {}

impl Executable for GetAPIProductVersionSpecRequest {
    type Response = APIProductVersionSpecResponse;
}

impl GetAPIProductVersionSpecRequest {
    pub fn new() -> Self {
        GetAPIProductVersionSpecRequest {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllAPIProductVersionSpecRequest {}

impl Executable for GetAllAPIProductVersionSpecRequest {
    type Response = GetAllAPIProductVersionSpecResponse;
}

impl GetAllAPIProductVersionSpecRequest {
    pub fn new() -> Self {
        GetAllAPIProductVersionSpecRequest {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllAPIProductVersionSpecResponse {
    data: Vec<APIProductVersionSpecResponse>,
    meta: Meta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAPIProductVersionSpecRequest {
    pub name: String,
    pub content: String,
}
// Function to extract the filename from a path

impl CreateAPIProductVersionSpecRequest {
    pub fn new(spec_cli: CreateApiProductVersionSpec) -> Self {
        // let file = File::open(spec_cli.path_to_spec_file));
        let path = spec_cli.path_to_spec_file;
        let name_t = get_filename_from_path(&path);
        let mut name = String::new();
        match name_t {
            Some(f) => name = f.to_string(),
            None => error!("Error getting file name"),
        }
        let mut content = String::new();
        let file_content = fs::read_to_string(&path);
        match file_content {
            Ok(c) => {
                info!("Content: {}", &c);
                content = string_to_base64(c);
            }
            Err(er) => {
                error!("Error reading file {}", er);
            }
        }

        CreateAPIProductVersionSpecRequest { name, content }
    }
}

impl Executable for CreateAPIProductVersionSpecRequest {
    // add code here
    type Response = APIProductVersionSpecResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIProductVersionSpecResponse {
    id: String,
    name: String,
    content: String,
    created_at: String,
    updated_at: String,
}
