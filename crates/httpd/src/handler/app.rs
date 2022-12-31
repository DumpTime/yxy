//! Application RESTful API Handler

use axum::{extract::Query, http::StatusCode, Json};
use yxy::error::Error;
use yxy::{wrapper::*, AppHandler};

use super::{error_result, success_result};
use crate::handler::{HttpResult, ResultE};

/// Build [`AppHandler`]
fn build_handler(token: &str) -> ResultE<AppHandler> {
    match AppHandler::build(token) {
        Ok(v) => Ok(v),
        Err(e) => Err(Json((StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e).into())),
    }
}

pub mod auth {
    use super::*;
    use crate::model::app::auth;

    pub async fn by_uid(query: Query<auth::Request>) -> HttpResult<auth::Response> {
        match app_auth(&query.uid).await {
            Ok(r) => success_result(auth::Response::from(r)),
            Err(e @ Error::Auth(_)) => error_result(StatusCode::UNAUTHORIZED.as_u16(), e),
            Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
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
        ) -> HttpResult<Response> {
            match query_ele(&token).await {
                Ok(v) => success_result(Response::from(v)),
                Err(e @ Error::NoBind) => error_result(StatusCode::FORBIDDEN.as_u16(), e),
                Err(e @ Error::Auth(_)) => error_result(StatusCode::UNAUTHORIZED.as_u16(), e),
                Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
            }
        }

        pub async fn by_room(v: Query<RoomInfoRequest>) -> HttpResult<Response> {
            let (token, room_info) = v.0.split();
            match query_ele_by_room_info(&token, &room_info).await {
                Ok(v) => success_result(Response::from(v)),
                Err(e @ Error::Auth(_)) => error_result(StatusCode::UNAUTHORIZED.as_u16(), e),
                Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
            }
        }
    }

    pub mod bind {
        use super::*;
        use electricity::bind::*;

        pub async fn by_user(
            Query(bind::Request { token }): Query<bind::Request>,
        ) -> HttpResult<Response> {
            match query_ele_bind(&token).await {
                Ok(v) => success_result(Response::from(v)),
                Err(e @ Error::NoBind) => error_result(StatusCode::FORBIDDEN.as_u16(), e),
                Err(e @ Error::Auth(_)) => error_result(StatusCode::UNAUTHORIZED.as_u16(), e),
                Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
            }
        }
    }

    pub mod consumption {
        use super::*;
        use electricity::consumption::*;

        pub async fn by_room(Query(v): Query<Request>) -> HttpResult<Response> {
            let (token, md_type, room_info) = v.split();
            let handler = build_handler(&token)?;

            match handler.usage_records(&room_info, &md_type).await {
                Ok(v) => success_result(Response::from(v)),
                Err(e @ Error::Auth(_)) => error_result(StatusCode::UNAUTHORIZED.as_u16(), e),
                Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
            }
        }
    }

    pub mod recarge {
        use std::convert::TryFrom;

        use super::*;
        use electricity::recharge::*;

        pub async fn by_room(Query(v): Query<ByRoomRequest>) -> HttpResult<ByRoomResponse> {
            let (token, page, room_info) = v.split();
            let handler = build_handler(&token)?;

            match handler.room_recharge_records(page, &room_info).await {
                Ok(v) => success_result(ByRoomResponse::from(v)),
                Err(e @ Error::Auth(_)) => error_result(StatusCode::UNAUTHORIZED.as_u16(), e),
                Err(e @ Error::EmptyResp) => error_result(StatusCode::NO_CONTENT.as_u16(), e),
                Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
            }
        }

        pub async fn by_user(
            Query(ByUserRequest { token, page, time }): Query<ByUserRequest>,
        ) -> HttpResult<ByUserResponse> {
            let handler = build_handler(&token)?;

            match handler.user_recharge_records(page, time.as_deref()).await {
                Ok(v) => match ByUserResponse::try_from(v) {
                    Ok(v) => success_result(v),
                    Err(()) => error_result(StatusCode::NO_CONTENT.as_u16(), Error::EmptyResp),
                },
                Err(e @ Error::Auth(_)) => error_result(StatusCode::UNAUTHORIZED.as_u16(), e),
                Err(e @ Error::EmptyResp) => error_result(StatusCode::NO_CONTENT.as_u16(), e),
                Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
            }
        }
    }
}
