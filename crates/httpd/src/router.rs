use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::handler;

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
                        get(handler::app::electricity::subsidy::by_token)
                            .post(handler::app::electricity::subsidy::by_room_info),
                    )
                    .route("/bind", get(handler::app::electricity::bind::by_token)),
            ),
    );

    let router = Router::new()
        .route("/", get(|| async { "Hello, YXY HTTPd" }))
        .nest("/v1", v1)
        .layer(middleware::from_fn(access_log));

    if cfg!(debug_assertions) {
        use tower_http::trace::TraceLayer;
        return router.layer(TraceLayer::new_for_http());
    }

    router
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
