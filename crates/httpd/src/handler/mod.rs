pub mod app;
pub mod campus;

type ResultE<T> = std::result::Result<T, (axum::http::StatusCode, String)>;
