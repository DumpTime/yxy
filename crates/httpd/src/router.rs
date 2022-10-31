use crate::handler;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};

pub fn init() -> Router {
    let app = Router::new()
        .route("/auth", get(handler::app::auth))
        .route("/electricity", get(handler::app::electricity::by_token));

    let api = Router::new().nest("/app", app);

    Router::new()
        .nest("/api", api)
        .layer(middleware::from_fn(access_log))
}

async fn access_log(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let method = req.method().clone();
    let uri = req.uri().clone();

    // Process
    let res = next.run(req).await;

    let status = res.status();

    let _enter = tracing::span!(tracing::Level::INFO, "ACCESS").entered();
    // log
    tracing::info!("{} | {} | {}", method, status, uri);

    Ok(res)
}
