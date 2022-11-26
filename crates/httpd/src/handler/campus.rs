use axum::{extract::Query, http::StatusCode, Json};

use super::*;

pub mod login {
    use yxy::bind::campus::login::*;

    use super::*;
    use crate::model::campus::login::*;

    fn build_handler(device_id: String) -> ResultE<LoginHandler> {
        match LoginHandler::build(device_id) {
            Ok(v) => Ok(v),
            Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
        }
    }

    pub async fn security_token(
        Query(DeviceIDQuery { device_id }): Query<DeviceIDQuery>,
    ) -> ResultE<Json<SecurityTokenResponce>> {
        let handler = build_handler(device_id)?;

        match handler.security_token().await {
            Ok(v) => Ok(Json(v.into())),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }
}
