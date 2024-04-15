use crate::media::media_service::read_video;
use crate::AppState;
use axum::routing::get;
use axum::Router;

pub fn media_router() -> Router<AppState> {
    let router = Router::new().route("/read_video", get(read_video));
    router
}
