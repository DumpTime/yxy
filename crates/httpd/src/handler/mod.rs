use axum::Json;

use crate::model::{ErrorResponse, SuccessResponse};

pub mod app;
pub mod campus;

type ResultE<T> = Result<T, Json<ErrorResponse>>;
type HttpResult<T> = std::result::Result<Json<SuccessResponse<T>>, Json<ErrorResponse>>;

fn success_result<T>(data: T) -> HttpResult<T> {
    HttpResult::Ok(Json(SuccessResponse::new(data)))
}

fn error_result<T>(code: u16, error: yxy::error::Error) -> HttpResult<T> {
    HttpResult::Err(Json((code, error).into()))
}
