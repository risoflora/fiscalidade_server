#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fiscalidade_server;

fn main() -> anyhow::Result<()> {
    fiscalidade_server::rocket()?.launch();
    Ok(())
}
