use serde::{Deserialize, Serialize};
use lib_entity::file_path::Model;

#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentPathFoldersResponse {
    pub data: Vec<Model>,
    pub current_path: String
}

impl CurrentPathFoldersResponse {
    pub fn new(data:Vec<Model>,current_path: String) -> Self {
        Self {
            data,
            current_path
        }
    }
}