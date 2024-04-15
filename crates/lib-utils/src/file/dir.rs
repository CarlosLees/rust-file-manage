use crate::file::{AUDIO_DIR, DOC_DIR, IMAGE_DIR, INIT_DIR, VIDEO_DIR};
use anyhow::{anyhow, Result};
use std::path::Path;
use chrono::Local;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};
use tokio::fs::{create_dir, metadata};
use tracing::{error, info};
use lib_entity::file_path;

pub async fn check_and_init_dir(connection: &DatabaseConnection) -> Result<()> {
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
        //创建数据库默认数据
        let default_path = file_path::ActiveModel {
            id: ActiveValue::NotSet,
            parent_id: ActiveValue::NotSet,
            create_time: ActiveValue::Set(Local::now().naive_local()),
            update_time: ActiveValue::Set(Local::now().naive_local()),
            folder_name: ActiveValue::Set(String::from("file-manage")),
        };
        if let Ok(modal) = file_path::Entity::insert(default_path).exec(connection)
            .await {
            let insert_id = modal.last_insert_id;
            // 添加初始子文件夹
            insert_default_data(connection,String::from("video"),insert_id).await?;
            insert_default_data(connection,String::from("image"),insert_id).await?;
            insert_default_data(connection,String::from("audio"),insert_id).await?;
            insert_default_data(connection,String::from("doc"),insert_id).await?;
        }
    } else {
        info!("初始文件夹已存在");
    }
    Ok(())
}

async fn insert_default_data(connection: &DatabaseConnection,folder_name: String, parent_id: i32)
    -> Result<()> {
    let doc = file_path::ActiveModel {
        id: ActiveValue::NotSet,
        parent_id: ActiveValue::Set(Some(parent_id)),
        create_time: ActiveValue::Set(Local::now().naive_local()),
        update_time: ActiveValue::Set(Local::now().naive_local()),
        folder_name: ActiveValue::Set(folder_name),
    };
    doc.insert(connection).await?;
    Ok(())
}
