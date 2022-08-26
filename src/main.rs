#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

mod api;
mod constants;
mod errors;
mod middleware;
mod model;
mod schema;
mod service;
mod utils;

use crate::api::user as user_api;
use axum_casbin_auth::casbin::MgmtApi;
use std::env;

use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};

use axum_casbin_auth::casbin::{function_map::key_match2, CoreApi, DefaultModel, Result};
use axum_casbin_auth::CasbinAxumLayer;
use diesel_adapter::DieselAdapter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect("Failed to read .env file, please add it");
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST must be set.");
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool_size: u32 = std::env::var("POOL_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);

    let pool = model::db::migrate_and_config_db(&database_url, pool_size);
    let model = DefaultModel::from_file("rbac_model.conf").await.unwrap();
    let adapter = DieselAdapter::new(database_url, pool_size)?;
    let mut casbin_middleware = CasbinAxumLayer::new(model, adapter).await.unwrap();

    casbin_middleware
        .write()
        .await
        .get_role_manager()
        .write()
        .matching_fn(Some(key_match2), None);

    let share_enforcer = casbin_middleware.get_enforcer();
    let clone_enforcer = share_enforcer.clone();

    Ok(())
}

// setup database
// setup model
