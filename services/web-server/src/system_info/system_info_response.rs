use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Builder)]
#[builder(setter(into))]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoResponse {
    pub real_name: String,
    pub username: String,
    pub device_name: String,
    pub hostname: String,
    pub platform: String,
    pub distro: String,
    pub desktop_env: String,
    pub arch: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct SysInfoResponse {
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_name: Option<String>,
    pub cpu_length: usize,
    pub disks: Vec<DiskInfo>,
    pub temperatures: Vec<Temperature>,
}

#[derive(Serialize, Deserialize, Default, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct DiskInfo {
    //硬盘名称
    pub name: String,
    //硬盘可用空间
    pub available_space: u64,
    //硬盘总共空间
    pub total_space: u64,
    //硬盘格式
    pub file_system: String,
    //硬盘类型
    pub kind: String,
    //挂载点
    pub mount_point: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Builder, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct Temperature {
    pub temperature: f32,
    pub max_temperature: f32,
    pub label: String,
}
