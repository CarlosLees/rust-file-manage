use axum::extract::{Query, State};
use axum::Json;
use axum::response::IntoResponse;
use chrono::Local;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::{NotSet, Set};
use tokio::fs;
use urlencoding::decode;

use lib_entity::file_path;
use lib_entity::file_path::{Entity as FilePath, Model};
use lib_utils::file::INIT_DIR;
use lib_utils::result::http_result::HttpResult;

use crate::AppState;
use crate::file_path::file_path_params::CurrentPathParams;
use crate::file_path::file_path_response::{CurrentPathFoldersResponse};

// 获取首页全部文件夹
pub async fn home_page_folders(state: State<AppState>) -> Json<HttpResult<Vec<Model>>> {
    if let Ok(Some(result)) = FilePath::find()
        .filter(file_path::Column::ParentId.is_null())
        .one(&state.connection)
        .await
    {
        println!("{:?}", result);
        // 获取子文件夹
        if let Ok(data) = FilePath::find()
            .filter(file_path::Column::ParentId.eq(result.id))
            .all(&state.connection)
            .await
        {
            // 检查首页文件夹的内容
            check_path_file(
                format!("{}", INIT_DIR.to_owned() + "/"),
                &data,
                result.id,
                &state.connection,
            )
            .await;
            return Json(HttpResult::ok(data));
        }
    }
    return Json(HttpResult::ok(vec![]));
}

pub async fn current_path_folder(
    Query(params): Query<CurrentPathParams>,
    state: State<AppState>,
) -> impl IntoResponse {
    if let Ok(data) = FilePath::find()
        .filter(file_path::Column::ParentId.eq(params.parent_id))
        .all(&state.connection)
        .await
    {
        // 检查文件内容
        // check_path_file(format!("{}",INIT_DIR.to_owned() + "/" + params.intact_path.as_str())
        // ,&data,params.parent_id,&state.connection).await;
        return Json(HttpResult::ok(CurrentPathFoldersResponse::new(data,params.intact_path)));
    }
    Json(HttpResult::default())
}

//检查对应路径下的文件夹内容
async fn check_path_file(
    path: String,
    data: &Vec<Model>,
    parent_id: i32,
    connection: &DatabaseConnection,
) {
    println!("{:?}",decode(path.as_str()).unwrap().to_string());
    let mut dir = fs::read_dir(decode(path.as_str()).unwrap().to_string()).await.unwrap();
    let path_vec: Vec<String> = data.into_iter().map(|x| x.folder_name.clone()).collect();

    let mut insert_vec = vec![];
    while let Some(entity) = dir.next_entry().await.unwrap() {
        let path_file_name: String = entity.path().into_os_string().to_string_lossy().to_string();
        let last = path_file_name.split("/").last().unwrap();

        if !path_vec.contains(&last.to_string()) {
            let metadata = entity.metadata().await.unwrap();
            if metadata.is_file() {
                let file_modal = file_path::ActiveModel {
                    id: NotSet,
                    parent_id: Set(Some(parent_id)),
                    create_time: Set(Local::now().naive_local()),
                    update_time: Set(Local::now().naive_local()),
                    folder_name: Set(last.to_owned()),
                    file_type: Set(1),
                };
                insert_vec.push(file_modal);
            } else if metadata.is_dir() {
                let dir_modal = file_path::ActiveModel {
                    id: NotSet,
                    parent_id: Set(Some(parent_id)),
                    create_time: Set(Local::now().naive_local()),
                    update_time: Set(Local::now().naive_local()),
                    folder_name: Set(last.to_owned()),
                    file_type: Set(0),
                };
                insert_vec.push(dir_modal);
            } else {
            }
        }
    }

    if !insert_vec.is_empty() {
        FilePath::insert_many(insert_vec)
            .exec(connection)
            .await
            .unwrap();
    }
}