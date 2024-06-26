use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::Json;
use lib_utils::result::http_result::HttpResult;

pub async fn check_hello_world(
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, Json<HttpResult<String>>> {
    // requires the http crate to get the header name
    // info!("{:?}", req.headers().get(CONTENT_TYPE));

    // return Err(Json(HttpResult::error(String::from("123"))));

    Ok(next.run(req).await)
}
