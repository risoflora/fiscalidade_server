#[macro_use]
extern crate serde;

use std::result;

use axum::{
    handler::{get, Handler},
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

pub type Result<T> = result::Result<T, Errors>;

pub fn app(config: Configuration) -> crate::Result<Router<BoxRoute>> {
    let middleware_stack = ServiceBuilder::new()
        .layer(AddExtensionLayer::new(config.deployments()?))
        .into_inner();
    Ok(Router::new()
        .route("/versao", get(handlers::version))
        .route("/status_servico", get(handlers::dfe::status_servico))
        .route(
            "/consultar_protocolo/:chave",
            get(handlers::dfe::consultar_protocolo),
        )
        .or(handlers::not_found.into_service())
        .layer(middleware_stack)
        .boxed())
}
