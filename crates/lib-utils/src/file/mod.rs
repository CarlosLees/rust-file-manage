pub mod dir;

#[cfg(target_os = "macos")]
pub const INIT_DIR: &'static str = "/Users/lishaowen/file-manage";
#[cfg(target_os = "macos")]
pub const VIDEO_DIR: &'static str = "/Users/lishaowen/file-manage/video";
#[cfg(target_os = "macos")]
pub const AUDIO_DIR: &'static str = "/Users/lishaowen/file-manage/audio";
#[cfg(target_os = "macos")]
pub const IMAGE_DIR: &'static str = "/Users/lishaowen/file-manage/image";
#[cfg(target_os = "macos")]
pub const DOC_DIR: &'static str = "/Users/lishaowen/file-manage/doc";

#[cfg(target_os = "windows")]
pub const INIT_DIR: &'static str = "D:\\file-manage";
#[cfg(target_os = "windows")]
pub const VIDEO_DIR: &'static str = "D:\\file-manage/video";
#[cfg(target_os = "windows")]
pub const AUDIO_DIR: &'static str = "D:\\file-manage/audio";
#[cfg(target_os = "windows")]
pub const IMAGE_DIR: &'static str = "D:\\file-manage/image";
#[cfg(target_os = "windows")]
pub const DOC_DIR: &'static str = "D:\\file-manage/doc";

#[cfg(target_os = "linux")]
pub const INIT_DIR: &'static str = "/usr/local/file-manage";
#[cfg(target_os = "linux")]
pub const VIDEO_DIR: &'static str = "/usr/local/file-manage/video";
#[cfg(target_os = "linux")]
pub const AUDIO_DIR: &'static str = "/usr/local/file-manage/audio";
#[cfg(target_os = "linux")]
pub const IMAGE_DIR: &'static str = "/usr/local/file-manage/image";
#[cfg(target_os = "linux")]
pub const DOC_DIR: &'static str = "/usr/local/file-manage/doc";
