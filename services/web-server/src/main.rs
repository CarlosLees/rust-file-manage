mod system_info;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use lib_utils::file::dir::check_and_init_dir;
use crate::system_info::system_info_router::system_info_router;

const SERVER_PORT: &'static str = "0.0.0.0:8888";

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
    // 3.初始化web服务器
    let app = Router::new().merge(system_info_router())
        .layer(CorsLayer::very_permissive());

    let listener = TcpListener::bind(SERVER_PORT).await.unwrap();
    axum::serve(listener,app).await.unwrap()
}
