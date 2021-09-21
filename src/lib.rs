#[macro_use]
extern crate serde;

use axum::{handler::get, routing::BoxRoute, Router};

mod handlers;
mod version;

pub fn app() -> Router<BoxRoute> {
    Router::new()
        .route("/version", get(handlers::version))
        .boxed()
}
