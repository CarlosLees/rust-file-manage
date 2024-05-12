mod media;
mod system_info;
mod file_path;

use crate::media::media_router::media_router;
use crate::system_info::system_info_router::system_info_router;
use axum::Router;
use dotenv::dotenv;
use lib_utils::file::dir::check_and_init_dir;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use lib_core::middleware::middleware::check_hello_world;
use crate::file_path::file_path_router::file_path_router;

const SERVER_PORT: &'static str = "0.0.0.0:8888";

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub connection: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    //sea-orm-cli generate entity -u mysql://root:lsw0516@82.156.175.47:3300/file_manage --with-serde both --model-extra-attributes "serde(rename_all = \"camelCase\")" -o crates/lib-entity/src

    // 1.初始化日志
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    // 2.配置数据库连接
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("读取数据库配置失败");
    let connection = Database::connect(database_url).await.unwrap();
    Migrator::up(&connection, None).await.unwrap();

    // 3.初始化文件系统

    // 1.视频
    // 2.音频
    // 3.图片
    // 4.文档

    // 支持Linux系统
    //检查文件夹初始是否存在，不存在则创建默认文件夹 Linux默认目录为 /usr/local/file-manage
    check_and_init_dir(&connection).await.unwrap();

    // 4.初始化web服务器
    let state = AppState { connection };

    //媒体相关
    let media_router = media_router().with_state(state.clone());
    //系统相关
    let system_router = system_info_router();
    //文件路径相关
    let file_path_router = file_path_router().with_state(state.clone());

    let app = Router::new()
        .nest("/media", media_router)
        .nest("/path", file_path_router)
        .nest("/system", system_router)
        .layer(CorsLayer::very_permissive())
        .layer(axum::middleware::from_fn(check_hello_world));

    let listener = TcpListener::bind(SERVER_PORT).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
