pub mod app;
pub mod user;

type ResultE<T> = std::result::Result<T, (axum::http::StatusCode, String)>;
