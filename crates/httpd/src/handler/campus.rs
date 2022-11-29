use axum::{extract::Query, http::StatusCode, Json};
use yxy::error::Error;

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
        Query(request::DeviceID { device_id }): Query<request::DeviceID>,
    ) -> ResultE<Json<response::SecurityToken>> {
        let handler = build_handler(device_id)?;

        match handler.security_token().await {
            Ok(v) => Ok(Json(v.into())),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    pub async fn captcha_image(
        Query(request::CaptchaImage {
            security_token,
            device_id,
        }): Query<request::CaptchaImage>,
    ) -> ResultE<Json<response::CaptchaImage>> {
        let handler = build_handler(device_id)?;

        match handler.captcha_image(&security_token).await {
            Ok(v) => Ok(Json(response::CaptchaImage { img: v })),
            Err(e @ Error::BadInput(_)) => Err((StatusCode::BAD_REQUEST, e.to_string())),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    pub async fn send_verification_code(
        Json(request::SendVerificationCode {
            security_token,
            device_id,
            phone_num,
            captcha,
        }): Json<request::SendVerificationCode>,
    ) -> ResultE<Json<response::SendVerificationCode>> {
        let handler = build_handler(device_id)?;

        match handler
            .send_verification_code(&phone_num, &security_token, captcha.as_deref())
            .await
        {
            Ok(v) => Ok(Json(response::SendVerificationCode { user_exists: v })),
            Err(e @ Error::BadInput(_)) => Err((StatusCode::BAD_REQUEST, e.to_string())),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    pub async fn login_by_code(
        Json(request::LoginByCode {
            device_id,
            phone_num,
            code,
        }): Json<request::LoginByCode>,
    ) -> ResultE<Json<response::LoginInfo>> {
        let handler = build_handler(device_id)?;

        match handler.login_by_code(&phone_num, &code).await {
            Ok(v) => Ok(Json(v.into())),
            Err(e @ Error::BadLoginSecret) => Err((StatusCode::FORBIDDEN, e.to_string())),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    pub async fn silent_login(
        Json(request::SilentLogin {
            device_id,
            uid,
            token,
        }): Json<request::SilentLogin>,
    ) -> ResultE<Json<response::LoginInfo>> {
        let handler = build_handler(device_id)?;

        match handler.silent_login(&uid, token.as_deref()).await {
            Ok(v) => Ok(Json(v.into())),
            Err(e @ Error::AuthUserNotFound) => Err((StatusCode::FORBIDDEN, e.to_string())),
            Err(e @ Error::AuthDeviceChanged) => Err((StatusCode::FORBIDDEN, e.to_string())),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    pub async fn public_key(
        Query(request::DeviceID { device_id }): Query<request::DeviceID>,
    ) -> ResultE<Json<response::PublicKey>> {
        let handler = build_handler(device_id)?;

        match handler.public_key().await {
            Ok(v) => Ok(Json(response::PublicKey { key: v })),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    pub async fn login_by_password(
        Json(request::LoginByPassword {
            device_id,
            phone_num,
            password,
            public_key,
        }): Json<request::LoginByPassword>,
    ) -> ResultE<Json<response::LoginInfo>> {
        let handler = build_handler(device_id)?;

        match handler
            .login_by_password(&phone_num, &password, &public_key)
            .await
        {
            Ok(v) => Ok(Json(v.into())),
            Err(e @ Error::BadLoginSecret) => Err((StatusCode::FORBIDDEN, e.to_string())),
            Err(e @ Error::AuthDeviceChanged) => Err((StatusCode::FORBIDDEN, e.to_string())),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }
}
