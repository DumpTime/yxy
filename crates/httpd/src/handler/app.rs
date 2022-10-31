use axum::{extract::Query, http::StatusCode, Json};

use crate::model::app::auth;
use yxy::wrapper::*;

pub async fn auth(query: Query<auth::Query>) -> Result<Json<auth::Response>, StatusCode> {
    if let Ok((t, v)) = app_auth(&query.uid).await {
        Ok(Json(auth::Response {
            token: t,
            user_info: v,
        }))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub mod electricity {
    use super::*;
    use crate::model::app::electricity::ByTokenQuery;

    pub async fn by_token(
        Query(ByTokenQuery { token }): Query<ByTokenQuery>,
    ) -> Result<Json<yxy::ElectricityInfo>, StatusCode> {
        match query_ele(&token).await {
            Ok(v) => Ok(Json(v)),
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    }
}
