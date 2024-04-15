use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HttpResult<T> {
    pub msg: Option<String>,
    pub code: u16,
    pub data: Option<T>,
}

impl<T> HttpResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 200,
            msg: Some("成功".into()),
            data: Some(data),
        }
    }

    pub fn error(msg: String) -> Self {
        Self {
            code: 500,
            msg: Some(msg),
            data: None,
        }
    }
}

impl<T> Default for HttpResult<T> {
    fn default() -> Self {
        HttpResult {
            code: 200,
            msg: Some("成功".into()),
            data: None
        }
    }
}
