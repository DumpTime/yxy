use serde::Serialize;
use yxy::error::Error;

pub mod app;
pub mod campus;

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub code: u16,
    pub msg: String,
    pub data: T,
}

impl<T> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            code: 0,
            msg: "success".to_string(),
            data,
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub msg: String,
}

impl From<(u16, Error)> for ErrorResponse {
    fn from(e: (u16, Error)) -> Self {
        Self {
            code: e.0,
            msg: e.1.to_string(),
        }
    }
}
