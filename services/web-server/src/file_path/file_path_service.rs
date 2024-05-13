use axum::extract::{Query, State};
use axum::Json;
use axum::response::IntoResponse;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tokio::fs;

use lib_entity::file_path;
use lib_entity::file_path::{Entity as FilePath, Model};
use lib_utils::file::VIDEO_DIR;
use lib_utils::result::http_result::HttpResult;

use crate::AppState;
use crate::file_path::file_path_params::{CurrentPathParams, HomeFolderFromPathParam, PathType};
use crate::file_path::file_path_response::HomeFolderFromPathResponse;

// 获取首页全部文件夹
pub async fn home_page_folders(state: State<AppState>) -> Json<HttpResult<Vec<Model>>> {
    if let Ok(Some(result)) = FilePath::find().filter(file_path::Column::ParentId.is_null()).one(&state
        .connection)
        .await {
        // 获取子文件夹

        if let Ok(data) = FilePath::find().filter(file_path::Column::ParentId.eq(result.id))
            .all
        (&state.connection)
            .await {
            // 检查首页文件夹的内容

            return Json(HttpResult::ok(data));
        }
    }
    return Json(HttpResult::ok(vec![]))
}

//检查对应路径下的文件夹内容
async fn check_path_file(path: String) {
    let mut dir = fs::read_dir(path).await.unwrap();
    while let Some(entity) = dir.next_entry().await.unwrap() {
        let path_file_name:String = entity.path().into_os_string().to_string_lossy().to_string();

        let metadata = entity.metadata().await.unwrap();
        if metadata.is_file() {

        }else if metadata.is_dir() {

        }else {

        }
    }
}

pub async fn current_path_folder(Query(params):Query<CurrentPathParams>,state: State<AppState>)
    -> impl IntoResponse {
    println!("{}", params.current_id);

    if let Ok(file_path_result) = FilePath::find().filter(file_path::Column::ParentId.eq(params
        .current_id)).all(&state.connection).await {
        return Json(HttpResult::ok(file_path_result));
    }
    Json(HttpResult::default())
}

pub async fn home_folder_from_path(Query(param): Query<HomeFolderFromPathParam>) ->
                                                                         Json<HttpResult<Vec<HomeFolderFromPathResponse>>>{
    // 获取路径
    let path = joint_path(param.path_type);
    let mut dir = fs::read_dir(path).await.unwrap();

    let mut result = vec![];
    while let Some(entity) = dir.next_entry().await.unwrap() {
        let path_file_name:String = entity.path().into_os_string().to_string_lossy().to_string();

        let metadata = entity.metadata().await.unwrap();
        if metadata.is_file() {
            result.push(HomeFolderFromPathResponse::new(1,path_file_name));
        }else if metadata.is_dir() {
            result.push(HomeFolderFromPathResponse::new(2,path_file_name));
        }else {
            result.push(HomeFolderFromPathResponse::new(3,path_file_name))
        }
    }

    Json(HttpResult::ok(result))
}

fn joint_path(path_type: u32) -> String {
    let path_type:PathType = path_type.into();
    return match path_type {
        PathType::Video => {
            VIDEO_DIR.to_string()
        }
        PathType::Audio => {
            VIDEO_DIR.to_string()
        }
        PathType::Image => {
            VIDEO_DIR.to_string()
        }
        PathType::Doc => {
            VIDEO_DIR.to_string()
        }
        PathType::NotNull => {
            VIDEO_DIR.to_string()
        }
        _ => {
            String::from("error")
        }
    }
}