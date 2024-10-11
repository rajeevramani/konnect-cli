use clap::Args;

#[derive(Debug, Args)]
pub struct CreateApiProductVersionSpec {
    #[arg(short = 'p', long, required = true)]
    /// API product id
    pub product_id: String,

    #[arg(short = 'v', long, required = true)]
    /// API product version id
    pub product_version_id: String,

    #[arg(short = 'f', long, required = true)]
    /// Path to spec file
    pub path_to_spec_file: String,
}

#[derive(Debug, Args)]
pub struct DeleteApiProductVersionSpec {
    #[arg(short = 'p', long, required = true)]
    /// API product id
    pub product_id: String,

    #[arg(short = 'v', long, required = true)]
    /// API product version id
    pub product_version_id: String,

    #[arg(short, long, required = true)]
    /// API specification id
    pub id: String,
}

#[derive(Debug, Args)]
pub struct GetApiProductVersionSpec {
    #[arg(short = 'p', long, required = true)]
    /// API product id
    pub product_id: String,

    #[arg(short = 'v', long, required = true)]
    /// API product version id
    pub product_version_id: String,

    #[arg(short, long)]
    /// API specification id
    pub id: Option<String>,
}

#[derive(Debug, Args)]
pub struct PatchApiProductVersionSpec {
    #[arg(short = 'p', long, required = true)]
    /// API product id
    pub product_id: String,

    #[arg(short = 'v', long, required = true)]
    /// API product id
    pub product_version_id: String,

    #[arg(short, long, required = true)]
    /// API specification id
    pub id: String,

    #[arg(short = 'f', long, required = true)]
    /// Path to spec file
    pub path_to_spec_file: String,
}
