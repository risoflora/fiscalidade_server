#[cfg(not(debug_assertions))]
#[cfg(any(target_os = "linux"))]
#[path = "daemon_linux.rs"]
mod server;

#[cfg(not(debug_assertions))]
#[cfg(target_os = "windows")]
#[path = "service_windows.rs"]
mod server;

#[cfg(debug_assertions)]
use fiscalidade_server as server;

fn main() -> anyhow::Result<()> {
    server::run()
}
