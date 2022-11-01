pub mod app;

type ResultE<T> = std::result::Result<T, (axum::http::StatusCode, String)>;
