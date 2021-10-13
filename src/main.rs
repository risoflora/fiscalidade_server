use std::{net::SocketAddr, process::exit};

use axum::Server;
use fiscalidade_server::{app, args, config::Configuration};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let opts = args::Args::parse_from_env()?;
    if let Some(help) = opts.help.or(opts.version) {
        println!("{}", help);
        exit(0);
    }
    let config = Configuration::from_file(opts.config_file.unwrap_or_default())?;
    let addr = SocketAddr::from((config.server.host, config.server.port));
    Server::bind(&addr)
        .serve(app(config)?.into_make_service())
        .await?;
    Ok(())
}
