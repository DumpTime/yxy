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
    let v1 = Router::new().nest(
        "/app",
        Router::new()
            .route("/auth", get(handler::app::auth::by_uid))
            .nest(
                "/electricity",
                Router::new()
                    .route(
                        "/subsidy",
                        get(handler::app::electricity::subsidy::by_token),
                    )
                    .route("/bind", get(handler::app::electricity::bind::by_token)),
            ),
    );

    Router::new()
        .route("/", get(|| async { "Hello, YXY HTTPd" }))
        .nest("/v1", v1)
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
    tracing::info!("{method} | {status} | {uri}");

    Ok(res)
}
