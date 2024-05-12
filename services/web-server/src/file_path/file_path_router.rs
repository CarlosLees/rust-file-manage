use axum::Router;
use axum::routing::get;
use crate::AppState;
use crate::file_path::file_path_service::{current_path_folder, home_folder_from_path};

pub fn file_path_router() -> Router<AppState> {
    let router = Router::new().route("/current_path",get(current_path_folder))
        .route("/folder_from_path",get(home_folder_from_path));
    router
}