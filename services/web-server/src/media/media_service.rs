use crate::AppState;
use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use tokio::fs::{metadata, File};
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::media::media_params::ReadVideoParams;

pub async fn read_video(
    Query(param): Query<ReadVideoParams>,
    _state: State<AppState>,
) -> impl IntoResponse {
    let file_path = param.file_path.as_str();
    let file = File::open(file_path).await.unwrap();

    let len = metadata(file_path).await.unwrap().len();

    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::from_stream(stream);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "video/mp4")
        .header(header::CONTENT_LENGTH, len)
        .header(header::ACCEPT_RANGES, "bytes")
        .header(header::CACHE_CONTROL, "no-cache")
        .body(body)
        .unwrap();
    response
}
