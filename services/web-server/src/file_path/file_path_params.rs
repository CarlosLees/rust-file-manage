use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct CurrentPathParams {
    pub parent_id: i32,
    pub intact_path: String,
}

