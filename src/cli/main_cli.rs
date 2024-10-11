use clap::{Args, Parser, Subcommand};

use super::{
    api_product_version_spec::{
        CreateApiProductVersionSpec, DeleteApiProductVersionSpec, GetApiProductVersionSpec,
        PatchApiProductVersionSpec,
    },
    api_product_versions_cli::{
        CreateApiProductVersion, DeleteAPIProductVersion, GetAPIProductVersion,
    },
    api_products_cli::{CreateApiProduct, DeleteApiProduct, GetApiProduct, PatchApiProduct},
};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "konnectctl")]
#[command(author, version, about = "Konnect CLI")]
pub struct KonnectCLIArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Get Konnect objects
    ///
    /// try konnectctl get --help
    Get(GetCommand),

    /// Create Konnect objects
    ///
    /// try konnectctl create --help
    Create(CreateCommand),

    /// Delete Konnect objects
    ///
    /// try konnectctl delete --help
    Delete(DeleteCommand),

    /// Patch Konnect objects
    ///
    /// try konnectctl patch --help
    Patch(PatchCommand),
}

#[derive(Debug, Args)]
pub struct PatchCommand {
    #[clap(subcommand)]
    pub command: PatchSubCommand,
}

#[derive(Debug, Args)]
pub struct DeleteCommand {
    #[clap(subcommand)]
    pub command: DeleteSubCommand,
}

#[derive(Debug, Args)]
pub struct GetCommand {
    #[clap(subcommand)]
    pub command: GetSubCommand,
}

#[derive(Debug, Args)]
pub struct CreateCommand {
    #[clap(subcommand)]
    pub command: CreateSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum PatchSubCommand {
    /// Patch an API product
    ///
    /// e.g usage
    ///
    /// konnectctl patch api-product -i <api-product-id> -n "new name"
    ApiProduct(PatchApiProduct),

    /// Patch product version specification
    ///
    /// e.g usage
    ///
    /// konnectctl patch api-product-versopn-specification -p <api-product-id> -v <api-version-id>
    /// -i <api-product-versopn-specificationid>
    ApiProductVersionSpecification(PatchApiProductVersionSpec),
}

#[derive(Debug, Subcommand)]
pub enum DeleteSubCommand {
    /// Delete an API product
    ///
    /// e.g usage
    ///
    /// konnectctl delete api-product -i <api-product-id>
    ApiProduct(DeleteApiProduct),

    /// Delete an API product version
    ///
    /// e.g usage
    ///
    /// konnectctl delete api-product-version -p <api-product-id> -i <api-product-version-id>
    ApiProductVersion(DeleteAPIProductVersion),

    /// Delete a API production version specification
    ///
    /// e.g usage
    ///
    /// konnectctl delete api-product-versopn-specification -p <api-product-id> -v
    /// <api-product-version-id> -i <api-product-versopn-specification-d>
    ApiProductVersionSpecification(DeleteApiProductVersionSpec),
}

#[derive(Debug, Subcommand)]
pub enum GetSubCommand {
    /// Get API product(s). This will return a list of products or a product depending on whether a
    /// product id is passed or not.
    ///
    /// Additionally pass a name if you want to filter by name of API Product.
    ///
    /// e.g usage
    ///
    /// To get a list of API products
    ///
    /// konnectctl get api-product | jq '.data[0]id'
    ///
    /// To get a specific product
    ///
    /// konnectctl get api-product --id <api-product-id> | jq '.id'
    ///
    /// To get a product by name
    ///
    /// konnectctl get api-product --name "abd" | jq '.data[0].id'
    ApiProduct(GetApiProduct),

    /// Get version(s) for a given API product.
    ///
    /// To get a version supply both the product id and version id
    ///
    /// Provide only the product id to get a list of all versions for the product
    /// .
    /// e.g usage
    ///
    /// To get a list of all api product verion of a product
    ///
    /// konnectctl get api-product-version -p <api-product-id> | jq '.data[0]id'
    ///
    /// To get a specific version
    ///
    /// konnectctl get api-product-version -p <api-product-id>--i <api-product-version-id> | jq '.id'
    ///
    /// konnectctl get api-product --name "abd" | jq '.data[0].id'
    ApiProductVersion(GetAPIProductVersion),

    /// Get specification(s) for a given API product and version. This will return a list of specs
    /// or a spec depending on whether a spec id is passed or not.
    ///
    /// e.g usage
    ///
    /// To get a spec for a api product verion
    ///
    /// konnectctl get api-product-version-specification -p <api-product-id> -v
    /// <api-product-version-id> -i <api-product-versopn-specification-id> | jq '.data[0]id'
    ///
    ApiProductVersionSpecification(GetApiProductVersionSpec),
}

#[derive(Debug, Subcommand)]
pub enum CreateSubCommand {
    /// Create an API Product
    ///
    /// e.g. usage
    ///
    /// Create API product with name nd description
    ///
    /// konnectctl create api-product -n <NAME> -d <DESCRIPTION>
    ///
    /// Adding labels
    ///
    /// konnectctl create api-product -n <NAME> -d <DESCRIPTION> -l "key1:valu1,key2:value2"
    ///
    /// Adding portal ids
    ///
    /// konnectctl create api-product -n <NAME> -d <DESCRIPTION> -p "portalid-1,portalid-2"
    ApiProduct(CreateApiProduct),

    /// Create an API Product version
    ///
    /// e.g. usage
    ///
    /// Create API product version with name
    ///
    /// konnectctl create api-product-version -p <api-product-id> -n <NAME>
    ///
    /// Adding gateway service
    ///
    /// konnectctl create api-product-version -p <api-product-id> -n <NAME> -g <gateway-service-id> -c
    /// <control-plane-id>
    ApiProductVersion(CreateApiProductVersion),

    /// Add a specification file to a product version
    ApiProductVersionSpecification(CreateApiProductVersionSpec),
}
