use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HomeFolderFromPathResponse {
    file_type: u32,
    file_name: String
}

impl HomeFolderFromPathResponse {
    pub fn new(file_type: u32, file_name: String) -> Self{
        Self {
            file_type,
            file_name,
        }
    }
}