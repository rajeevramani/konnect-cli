use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Meta {
    pub page: Page,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    pub total: i32,
    pub size: i32,
    pub number: i32,
}
