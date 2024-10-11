use api::{
    api_product_version_spec::{
        DeleteAPIProductVersionSpecRequest, GetAPIProductVersionSpecRequest,
        GetAllAPIProductVersionSpecRequest, PatchAPIProductVersionSpecRequest,
    },
    api_product_versions::DeleteAPIProductVersionRequest,
    api_products::{
        DeleteAPIProductRequest, GetAPIProductRequest, GetAPIProductsArrayRequest,
        PatchAPIProductRequest,
    },
    api_utils::Executable,
};
use clap::Parser;
use cli::main_cli;
use serde::Serialize;
use serde_json::Value;
use utils::konnect_utils::{self, FetchFilter};

use crate::api::api_utils::ToJson;

mod cli;

mod api;
mod utils;
use log::{error, info};

enum Operation {
    Fetch(FetchFilter),
    Delete,
    Create,
    Patch(Value),
}

fn handle_request<T>(request: &T, path: &str, operation: Operation)
where
    T: Executable,
    T::Response: ToJson + Serialize,
{
    match operation {
        Operation::Delete => {
            let response = request.delete(path);
            match response {
                Ok(res) => {
                    info!("{:#?}", res);
                    println!("{:#?}", res);
                }
                Err(err) => error!("Error processing request for {} : {}", path, err),
            }
        }
        _ => {
            let response = match operation {
                Operation::Fetch(filter) => request.fetch(path, filter),
                Operation::Delete => unreachable!(),
                Operation::Create => request.create(path),
                Operation::Patch(value) => request.patch(path, value),
            };
            match response {
                Ok(res) => {
                    info!("{}", res.to_json());
                    println!("{}", res.to_json());
                }
                Err(err) => error!("Error processing request for {} : {}", path, err),
            }
        }
    }
}

fn main() {
    env_logger::init();
    let args = cli::main_cli::KonnectCLIArgs::parse();
    match args.entity_type {
        cli::main_cli::EntityType::Get(get_command) => match get_command.command {
            cli::main_cli::GetSubCommand::ApiProduct(apiproduct) => {
                // let api_product = APIProduct::new();
                let ap = GetAPIProductRequest::new(apiproduct);
                let mut filter = utils::konnect_utils::FetchFilter::None;

                let name = &ap.name;
                let id = &ap.id;
                let mut path = "/api-products".to_string();
                if !&name.is_empty() {
                    let ap = GetAPIProductsArrayRequest {};
                    filter = FetchFilter::FieldName("name".to_string(), name.to_string());
                    // handle_fetch(&ap, &path, filter);
                    handle_request(&ap, &path, Operation::Fetch(filter));
                } else if !id.is_empty() {
                    path = format!("{}/{}", path, id);
                    handle_request(&ap, &path, Operation::Fetch(filter));
                } else {
                    let ap = GetAPIProductsArrayRequest {};
                    handle_request(&ap, &path, Operation::Fetch(filter));
                }
                info!("path: {}", &path);
            }
            main_cli::GetSubCommand::ApiProductVersionSpecification(get_spec) => {
                let id = &get_spec.id;
                let pid = &get_spec.product_id;
                let vid = &get_spec.product_version_id;
                match id {
                    Some(i) => {
                        let spv = GetAPIProductVersionSpecRequest::new();

                        let path = format!(
                            "/api-products/{}/product-versions/{}/specifications/{}",
                            pid, vid, i
                        );
                        handle_request(
                            &spv,
                            &path,
                            Operation::Fetch(utils::konnect_utils::FetchFilter::None),
                        );
                    }
                    None => {
                        let spv = GetAllAPIProductVersionSpecRequest::new();
                        let path = format!(
                            "/api-products/{}/product-versions/{}/specifications",
                            pid, vid
                        );
                        handle_request(
                            &spv,
                            &path,
                            Operation::Fetch(utils::konnect_utils::FetchFilter::None),
                        );
                    }
                }
            }
            main_cli::GetSubCommand::ApiProductVersion(get_version) => {
                let id = &get_version.id;
                let pid = &get_version.product_id;
                match id {
                    Some(vid) => {
                        let pv = api::api_product_versions::GetApiProductVersionRequest::new();
                        let path = format!("/api-products/{}/product-versions/{}", pid, vid);
                        handle_request(
                            &pv,
                            &path,
                            Operation::Fetch(utils::konnect_utils::FetchFilter::None),
                        )
                    }
                    None => {
                        let pv = api::api_product_versions::GetAllApiProductVersionRequest::new();
                        let path = format!("/api-products/{}/product-versions", pid);
                        handle_request(
                            &pv,
                            &path,
                            Operation::Fetch(utils::konnect_utils::FetchFilter::None),
                        )
                    }
                }
            }
        },
        cli::main_cli::EntityType::Create(create_command) => {
            match create_command.command {
                cli::main_cli::CreateSubCommand::ApiProduct(apiproduct) => {
                    /*                     println!("apiproduc: {:?}", apiproduct); */
                    let apr = api::api_products::APIProductRequest::new(apiproduct);
                    handle_request(&apr, "/api-products", Operation::Create);
                }
                cli::main_cli::CreateSubCommand::ApiProductVersion(api_product_version) => {
                    let pid = &api_product_version.product_id;
                    let path = format!("/api-products/{}/product-versions", pid);
                    let apvr = api::api_product_versions::APIProductVersionRequest::new(
                        api_product_version,
                    );
                    handle_request(&apvr, &path, Operation::Create);
                }
                main_cli::CreateSubCommand::ApiProductVersionSpecification(spec_version) => {
                    let pid = &spec_version.product_id;
                    let vid = &spec_version.product_version_id;
                    let path = format!(
                        "/api-products/{}/product-versions/{}/specifications",
                        pid, vid
                    );
                    let svr =
                        api::api_product_version_spec::CreateAPIProductVersionSpecRequest::new(
                            spec_version,
                        );
                    handle_request(&svr, &path, Operation::Create);
                }
            }
        }
        cli::main_cli::EntityType::Delete(delete_product_command) => {
            match delete_product_command.command {
                main_cli::DeleteSubCommand::ApiProduct(delprodcli) => {
                    let dpr = DeleteAPIProductRequest::new(delprodcli);
                    let path = format!("/api-products/{}", dpr.id);
                    handle_request(&dpr, &path, Operation::Delete);
                }
                main_cli::DeleteSubCommand::ApiProductVersion(depprodvercli) => {
                    let dprv = DeleteAPIProductVersionRequest::new(depprodvercli);
                    let path = format!(
                        "/api-products/{}/product-versions/{}",
                        dprv.product_id, dprv.id
                    );
                    handle_request(&dprv, &path, Operation::Delete);
                }
                main_cli::DeleteSubCommand::ApiProductVersionSpecification(speccli) => {
                    let dprvs = DeleteAPIProductVersionSpecRequest::new(&speccli);
                    let path = format!(
                        "/api-products/{}/product-versions/{}/specifications/{}",
                        &speccli.product_id, speccli.product_version_id, speccli.id
                    );
                    handle_request(&dprvs, &path, Operation::Delete);
                }
            }
        }
        cli::main_cli::EntityType::Patch(patch_command) => match patch_command.command {
            main_cli::PatchSubCommand::ApiProduct(patchproduct) => {
                let ppr = PatchAPIProductRequest::new(&patchproduct);
                let s_json =
                    konnect_utils::filter_empty_fields(serde_json::to_value(&ppr).unwrap());
                let path = format!("/api-products/{}", &patchproduct.id);
                // let response = ppr.patch(&path, s_json);
                handle_request(&ppr, &path, Operation::Patch(s_json));
            }
            main_cli::PatchSubCommand::ApiProductVersionSpecification(patchspec) => {
                let spr = PatchAPIProductVersionSpecRequest::new(&patchspec);
                let sj = serde_json::to_value(&spr).unwrap();
                let pid = &patchspec.product_id;
                let vid = &patchspec.product_version_id;
                let id = &patchspec.id;
                let path = format!(
                    "/api-products/{}/product-versions/{}/specifications/{}",
                    &pid, &vid, &id
                );
                handle_request(&spr, &path, Operation::Patch(sj));
            }
        },
    }
}
