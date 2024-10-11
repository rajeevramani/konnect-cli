use std::collections::HashMap;

use crate::cli::api_product_versions_cli::{
    CreateApiProductVersion, DeleteAPIProductVersion, PublishStatus,
};
use log::info;
use serde::{Deserialize, Serialize};

use super::{api_utils::Executable, common_structs::Meta};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAPIProductVersionRequest {
    pub id: String,
    pub product_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAPIProductVersionResponse {
    response_code: i32,
}

impl DeleteAPIProductVersionRequest {
    pub fn new(dprv: DeleteAPIProductVersion) -> Self {
        DeleteAPIProductVersionRequest {
            id: dprv.id,
            product_id: dprv.product_id,
        }
    }
}

impl Executable for DeleteAPIProductVersionRequest {
    // add code here
    type Response = DeleteAPIProductVersionResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIProductVersionRequest {
    name: String,
    publish_status: String,
    deprecated: bool,
    gateway_service: Option<APIProductVersionGatewayServiceRequest>,
}

impl Executable for APIProductVersionRequest {
    type Response = APIProductVersionResponse;
}

impl APIProductVersionRequest {
    pub fn new(api_product_version: CreateApiProductVersion) -> Self {
        let gsid = api_product_version
            .gateway_service_id
            .unwrap_or("".to_string());
        let cpid = api_product_version
            .control_plane_id
            .unwrap_or("".to_string());
        let ps = match api_product_version.publish_status {
            PublishStatus::Unpublished => "unpublished",
            PublishStatus::Published => "published",
        };
        let d = match api_product_version.deprecated {
            Some(dep) => dep == "true",
            None => false,
        };

        let gs = Some(APIProductVersionGatewayServiceRequest {
            id: gsid.clone(),

            control_plane_id: cpid.clone(),
        });
        info!("gs: {:?}", &gs);

        if gsid.is_empty() || cpid.is_empty() {
            APIProductVersionRequest {
                name: api_product_version.name,
                publish_status: ps.to_string(),
                deprecated: d,
                gateway_service: { None },
            }
        } else {
            APIProductVersionRequest {
                name: api_product_version.name,
                publish_status: ps.to_string(),
                deprecated: d,
                gateway_service: gs,
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIProductVersionGatewayServiceRequest {
    pub control_plane_id: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIProductVersionGatewayServiceResponse {
    control_plane_id: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIProductVersionResponse {
    pub id: String,
    pub name: String,
    gateway_service: Option<APIProductVersionGatewayServiceResponse>,
    publish_status: String,
    deprecated: bool,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct APIProductVersion {
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetApiProductVersionRequest {}

impl Executable for GetApiProductVersionRequest {
    type Response = GetApiProductVersionResponse;
}

impl GetApiProductVersionRequest {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllApiProductVersionRequest {}

impl Executable for GetAllApiProductVersionRequest {
    type Response = GetAllApiProductVersionResponse;
}

impl GetAllApiProductVersionRequest {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllApiProductVersionResponse {
    data: Vec<GetApiProductVersionResponse>,
    meta: Meta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetApiProductVersionResponse {
    pub labels: HashMap<String, String>,
    pub id: String,
    pub name: String,
    pub publish_status: String,
    pub deprecated: bool,
    pub auth_strategy_sync_errors: Option<String>,
    pub portals: Vec<Portal>,
    pub created_at: String,
    pub updated_at: String,
    pub gateway_service: Option<GatewayService>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Portal {
    pub portal_id: String,
    pub portal_name: String,
    pub portal_product_version_id: String,
    pub publish_status: String,
    pub deprecated: bool,
    pub application_registration_enabled: bool,
    pub auto_approve_registration: bool,
    pub auth_strategies: Vec<AuthStrategy>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthStrategy {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayService {
    pub id: String,
    pub runtime_group_id: String,
    pub control_plane_id: String,
}
