mod media;
mod system_info;

use crate::media::media_router::media_router;
use crate::system_info::system_info_router::system_info_router;
use axum::Router;
use dotenv::dotenv;
use lib_utils::file::dir::check_and_init_dir;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use std::env;
use std::sync::Arc;
use axum::handler::Handler;
use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use migration::{Migrator, MigratorTrait};

const SERVER_PORT: &'static str = "0.0.0.0:8888";

#[derive(Debug)]
pub(crate) struct AppState {
    pub connection: DatabaseConnection
}

#[tokio::main]
async fn main() {
    // 1.初始化日志
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
    // 2.初始化文件系统
    // 1.视频
    // 2.音频
    // 3.图片
    // 4.文档

    // 支持Linux系统
    //检查文件夹初始是否存在，不存在则创建默认文件夹 Linux默认目录为 /usr/local/file-manage
    check_and_init_dir().await.unwrap();

    // 3.配置数据库连接
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("读取数据库配置失败");
    let connection = Database::connect(database_url).await.unwrap();
    Migrator::up(&connection,None).await.unwrap();

    // 4.初始化web服务器
    let app_state = Arc::new(AppState { connection });

    let app = Router::new()
        .with_state(app_state)
        .merge(system_info_router())
        .merge(media_router())
        .layer(CorsLayer::very_permissive());

    let listener = TcpListener::bind(SERVER_PORT).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
