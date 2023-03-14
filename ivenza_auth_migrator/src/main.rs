#[macro_use]
extern crate diesel;

mod data;
mod models;
mod schema;
mod services;

use dotenv::dotenv;
use services::UserSyncer;

use crate::services::{
    ImportValidator, PermissionSyncer, PolicySyncer, ResourceSyncer, RoleSyncer, ScopeSyncer,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize environment variables.
    dotenv().ok();
    RoleSyncer::sync().await?;
    ScopeSyncer::sync().await?;
    PolicySyncer::sync().await?;
    ResourceSyncer::sync().await?;
    PermissionSyncer::sync().await?;
    ImportValidator::validate().await?;
    UserSyncer::sync().await?;
    Ok(())
}
