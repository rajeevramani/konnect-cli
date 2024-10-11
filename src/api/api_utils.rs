use std::{collections::HashMap, error::Error, path::Path};

use log::info;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::utils::konnect_utils::{FetchFilter, KonnectClient};
use base64::{engine::general_purpose, Engine as _};

pub fn string_to_base64(input: String) -> String {
    general_purpose::STANDARD.encode(input)
}

pub fn get_filename_from_path(file_path: &str) -> Option<&str> {
    Path::new(file_path).file_name()?.to_str()
}

/// function will receive a string in format "key1:value1,key2:value2 return the HashMap"
pub fn string_to_hashmap(string: String) -> HashMap<String, String> {
    let mut hs: HashMap<String, String> = HashMap::new();
    let s: Vec<&str> = string.split(',').collect();
    for kv in s {
        let kva: Vec<&str> = kv.split(':').collect();
        // println!("{:?}",kva);
        hs.insert(kva[0].to_string(), kva[1].to_string());
    }
    hs
}
pub trait ToJson {
    fn to_json(&self) -> String;
}

impl<T: Serialize> ToJson for T {
    fn to_json(&self) -> String {
        let j = serde_json::to_string_pretty(self)
            .unwrap_or_else(|_| String::from("Error: Serialization failed"));
        info!("ToJson: {}", j);
        j
    }
}

pub trait Executable: Serialize {
    type Response: DeserializeOwned;

    fn create(&self, path: &str) -> Result<Self::Response, Box<dyn Error>> {
        let kc = KonnectClient::new();
        let response = kc.create(path, self)?;
        let parsed_response: Self::Response = serde_json::from_str(&response)?;
        Ok(parsed_response)
    }
    fn patch(&self, path: &str, value: Value) -> Result<Self::Response, Box<dyn Error>> {
        let kc = KonnectClient::new();
        let response = kc.patch(path, value)?;
        let parsed_response: Self::Response = serde_json::from_str(&response)?;
        Ok(parsed_response)
    }

    fn fetch(&self, path: &str, filter: FetchFilter) -> Result<Self::Response, Box<dyn Error>> {
        let kc = KonnectClient::new();
        let response = kc.fetch(path, filter)?;
        info!("parsed_response: {:#?}", response);

        let parsed_response: Self::Response = serde_json::from_str(&response)?;
        Ok(parsed_response)
    }
    fn delete(&self, path: &str) -> Result<String, Box<dyn Error>> {
        let kc = KonnectClient::new();
        let response = kc.delete(path)?;
        info!("{:#?}", response);
        // let parseas_strd_response: Self::Response = &response;
        Ok(response)
    }
}
