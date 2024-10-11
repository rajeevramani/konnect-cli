use dotenv::dotenv;
use log::{error, info};
use serde_json::Value;
use std::{env, error::Error};

#[derive(Debug)]
pub struct KonnectClient {
    url: String,
    auth_token: String,
    client: reqwest::blocking::Client,
}

pub enum FetchFilter {
    FieldName(String, String),
    None,
}

pub fn filter_empty_fields(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let filtered_map = map
                .into_iter()
                .filter_map(|(k, v)| {
                    if k == "id" {
                        return None;
                    }
                    match &v {
                        Value::String(s) if s.is_empty() => None,
                        Value::Array(arr) if arr.is_empty() => None,
                        Value::Object(obj) if obj.is_empty() => None,
                        Value::Null => None,
                        _ => Some((k, filter_empty_fields(v))),
                    }
                })
                .collect();
            Value::Object(filtered_map)
        }
        _ => value,
    }
}

impl KonnectClient {
    pub fn new() -> Self {
        dotenv().ok();

        let auth_token = env::var("KONNECT_PAT").expect("KONNECT_PAT must be set");
        let konnect_region = env::var("KONNECT_REGION");
        let region = match konnect_region {
            Ok(r) => {
                if ["us", "au", "eu"].contains(&r.as_str()) {
                    info!("Region: {}", &r);
                    r
                } else {
                    error!(
                        "Invalid region: {}, it can can only 'eu' or 'au', or 'us'",
                        r
                    );
                    std::process::exit(1);
                }
            }
            Err(e) => {
                error!("Cant find env variable KONNECT_REGION: {}, it can can only 'eu' or 'au', or 'us'", e);
                std::process::exit(1)
            }
        };

        let url = format!("https://{}.api.konghq.com/v2", &region);
        let client = reqwest::blocking::Client::new();

        Self {
            url,
            auth_token,
            client,
        }
    }

    pub fn delete(&self, path: &str) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .delete(format!("{}{}", self.url, path))
            .header("authorization", format!("Bearer {}", self.auth_token))
            .send()?;

        if response.status().is_success() {
            let data = response.status().as_str().to_string();
            Ok(data)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                response.text().unwrap(),
            )))
        }
    }

    pub fn fetch(&self, path: &str, filter: FetchFilter) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{}{}", self.url, path))
            .header("authorization", format!("Bearer {}", self.auth_token));

        let response = match filter {
            FetchFilter::FieldName(filter_field_name, field_value) => {
                response.query(&[(&format!("filter[{}]", filter_field_name), field_value)])
            }
            FetchFilter::None => response,
        }
        .send()?;

        info!("Response status: {}", response.status());

        if response.status().is_success() {
            let data = response.text().unwrap();
            Ok(data)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                response.text().unwrap(),
            )))
        }
    }

    pub fn create<T: serde::Serialize>(
        &self,
        path: &str,
        body: T,
    ) -> Result<String, Box<dyn Error>> {
        info!("request body: {}", serde_json::json!(&body));
        let response = self
            .client
            .post(format!("{}{}", self.url, path))
            .header("authorization", format!("Bearer {}", self.auth_token))
            .json(&body)
            .send()?;
        if response.status().is_success() {
            let data = response.text().unwrap();
            return Ok(data);
        }

        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            response.text().unwrap(),
        )))
    }

    pub fn patch<T: serde::Serialize>(
        &self,
        path: &str,
        body: T,
    ) -> Result<String, Box<dyn Error>> {
        info!("request body: {}", serde_json::json!(&body));
        let response = self
            .client
            .patch(format!("{}{}", self.url, path))
            .header("authorization", format!("Bearer {}", self.auth_token))
            .json(&body)
            .send()?;
        if response.status().is_success() {
            let data = response.text().unwrap();
            return Ok(data);
        }

        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            response.text().unwrap(),
        )))
    }
}
