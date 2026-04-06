// use std::ffi::OsString;
// use std::io;
// use std::sync::mpsc;
// use std::time::Duration;
// use windows_service::define_windows_service;
// use windows_service::service::{
//     ServiceControl, ServiceControlAccept, ServiceEventHandle, ServiceStatus, ServiceState,
//     ServiceType,
// };
// use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};
// use windows_service::Error;

// const SERVICE_NAME: &str = "MyRustApp";
// const SERVICE_TYPE: ServiceType = ServiceType::OwnProcess;

// define_windows_service!(ffi_service_main, my_service_main);

// pub fn my_service_main(_arguments: Vec<OsString>) {
//     if let Err(e) = run_service() {
//         eprintln!("Service error: {}", e);
//     }
// }

// fn run_service() -> Result<(), Error> {
//     let (tx, rx) = mpsc::channel();

//     let event_handler = move |control_event| -> ServiceControlAccept {
//         match control_event {
//             ServiceControl::Stop => {
//                 tx.send(()).unwrap();
//                 ServiceControlAccept::Stop
//             }
//             ServiceControl::Interrogate => ServiceControlAccept::None,
//             _ => ServiceControlAccept::None,
//         }
//     };

//     let status_handle = ServiceEventHandle::new(SERVICE_NAME, event_handler)?;

//     let next_status = ServiceStatus {
//         service_type: SERVICE_TYPE,
//         current_state: ServiceState::Running,
//         controls_accepted: ServiceControlAccept::Stop,
//         exit_code: windows_service::service::ServiceExitCode::Win32(0),
//         checkpoint: 0,
//         wait_hint: Duration::default(),
//         process_id: None,
//     };

//     status_handle.set_service_status(next_status)?;

//     // Start the runtime
//     let rt = tokio::runtime::Runtime::new()
//         .map_err(|e| Error::Winapi(io::Error::new(io::ErrorKind::Other, e)))?;

//     // Run the app in a separate task
//     rt.spawn(async {
//         let _ = run_app().await;
//     });

//     // Wait for stop signal
//     let _ = rx.recv();

//     let stop_status = ServiceStatus {
//         service_type: SERVICE_TYPE,
//         current_state: ServiceState::Stopped,
//         controls_accepted: ServiceControlAccept::None,
//         exit_code: windows_service::service::ServiceExitCode::Win32(0),
//         checkpoint: 0,
//         wait_hint: Duration::default(),
//         process_id: None,
//     };

//     status_handle.set_service_status(stop_status)?;

//     rt.block_on(async {
//         tokio::time::sleep(Duration::from_secs(1)).await;
//     });

//     Ok(())
// }

// async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
//     use crate::{config::Config, db::init_db, routes::create_router};
//     use std::net::SocketAddr;

//     let config = Config::from_env();
//     let pool = init_db(&config.database_url).await;
//     let app = create_router(pool);

//     let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
//     println!("🚀 Service running on http://{}", addr);

//     let listener = tokio::net::TcpListener::bind(addr).await?;
//     axum::serve(listener, app).await?;

//     Ok(())
// }

// pub fn install_service() -> Result<(), Error> {
//     let manager_access = ServiceManagerAccess::Connect | ServiceManagerAccess::CreateService;
//     let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

//     let service_binary_path = std::env::current_exe()
//         .map_err(|e| Error::Winapi(io::Error::from(e)))?;

//     let service_info = windows_service::service_manager::ServiceInfo {
//         name: OsString::from(SERVICE_NAME),
//         display_name: OsString::from("My Rust App Service"),
//         service_type: SERVICE_TYPE,
//         start_type: windows_service::service_manager::ServiceStartType::AutoStart,
//         error_control: windows_service::service_manager::ServiceErrorControl::Normal,
//         executable_path: service_binary_path,
//         launch_arguments: vec![OsString::from("--service")],
//         dependencies: vec![],
//         account_name: None,
//         account_password: None,
//     };

//     let _service = service_manager.create_service(&service_info, ServiceType::OwnProcess)?;
//     println!("✅ Service '{}' installed successfully!", SERVICE_NAME);
//     println!("Run 'sc start {}' to start the service", SERVICE_NAME);
//     Ok(())
// }

// pub fn uninstall_service() -> Result<(), Error> {
//     let manager_access = ServiceManagerAccess::Connect;
//     let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

//     let service = service_manager.open_service(SERVICE_NAME, ServiceType::OwnProcess)?;
//     service.delete()?;
//     println!("✅ Service '{}' uninstalled successfully!", SERVICE_NAME);
//     Ok(())
// }