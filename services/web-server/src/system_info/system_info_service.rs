use crate::system_info::system_info_response::{
    DiskInfoBuilder, SysInfoResponse, SysInfoResponseBuilder, SystemInfoResponse,
    SystemInfoResponseBuilder, TemperatureBuilder,
};
use axum::Json;
use lib_utils::result::http_result::HttpResult;
use sysinfo::{Components, Disks, System};
use whoami::fallible;

pub async fn system_device_info() -> Json<HttpResult<SystemInfoResponse>> {
    let response = SystemInfoResponseBuilder::default()
        .real_name(whoami::realname())
        .username(whoami::username())
        .device_name(whoami::devicename())
        .hostname(fallible::hostname().unwrap())
        .platform(whoami::platform().to_string())
        .distro(whoami::distro())
        .arch(whoami::arch().to_string())
        .desktop_env(whoami::desktop_env().to_string())
        .build()
        .unwrap();

    Json(HttpResult::ok(response))
}

pub async fn system_hardware_info() -> Json<HttpResult<SysInfoResponse>> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let disks = Disks::new_with_refreshed_list();
    let mut disk_vec = vec![];
    for disk in &disks {
        let disk_info = DiskInfoBuilder::default()
            .name(disk.name().to_str().unwrap().to_string())
            .available_space(disk.available_space() / 1024 / 1024 / 1024)
            .total_space(disk.total_space() / 1024 / 1024 / 1024)
            .file_system(disk.file_system().to_str().unwrap().to_string())
            .kind(disk.kind().to_string())
            .mount_point(disk.mount_point().to_string_lossy())
            .build()
            .unwrap();
        disk_vec.push(disk_info);
    }

    // Components temperature:
    let components = Components::new_with_refreshed_list();
    let mut temperature_vec = vec![];

    for component in &components {
        let temperature = TemperatureBuilder::default()
            .temperature(component.temperature())
            .max_temperature(component.max())
            .label(component.label())
            .build()
            .unwrap();
        temperature_vec.push(temperature);
    }

    let response = SysInfoResponseBuilder::default()
        .total_memory(sys.total_memory() / 1024 / 1024 / 1024)
        .used_memory(sys.used_memory() / 1024 / 1024 / 1024)
        .total_swap(sys.total_swap() / 1024 / 1024 / 1024)
        .used_swap(sys.used_swap() / 1024 / 1024 / 1024)
        .system_name(System::name().unwrap())
        .cpu_length(sys.cpus().len())
        .disks(disk_vec)
        .temperatures(temperature_vec)
        .build()
        .unwrap();

    // Display processes ID, name na disk usage:
    //     for (pid, process) in sys.processes() {
    //         println!("[{pid}] {} {:?}", process.name(), process.disk_usage());
    //     }

    Json(HttpResult::ok(response))
}
