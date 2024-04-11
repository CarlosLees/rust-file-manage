use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Default,Builder)]
#[builder(setter(into))]
pub struct SystemInfoResponse {
    pub real_name: String,
    pub username: String,
    pub device_name: String,
    pub hostname: String,
    pub platform: String,
    pub distro: String,
    pub desktop_env: String,
    pub arch: String
}