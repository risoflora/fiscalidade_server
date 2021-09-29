#[macro_use]
extern crate serde;

use axum::{
    handler::{get, post},
    routing::BoxRoute,
    AddExtensionLayer, Router,
};
use cert::store::CertificateStore;
use tower::ServiceBuilder;

use config::Configuration;

pub mod args;
pub mod cert;
pub mod config;

mod consts;
mod handlers;
mod home;
mod options;
mod server;
mod version;

pub fn app(config: Configuration) -> Router<BoxRoute> {
    let certs_state: CertificateStore = config
        .certificates
        .unwrap_or_default()
        .into_iter()
        .map(|item| (item.document, item.certificate))
        .collect();
    let middleware_stack = ServiceBuilder::new()
        .layer(AddExtensionLayer::new(certs_state))
        .into_inner();
    Router::new()
        .route("/version", get(handlers::version))
        .route("/status_servico", post(handlers::status_servico))
        .layer(middleware_stack)
        .boxed()
}
