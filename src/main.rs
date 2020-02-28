#[cfg(any(target_os = "linux"))]
#[path = "daemon_linux.rs"]
mod service;

#[cfg(target_os = "windows")]
#[path = "service_windows.rs"]
mod service;

fn main() -> anyhow::Result<()> {
    service::run()
}
