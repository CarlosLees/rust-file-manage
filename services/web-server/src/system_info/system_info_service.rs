use axum::Json;
use whoami::fallible;
use lib_utils::result::http_result::HttpResult;
use crate::system_info::params_response::system_info_response::{SystemInfoResponse, SystemInfoResponseBuilder};

pub async fn system_info() -> Json<HttpResult<SystemInfoResponse>> {

    let response = SystemInfoResponseBuilder::default()
        .real_name(whoami::realname())
        .username(whoami::username())
        .device_name(whoami::devicename())
        .hostname(fallible::hostname().unwrap())
        .platform(whoami::platform().to_string())
        .distro(whoami::distro())
        .arch(whoami::arch().to_string())
        .desktop_env(whoami::desktop_env().to_string())
        .build().unwrap();

    Json(HttpResult::ok(response))
}