use axum::body::Body;
use axum::http::Request;
use axum::Json;
use axum::middleware::Next;
use axum::response::IntoResponse;
use lib_utils::result::http_result::HttpResult;

pub async fn check_hello_world(
    req: Request<Body>,
    next: Next
) -> Result<impl IntoResponse, Json<HttpResult<String>>> {
    // requires the http crate to get the header name
    // println!("{:?}", req.headers().get(CONTENT_TYPE).unwrap());

        // return Err(Json(HttpResult::error(String::from("123"))));

    Ok(next.run(req).await)
}