pub mod auth {
    use axum::{extract::Query, http::StatusCode, Json};

    use crate::model::app::auth;
    use yxy::wrapper::*;

    pub async fn by_uid(query: Query<auth::Query>) -> Result<Json<auth::Response>, StatusCode> {
        if let Ok(r) = app_auth(&query.uid).await {
            Ok(Json(auth::Response::build(r)))
        } else {
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub mod electricity {

    pub mod subsidy {
        use axum::{extract::Query, http::StatusCode, Json};

        use crate::model::app::electricity::subsidy::*;
        use yxy::error::Error;
        use yxy::wrapper::*;

        pub async fn by_token(
            Query(TokenQuery { token }): Query<TokenQuery>,
        ) -> Result<Json<Response>, StatusCode> {
            match query_ele(&token).await {
                Ok(v) => Ok(Json(Response::build(v))),
                Err(Error::NoBind) => Err(StatusCode::NOT_FOUND),
                Err(_) => Err(StatusCode::UNAUTHORIZED),
            }
        }

        pub async fn by_room_info(v: Json<RoomInfoRequest>) -> Result<Json<Response>, StatusCode> {
            let (token, room_info) = v.0.split();
            match query_ele_by_room_info(&token, &room_info).await {
                Ok(v) => Ok(Json(Response::build(v))),
                Err(_) => Err(StatusCode::BAD_REQUEST),
            }
        }
    }

    pub mod bind {
        use axum::{extract::Query, http::StatusCode, Json};

        use crate::model::app::electricity::bind;
        use yxy::wrapper::*;

        pub async fn by_token(
            Query(bind::Query { token }): Query<bind::Query>,
        ) -> Result<Json<bind::Response>, StatusCode> {
            use yxy::error::Error;

            match query_ele_bind(&token).await {
                Ok(v) => Ok(Json(bind::Response::build(v))),
                Err(Error::NoBind) => Err(StatusCode::NOT_FOUND),
                Err(_) => Err(StatusCode::UNAUTHORIZED),
            }
        }
    }
}
