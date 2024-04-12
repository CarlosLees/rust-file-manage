use axum::body::Body;
use axum::extract::Query;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use tokio::fs::{File, metadata};
use tokio_util::codec::{BytesCodec, FramedRead};
use crate::media::media_params::ReadVideoParams;

pub async fn read_video(Query(params):Query<ReadVideoParams>) -> impl IntoResponse {
    let file_path = "/Users/lishaowen/file-manage/video/1.MP4";
    let file = File::open(file_path).await.unwrap();

    let len = metadata(file_path).await.unwrap().len();

    let stream = FramedRead::new(file,BytesCodec::new());
    let body = Body::from_stream(stream);

    // Response::builder()
    //     .status(StatusCode::NOT_FOUND)
    //     .header("x-foo", "custom header")
    //     .body(Body::from("not found"))
    //     .unwrap()
    let response = Response::builder().status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "video/mp4")
        .header(header::CONTENT_LENGTH, len)
        .header(header::ACCEPT_RANGES, "bytes")
        .header(header::CACHE_CONTROL, "no-cache").body(body).unwrap();
    response
}