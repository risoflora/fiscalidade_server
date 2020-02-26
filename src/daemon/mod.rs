#[cfg(any(target_os = "linux", target_os = "macos"))]
#[path = "linux.rs"]
mod os;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod os;

pub fn install() {
    os::install()
}

pub fn uninstall() {
    os::uninstall()
}
