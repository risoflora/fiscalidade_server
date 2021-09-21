use std::net::SocketAddr;

use fiscalidade_server::app;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}
