use anyhow::{anyhow, Result};
use std::path::Path;
use tokio::fs::{create_dir, metadata};
use tracing::{error, info};
use crate::file::{AUDIO_DIR, DOC_DIR, IMAGE_DIR, INIT_DIR, VIDEO_DIR};

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
    }else {
        info!("初始文件夹已存在");
    }
    Ok(())
}
