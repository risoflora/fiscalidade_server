#[macro_use]
extern crate serde;

use axum::{handler::get, routing::BoxRoute, Router};

pub mod args;
pub mod config;

mod consts;
mod handlers;
mod home;
mod options;
mod server;
mod version;

pub fn app() -> Router<BoxRoute> {
    Router::new()
        .route("/version", get(handlers::version))
        .boxed()
}
