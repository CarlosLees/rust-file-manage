use axum::extract::{Query, State};
use axum::Json;
use axum::response::IntoResponse;
use chrono::Local;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::{Set, NotSet};
use tokio::fs;

use lib_entity::file_path;
use lib_entity::file_path::{Entity as FilePath, Model};
use lib_utils::file::{INIT_DIR, VIDEO_DIR};
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
            check_path_file(format!("{}", INIT_DIR.to_owned() + "/"), &data,result.id,
                            &state.connection).await;

            return Json(HttpResult::ok(data));
        }
    }
    return Json(HttpResult::ok(vec![]))
}

//检查对应路径下的文件夹内容
async fn check_path_file(path: String, data: &Vec<Model>, parent_id: i32, connection: &DatabaseConnection) {
    let mut dir = fs::read_dir(path).await.unwrap();

    let path_vec:Vec<String> = data.into_iter().map(|x| x.folder_name.clone()).collect();

    let mut insert_vec = vec![];
    while let Some(entity) = dir.next_entry().await.unwrap() {
        let path_file_name:String = entity.path().into_os_string().to_string_lossy().to_string();
        let last = path_file_name.split("/").last().unwrap();

        println!("{:?}",last);

        if !path_vec.contains(&last.to_string()) {
            let metadata = entity.metadata().await.unwrap();
            if metadata.is_file() {
                let file_modal = file_path::ActiveModel {
                    id: NotSet,
                    parent_id: Set(Some(parent_id)),
                    create_time: Set(Local::now().naive_local()),
                    update_time: Set(Local::now().naive_local()),
                    folder_name: Set(last.to_owned()),
                    file_type: Set(1)
                };
                insert_vec.push(file_modal);
            }else if metadata.is_dir() {
                let dir_modal = file_path::ActiveModel {
                    id: NotSet,
                    parent_id: Set(Some(parent_id)),
                    create_time: Set(Local::now().naive_local()),
                    update_time: Set(Local::now().naive_local()),
                    folder_name: Set(last.to_owned()),
                    file_type: Set(0)
                };
                insert_vec.push(dir_modal);
            }else {

            }
        }
    }

    FilePath::insert_many(insert_vec).exec(connection).await.unwrap();
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