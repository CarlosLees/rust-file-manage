use crate::media::media_service::read_video;
use axum::routing::get;
use axum::Router;

pub fn media_router() -> Router {
    let router = Router::new().route("/read_video", get(read_video));
    router
}
