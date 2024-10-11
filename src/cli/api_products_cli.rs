use super::cli_utils;
use clap::Args;

#[derive(Debug, Args)]
pub struct DeleteApiProduct {
    #[arg(short, long)]
    /// The id of the API Product
    pub id: String,
}

#[derive(Debug, Args)]
pub struct GetApiProduct {
    #[arg(short, long)]
    /// The id of the API Product
    pub id: Option<String>,

    #[arg(short, long)]
    /// The name of the API Product
    pub name: Option<String>,
}

#[derive(Debug, Args)]
pub struct CreateApiProduct {
    #[arg(short, long)]
    /// The name of the API product
    pub name: String,

    #[arg(short, long)]
    /// The description of the API product
    pub description: String,

    #[arg(short, long, value_parser=cli_utils::validate_label)]
    /// Provide labels in the formart "key1:value1,key2:value2
    pub labels: Option<String>,

    #[arg(short, long)]
    /// The list of comma separate of the API product
    pub portal_ids: Option<String>,
}

#[derive(Debug, Args)]
pub struct PatchApiProduct {
    #[arg(short, long, required = true)]
    /// ID of the API product to be updated
    pub id: String,

    #[arg(short, long)]
    /// The name of the API product
    pub name: Option<String>,

    #[arg(short, long)]
    /// The description of the API product
    pub description: Option<String>,

    #[arg(short, long, value_parser=cli_utils::validate_label)]
    /// Provide labels in the formart "key1:value1,key2:value2
    pub labels: Option<String>,

    #[arg(short, long)]
    /// The list of comma separate of the API product
    pub portal_ids: Option<String>,
}
