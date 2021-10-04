#[macro_use]
extern crate serde;

use axum::{
    handler::{get, post},
    routing::BoxRoute,
    AddExtensionLayer, Router,
};

use config::Configuration;
use tower::ServiceBuilder;

pub mod args;
pub mod config;

mod consts;
mod extractors;
mod handlers;
mod hashes;
mod home;
mod options;
mod response;
mod version;

pub fn app(config: Configuration) -> Router<BoxRoute> {
    let middleware_stack = ServiceBuilder::new()
        .layer(AddExtensionLayer::new(config.deployments()))
        .into_inner();
    Router::new()
        .route("/version", get(handlers::version))
        .route("/status_servico", post(handlers::status_servico))
        .layer(middleware_stack)
        .boxed()
}
