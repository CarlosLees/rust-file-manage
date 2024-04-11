use axum::Router;
use axum::routing::get;
use crate::system_info::system_info_service::system_info;

pub fn system_info_router() -> Router {
    let router = Router::new()
        .route("/system",get(system_info));
    router
}