extern crate fiscalidade_server;

fn main() -> anyhow::Result<()> {
    fiscalidade_server::rocket()?.launch();
    Ok(())
}
