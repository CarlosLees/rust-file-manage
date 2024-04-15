use axum::extract::{Query, State};
use axum::Json;
use axum::response::IntoResponse;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use lib_entity::file_path;
use lib_entity::file_path::Entity as FilePath;
use lib_utils::result::http_result::HttpResult;

use crate::AppState;
use crate::file_path::file_path_params::CurrentPathParams;

pub async fn current_path_folder(Query(params):Query<CurrentPathParams>,state: State<AppState>)
    -> impl IntoResponse {
    println!("{}", params.current_id);

    if let Ok(file_path_result) = FilePath::find().filter(file_path::Column::ParentId.eq(params
        .current_id)).all(&state.connection).await {
        return Json(HttpResult::ok(file_path_result));
    }
    Json(HttpResult::default())
}