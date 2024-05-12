use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadVideoParams {
    pub file_path: String,
}
