use std::{
    ffi::OsString,
    result,
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

use log::error;
use thiserror::Error;
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
};

use fiscalidade_server as server;

const SERVICE_NAME: &str = env!("CARGO_PKG_NAME");
const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

#[derive(Error, Debug)]
pub enum WinServiceError {
    #[error(transparent)]
    HttpClient(#[from] windows_service::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub fn run() -> anyhow::Result<()> {
    Ok(service_dispatcher::start(SERVICE_NAME, ffi_service_main)?)
}

define_windows_service!(ffi_service_main, my_service_main);

pub fn my_service_main(_arguments: Vec<OsString>) {
    if let Err(error) = run_service() {
        error!("{}", error);
    }
}

pub fn run_service() -> result::Result<(), WinServiceError> {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            ServiceControl::Stop => {
                let (lock, cvar) = &*pair2;
                let mut terminated = lock.lock().unwrap();
                *terminated = true;
                cvar.notify_one();
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };
    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
    })?;
    thread::spawn(|| {
        if let Err(error) = server::run() {
            error!("{}", error);
        }
    });
    let (lock, cvar) = &*pair;
    let mut terminated = lock.lock().unwrap();
    while !*terminated {
        terminated = cvar.wait(terminated).unwrap();
    }
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
    })?;
    Ok(())
}
