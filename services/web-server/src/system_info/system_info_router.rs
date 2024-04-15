use crate::system_info::system_info_service::{sys_info, system_info};
use axum::routing::get;
use axum::Router;

pub fn system_info_router() -> Router {
    let router = Router::new()
        .route("/", get(system_info))
        .route("/info", get(sys_info));
    router
}
