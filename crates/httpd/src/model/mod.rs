use serde::Serialize;
use yxy::error::Error;

pub mod app;
pub mod campus;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: i16,
    pub msg: String,
}

impl From<Error> for ErrorResponse {
    fn from(e: Error) -> Self {
        Self {
            code: -1,
            msg: e.to_string(),
        }
    }
}
