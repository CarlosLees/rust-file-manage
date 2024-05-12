use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub(crate) struct CurrentPathParams {
    pub current_id: u32
}

#[derive(Debug,Serialize,Deserialize)]
pub(crate) struct HomeFolderFromPathParam {
    pub path_type: u32
}

#[derive(Debug,Serialize,Deserialize)]
pub(crate) enum PathType {
    Video,
    Audio,
    Image,
    Doc,
    Folder,
    NotNull
}

impl From<u32> for PathType {
    fn from(value: u32) -> Self {
        return match value {
            1 => {
                PathType::Video
            },
            2 => {
                PathType::Audio
            },
            3 => {
                PathType::Image
            },
            4 => {
                PathType::Doc
            },
            5 => {
                PathType::Folder
            },
            _ => {
                PathType::NotNull
            }
        }
    }
}