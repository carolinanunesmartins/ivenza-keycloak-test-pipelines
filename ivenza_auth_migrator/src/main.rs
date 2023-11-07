mod data;
mod models;
mod services;

use dotenv::dotenv;

use crate::services::{
    ImportValidator, PermissionSyncer, PolicySyncer, ResourceSyncer, RoleSyncer, ScopeSyncer, UserSyncer
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize environment variables.
    dotenv().ok();
    let ivenza_client = services::IvenzaClient::new().await;
    RoleSyncer::sync(&ivenza_client).await?;
    ScopeSyncer::sync(&ivenza_client).await?;
    PolicySyncer::sync(&ivenza_client).await?;
    ResourceSyncer::sync(&ivenza_client).await?;
    PermissionSyncer::sync(&ivenza_client).await?;
    UserSyncer::sync(&ivenza_client).await?;
    ImportValidator::validate(&ivenza_client).await?;
    Ok(())
}
