#[cfg(target_os = "windows")]
use winres;

#[cfg(target_os = "windows")]
fn main() {
    use std::io::Write;
    let mut res = winres::WindowsResource::new();
    res.set_icon("resources\\ico\\fiscalidade_server.ico");
    match res.compile() {
        Err(e) => {
            write!(std::io::stderr(), "{}", e).unwrap();
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
