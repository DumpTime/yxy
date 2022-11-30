use axum::Json;

use crate::model::ErrorResponse;

pub mod app;
pub mod campus;

type ResultE<T, E = Json<ErrorResponse>> = std::result::Result<T, (axum::http::StatusCode, E)>;
