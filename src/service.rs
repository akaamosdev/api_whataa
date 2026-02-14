use std::ffi::OsString;
use windows_service::define_windows_service;
use windows_service::service::{
    ServiceControl, ServiceControlAccept, ServiceEventHandle, ServiceState,
};
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};
use windows_service::Error;
use std::io;

const SERVICE_NAME: &str = "MyRustApp";

define_windows_service!(ffi_service_main, my_service_main);

pub fn my_service_main(_arguments: Vec<OsString>) {
    if let Err(e) = run_service() {
        eprintln!("Service error: {}", e);
    }
}

fn run_service() -> Result<(), Error> {
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel::<()>(1);

    let service_event_handle = ServiceEventHandle::new(SERVICE_NAME, move |control_event| {
        match control_event {
            ServiceControl::Stop | ServiceControl::Shutdown => {
                let _ = shutdown_tx.try_send(());
                ServiceControlAccept::STOP
            }
            ServiceControl::Interrogate => ServiceControlAccept::empty(),
            _ => ServiceControlAccept::empty(),
        }
    })?;

    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| Error::Winapi(io::Error::new(io::ErrorKind::Other, e)))?;

    rt.block_on(async {
        match run_app(&mut shutdown_rx).await {
            Ok(_) => {
                let _ = service_event_handle.report_status(
                    ServiceState::Stopped,
                    ServiceControlAccept::empty(),
                );
            }
            Err(e) => {
                eprintln!("App error: {}", e);
                let _ = service_event_handle.report_status(
                    ServiceState::Stopped,
                    ServiceControlAccept::empty(),
                );
            }
        }
    });

    Ok(())
}

async fn run_app(shutdown_rx: &mut tokio::sync::mpsc::Receiver<()>) -> Result<(), Box<dyn std::error::Error>> {
    use crate::{config::Config, db::init_db, routes::create_router};
    use std::net::SocketAddr;

    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    let pool = init_db(&config.database_url).await;
    let app = create_router(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("🚀 API disponible sur http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    let server = axum::serve(listener, app);

    tokio::select! {
        result = server => {
            result?;
        }
        _ = shutdown_rx.recv() => {
            println!("Service shutdown requested");
        }
    }

    Ok(())
}

pub fn install_service() -> Result<(), Error> {
    let manager_access = ServiceManagerAccess::all();
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service_binary_path = std::env::current_exe()
        .map_err(|e| Error::Winapi(io::Error::from(e)))?;

    let service_info = windows_service::service_manager::ServiceInfo {
        name: OsString::from(SERVICE_NAME),
        display_name: OsString::from("My Rust App Service"),
        service_type: windows_service::service_manager::ServiceType::OwnProcess,
        start_type: windows_service::service_manager::ServiceStartType::AutoStart,
        error_control: windows_service::service_manager::ServiceErrorControl::Normal,
        executable_path: service_binary_path,
        launch_arguments: vec![OsString::from("--service")],
        dependencies: vec![],
        account_name: None,
        account_password: None,
    };

    service_manager.create_service(&service_info, windows_service::service_manager::ServiceAccessRights::all())?;
    println!("Service installed successfully!");
    Ok(())
}

pub fn uninstall_service() -> Result<(), Error> {
    let manager_access = ServiceManagerAccess::all();
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service = service_manager.open_service(
        SERVICE_NAME,
        windows_service::service_manager::ServiceAccessRights::all(),
    )?;

    let service_status = service.query_status()?;
    if service_status.current_state != ServiceState::Stopped {
        service.stop()?;
    }

    service.delete()?;
    println!("Service uninstalled successfully!");
    Ok(())
}