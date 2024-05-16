use axum::Router;
use axum::routing::get;

use crate::AppState;
use crate::file_path::file_path_service::{
    current_path_folder, home_page_folders,
};

pub fn file_path_router() -> Router<AppState> {
    let router = Router::new()
        .route("/current_path_folder", get(current_path_folder))
        .route("/home_page_folders", get(home_page_folders));
    router
}
