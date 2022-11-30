//! Application RESTful API Handler

use axum::{extract::Query, http::StatusCode, Json};
use yxy::error::Error;
use yxy::{wrapper::*, AppHandler};

use crate::handler::ResultE;

/// Build [`AppHandler`]
fn build_handler(token: &str) -> ResultE<AppHandler> {
    match AppHandler::build(token) {
        Ok(v) => Ok(v),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
    }
}

pub mod auth {
    use super::*;
    use crate::model::app::auth;

    pub async fn by_uid(query: Query<auth::Request>) -> ResultE<Json<auth::Response>> {
        match app_auth(&query.uid).await {
            Ok(r) => Ok(Json(auth::Response::from(r))),
            Err(e @ Error::Auth(_)) => Err((StatusCode::UNAUTHORIZED, Json(e.into()))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
        }
    }
}

pub mod electricity {
    use super::*;
    use crate::model::app::electricity;

    pub mod subsidy {
        use super::*;
        use electricity::subsidy::*;

        pub async fn by_user(
            Query(TokenRequest { token }): Query<TokenRequest>,
        ) -> ResultE<Json<Response>> {
            match query_ele(&token).await {
                Ok(v) => Ok(Json(Response::from(v))),
                Err(e @ Error::NoBind) => Err((StatusCode::NOT_FOUND, Json(e.into()))),
                Err(e @ Error::Auth(_)) => Err((StatusCode::UNAUTHORIZED, Json(e.into()))),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
            }
        }

        pub async fn by_room(v: Query<RoomInfoRequest>) -> ResultE<Json<Response>> {
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
        use electricity::bind::*;

        pub async fn by_user(
            Query(bind::Request { token }): Query<bind::Request>,
        ) -> ResultE<Json<Response>> {
            match query_ele_bind(&token).await {
                Ok(v) => Ok(Json(Response::from(v))),
                Err(e @ Error::NoBind) => Err((StatusCode::NOT_FOUND, Json(e.into()))),
                Err(e @ Error::Auth(_)) => Err((StatusCode::UNAUTHORIZED, Json(e.into()))),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
            }
        }
    }

    pub mod consumption {
        use super::*;
        use electricity::consumption::*;

        pub async fn by_room(Query(v): Query<Request>) -> ResultE<Json<Response>> {
            let (token, md_type, room_info) = v.split();
            let handler = build_handler(&token)?;

            match handler.usage_records(&room_info, md_type.as_deref()).await {
                Ok(v) => Ok(Json(Response::from(v))),
                Err(e @ Error::Auth(_)) => Err((StatusCode::UNAUTHORIZED, Json(e.into()))),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
            }
        }
    }

    pub mod recarge {
        use std::convert::TryFrom;

        use super::*;
        use electricity::recharge::*;

        pub async fn by_room(Query(v): Query<ByRoomRequest>) -> ResultE<Json<ByRoomResponse>> {
            let (token, page, room_info) = v.split();
            let handler = build_handler(&token)?;

            match handler.room_recharge_records(page, &room_info).await {
                Ok(v) => Ok(Json(ByRoomResponse::from(v))),
                Err(e @ Error::Auth(_)) => Err((StatusCode::UNAUTHORIZED, Json(e.into()))),
                Err(e @ Error::EmptyResp) => Err((StatusCode::NO_CONTENT, Json(e.into()))),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
            }
        }

        pub async fn by_user(
            Query(ByUserRequest { token, page, time }): Query<ByUserRequest>,
        ) -> ResultE<Json<ByUserResponse>> {
            let handler = build_handler(&token)?;

            match handler.user_recharge_records(page, time.as_deref()).await {
                Ok(v) => match ByUserResponse::try_from(v) {
                    Ok(v) => Ok(Json(v)),
                    Err(()) => Err((
                        StatusCode::NO_CONTENT,
                        Json(crate::model::ErrorResponse {
                            code: -1,
                            msg: String::new(),
                        }),
                    )),
                },
                Err(e @ Error::Auth(_)) => Err((StatusCode::UNAUTHORIZED, Json(e.into()))),
                Err(e @ Error::EmptyResp) => Err((StatusCode::NO_CONTENT, Json(e.into()))),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
            }
        }
    }
}
