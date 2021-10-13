#[macro_use]
extern crate serde;

use std::result;

use axum::{
    handler::{get, post},
    routing::BoxRoute,
    AddExtensionLayer, Router,
};
use errors::Errors;
use tower::ServiceBuilder;

use config::Configuration;

pub mod args;
pub mod config;

mod consts;
mod errors;
mod handlers;
mod hashes;
mod home;
mod json;
mod options;
mod response;
mod version;

pub type Result<T> = result::Result<T, Errors>;

pub fn app(config: Configuration) -> crate::Result<Router<BoxRoute>> {
    let middleware_stack = ServiceBuilder::new()
        .layer(AddExtensionLayer::new(config.deployments()?))
        .into_inner();
    Ok(Router::new()
        .route("/version", get(handlers::version))
        .route("/status_servico", post(handlers::dfe::status_servico))
        .route("/consultar_xml", post(handlers::dfe::consultar_xml))
        .layer(middleware_stack)
        .boxed())
}
