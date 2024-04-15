use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub(crate) struct CurrentPathParams {
    pub current_id: u32
}