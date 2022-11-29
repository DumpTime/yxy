use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::handler::*;

pub fn init() -> Router {
    let v1 = Router::new()
        .nest(
            "/campus",
            Router::new().nest(
                "/login",
                Router::new()
                    .route("/security_token", get(campus::login::security_token))
                    .route("/captcha_image", get(campus::login::captcha_image))
                    .route(
                        "/send_verification_code",
                        post(campus::login::send_verification_code),
                    )
                    .route("/by_code", post(campus::login::login_by_code))
                    .route("/by_password", post(campus::login::login_by_password))
                    .route("/silent", post(campus::login::silent_login))
                    .route("/public_key", get(campus::login::public_key)),
            ),
        )
        .nest(
            "/app",
            Router::new().route("/auth", get(app::auth::by_uid)).nest(
                "/electricity",
                Router::new()
                    .route(
                        "/subsidy/by_token",
                        get(app::electricity::subsidy::by_token),
                    )
                    .route(
                        "/subsidy/by_room_info",
                        get(app::electricity::subsidy::by_room_info),
                    )
                    .route("/bind", get(app::electricity::bind::by_token)),
            ),
        );

    let router = Router::new()
        .route("/", get(|| async { "Hello, YXY HTTPd" }))
        .nest("/v1", v1)
        .layer(middleware::from_fn(access_log));

    #[cfg(debug_assertions)]
    {
        use tower_http::trace::TraceLayer;
        return router.layer(TraceLayer::new_for_http());
    }

    #[allow(unreachable_code)]
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
