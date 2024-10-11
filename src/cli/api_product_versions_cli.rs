use core::str;

use clap::{Args, ValueEnum};

#[derive(Debug, Args)]
pub struct CreateApiProductVersion {
    #[arg(short = 'p', long, required = true)]
    /// Id of the product
    pub product_id: String,

    #[arg(short, long)]
    /// The name of the API product version
    pub name: String,

    #[arg(short = 's', long, value_enum, default_value_t = PublishStatus::Unpublished )]
    /// The publish status of the product
    pub publish_status: PublishStatus,

    #[arg(short, long)]
    /// The deprecated of the API product version
    pub deprecated: Option<String>,

    #[arg(short, long)]
    /// The id of the gateway service associated with API product
    pub gateway_service_id: Option<String>,

    #[arg(short, long)]
    /// The id of the controla plane on which the gateway service is deployed
    pub control_plane_id: Option<String>,
    // #[arg(short, long)]
    // /// The list of comma separate of the API product
    // pub gateway_service: Option<GatewayService>,
}

#[derive(Debug, Args)]
pub struct GetAPIProductVersion {
    #[arg(short = 'p', long, required = true)]
    /// Product id
    pub product_id: String,

    #[arg(short, long)]
    /// Product version id
    pub id: Option<String>,
}

#[derive(Debug, Args)]
pub struct DeleteAPIProductVersion {
    #[arg(short, long, required = true)]
    /// Product version id
    pub id: String,

    #[arg(short, long, required = true)]
    /// Product  id
    pub product_id: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PublishStatus {
    // Staus is unpublished
    Unpublished,

    // Staus is published
    Published,
}
