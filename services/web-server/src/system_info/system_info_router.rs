use axum::Router;
use axum::routing::get;
use crate::system_info::system_info_service::{sys_info, system_info};

pub fn system_info_router() -> Router {
    let router = Router::new()
        .route("/system",get(system_info))
        .route("/system_info",get(sys_info));
    router
}