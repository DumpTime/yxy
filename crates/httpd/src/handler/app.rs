use axum::{extract::Query, http::StatusCode, Json};
use yxy::error::Error;
use yxy::wrapper::*;

use crate::handler::ResultE;

pub mod auth {
    use super::*;
    use crate::model::app::auth;

    pub async fn by_uid(query: Query<auth::Query>) -> ResultE<Json<auth::Response>> {
        match app_auth(&query.uid).await {
            Ok(r) => Ok(Json(auth::Response::from(r))),
            Err(e @ Error::Auth(_)) => Err((StatusCode::UNAUTHORIZED, Json(e.into()))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
        }
    }
}

pub mod electricity {
    use super::*;

    pub mod subsidy {
        use super::*;
        use crate::model::app::electricity::subsidy::*;

        pub async fn by_token(
            Query(TokenQuery { token }): Query<TokenQuery>,
        ) -> ResultE<Json<Response>> {
            match query_ele(&token).await {
                Ok(v) => Ok(Json(Response::from(v))),
                Err(e @ Error::NoBind) => Err((StatusCode::NOT_FOUND, Json(e.into()))),
                Err(e @ Error::Auth(_)) => Err((StatusCode::UNAUTHORIZED, Json(e.into()))),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
            }
        }

        pub async fn by_room_info(v: Query<RoomInfoRequest>) -> ResultE<Json<Response>> {
            let (token, room_info) = v.0.split();
            match query_ele_by_room_info(&token, &room_info).await {
                Ok(v) => Ok(Json(Response::from(v))),
                Err(e @ Error::Auth(_)) => Err((StatusCode::UNAUTHORIZED, Json(e.into()))),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
            }
        }
    }

    pub mod bind {
        use super::*;
        use crate::model::app::electricity::bind;

        pub async fn by_token(
            Query(bind::Query { token }): Query<bind::Query>,
        ) -> ResultE<Json<bind::Response>> {
            match query_ele_bind(&token).await {
                Ok(v) => Ok(Json(bind::Response::from(v))),
                Err(e @ Error::NoBind) => Err((StatusCode::NOT_FOUND, Json(e.into()))),
                Err(e @ Error::Auth(_)) => Err((StatusCode::UNAUTHORIZED, Json(e.into()))),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
            }
        }
    }
}
