use axum::{extract::Query, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use yxy::wrapper::app_auth;

#[derive(Deserialize)]
pub struct AuthQuery {
    uid: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    token: String,
    user_info: yxy::UserInfo,
}

pub async fn auth(query: Query<AuthQuery>) -> Result<Json<AuthResponse>, StatusCode> {
    if let Ok((t, v)) = app_auth(&query.uid).await {
        Ok(Json(AuthResponse {
            token: t,
            user_info: v,
        }))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
