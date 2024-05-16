use crate::system_info::system_info_service::{system_device_info, system_hardware_info};
use axum::routing::get;
use axum::Router;

pub fn system_info_router() -> Router {
    let router = Router::new()
        .route("/", get(system_device_info))
        .route("/info", get(system_hardware_info));
    router
}
