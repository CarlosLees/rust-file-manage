use anyhow::{anyhow, Result};
use std::path::Path;
use tokio::fs::{create_dir, metadata};
use tracing::{error, info};

const INIT_DIR: &'static str = "/Users/lishaowen/file-manage";
const VIDEO_DIR: &'static str = "/Users/lishaowen/file-manage/video";
const AUDIO_DIR: &'static str = "/Users/lishaowen/file-manage/audio";
const IMAGE_DIR: &'static str = "/Users/lishaowen/file-manage/image";
const DOC_DIR: &'static str = "/Users/lishaowen/file-manage/doc";

pub async fn check_and_init_dir() -> Result<()> {
    //获取初始文件夹路径
    let path = Path::new(INIT_DIR);

    //判断文件夹是否存在
    let metadata_result = metadata(path).await.is_ok();

    if !metadata_result {
        info!("未找到默认文件夹");
        //创建默认文件夹
        if create_dir(path).await.is_ok() {
            create_dir(Path::new(VIDEO_DIR)).await.map_err(|_| {
                error!("创建视频文件夹失败");
                return anyhow!("创建视频文件夹失败");
            })?;
            create_dir(Path::new(AUDIO_DIR)).await.map_err(|_| {
                error!("创建音频文件夹失败");
                return anyhow!("创建音频文件夹失败");
            })?;
            create_dir(Path::new(IMAGE_DIR)).await.map_err(|_| {
                error!("创建图片文件夹失败");
                return anyhow!("创建图片文件夹失败");
            })?;
            create_dir(Path::new(DOC_DIR)).await.map_err(|_| {
                error!("创建文档文件夹失败");
                return anyhow!("创建文档文件夹失败");
            })?;
        }
    }
    Ok(())
}
