pub mod dir;

#[cfg(target_os = "macos")]
const INIT_DIR: &'static str = "/Users/lishaowen/file-manage";
#[cfg(target_os = "macos")]
const VIDEO_DIR: &'static str = "/Users/lishaowen/file-manage/video";
#[cfg(target_os = "macos")]
const AUDIO_DIR: &'static str = "/Users/lishaowen/file-manage/audio";
#[cfg(target_os = "macos")]
const IMAGE_DIR: &'static str = "/Users/lishaowen/file-manage/image";
#[cfg(target_os = "macos")]
const DOC_DIR: &'static str = "/Users/lishaowen/file-manage/doc";

#[cfg(target_os = "windows")]
const INIT_DIR: &'static str = "D:\\file-manage";
#[cfg(target_os = "windows")]
const VIDEO_DIR: &'static str = "D:\\file-manage/video";
#[cfg(target_os = "windows")]
const AUDIO_DIR: &'static str = "D:\\file-manage/audio";
#[cfg(target_os = "windows")]
const IMAGE_DIR: &'static str = "D:\\file-manage/image";
#[cfg(target_os = "windows")]
const DOC_DIR: &'static str = "D:\\file-manage/doc";

#[cfg(target_os = "linux")]
const INIT_DIR: &'static str = "/usr/local/file-manage";
#[cfg(target_os = "linux")]
const VIDEO_DIR: &'static str = "/usr/local/file-manage/video";
#[cfg(target_os = "linux")]
const AUDIO_DIR: &'static str = "/usr/local/file-manage/audio";
#[cfg(target_os = "linux")]
const IMAGE_DIR: &'static str = "/usr/local/file-manage/image";
#[cfg(target_os = "linux")]
const DOC_DIR: &'static str = "/usr/local/file-manage/doc";
