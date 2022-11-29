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
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    pub async fn send_verification_code(
        Query(request::SendVerificationCode {
            security_token,
            device_id,
            phone_num,
            captcha,
        }): Query<request::SendVerificationCode>,
    ) -> ResultE<Json<response::SendVerificationCode>> {
        let handler = build_handler(device_id)?;

        match handler
            .send_verification_code(&security_token, &phone_num, captcha.as_deref())
            .await
        {
            Ok(v) => Ok(Json(response::SendVerificationCode { user_exists: v })),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    pub async fn login_by_code(
        Query(request::LoginByCode {
            device_id,
            phone_num,
            verification_code,
        }): Query<request::LoginByCode>,
    ) -> ResultE<Json<response::LoginInfo>> {
        let handler = build_handler(device_id)?;

        match handler.login_by_code(&phone_num, &verification_code).await {
            Ok(v) => Ok(Json(v.into())),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    pub async fn silent_login(
        Query(request::SilentLogin {
            device_id,
            uid,
            token,
        }): Query<request::SilentLogin>,
    ) -> ResultE<Json<response::LoginInfo>> {
        let handler = build_handler(device_id)?;

        match handler.silent_login(&uid, Some(&token)).await {
            Ok(v) => Ok(Json(v.into())),
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
        Query(request::LoginByPassword {
            device_id,
            phone_num,
            password,
            public_key,
        }): Query<request::LoginByPassword>,
    ) -> ResultE<Json<response::LoginInfo>> {
        let handler = build_handler(device_id)?;

        match handler
            .login_by_password(&phone_num, &password, &public_key)
            .await
        {
            Ok(v) => Ok(Json(v.into())),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }
}
