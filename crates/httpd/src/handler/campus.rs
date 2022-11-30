use std::convert::{TryFrom, TryInto};

use axum::{extract::Query, http::StatusCode, Json};
use yxy::{bind::campus::CampusHandler, error::Error};

use super::*;
use crate::model::campus::{self, BasicInfo};

/// Build [`CampusHandler`]
fn build_handler(
    device_id: &str,
    uid: &str,
    school_code: &str,
    token: Option<&str>,
) -> ResultE<CampusHandler> {
    match CampusHandler::build(device_id, uid, school_code, token) {
        Ok(v) => Ok(v),
        Err(e) => Err((StatusCode::BAD_REQUEST, Json(e.into()))),
    }
}

impl TryFrom<BasicInfo> for CampusHandler {
    type Error = (axum::http::StatusCode, Json<ErrorResponse>);

    fn try_from(
        BasicInfo {
            device_id,
            token,
            uid,
            school_code,
        }: BasicInfo,
    ) -> ResultE<Self> {
        match CampusHandler::build(&device_id, &uid, &school_code, token.as_deref()) {
            Ok(v) => Ok(v),
            Err(e) => Err((StatusCode::BAD_REQUEST, Json(e.into()))),
        }
    }
}

pub mod login {
    use yxy::bind::campus::login::*;

    use super::*;
    use campus::login::*;

    /// Build [`LoginHandler`]
    fn build_handler(device_id: String) -> ResultE<LoginHandler> {
        match LoginHandler::build(device_id) {
            Ok(v) => Ok(v),
            Err(e) => Err((StatusCode::BAD_REQUEST, Json(e.into()))),
        }
    }

    pub async fn security_token(
        Query(request::DeviceID { device_id }): Query<request::DeviceID>,
    ) -> ResultE<Json<response::SecurityToken>> {
        let handler = build_handler(device_id)?;

        match handler.security_token().await {
            Ok(v) => Ok(Json(v.into())),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
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
            Err(e @ Error::BadInput(_)) => Err((StatusCode::BAD_REQUEST, Json(e.into()))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
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
            Err(e @ Error::BadInput(_)) => Err((StatusCode::BAD_REQUEST, Json(e.into()))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
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
            Err(e @ Error::BadLoginSecret) => Err((StatusCode::FORBIDDEN, Json(e.into()))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
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
            Err(e @ Error::AuthUserNotFound) => Err((StatusCode::FORBIDDEN, Json(e.into()))),
            Err(e @ Error::AuthDeviceChanged) => Err((StatusCode::FORBIDDEN, Json(e.into()))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
        }
    }

    pub async fn public_key(
        Query(request::DeviceID { device_id }): Query<request::DeviceID>,
    ) -> ResultE<Json<response::PublicKey>> {
        let handler = build_handler(device_id)?;

        match handler.public_key().await {
            Ok(v) => Ok(Json(response::PublicKey { key: v })),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
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
            Err(e @ Error::BadLoginSecret) => Err((StatusCode::FORBIDDEN, Json(e.into()))),
            Err(e @ Error::AuthDeviceChanged) => Err((StatusCode::FORBIDDEN, Json(e.into()))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
        }
    }
}

pub mod user {
    use super::*;
    use campus::user::*;

    pub async fn card_balance(
        Query(info): Query<BasicInfo>,
    ) -> ResultE<Json<response::CardBalance>> {
        let handler: CampusHandler = info.try_into()?;

        match handler.card_balance().await {
            Ok(v) => Ok(Json(response::CardBalance { balance: v })),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
        }
    }

    pub async fn consumption_records(
        Query(request::ConsumptionRecords {
            device_id,
            token,
            uid,
            school_code,
            query_time,
        }): Query<request::ConsumptionRecords>,
    ) -> ResultE<Json<response::ConsumptionRecords>> {
        let handler = build_handler(&device_id, &uid, &school_code, token.as_deref())?;

        match handler.consumption_records(&query_time).await {
            Ok(v) => Ok(Json(v.into())),
            Err(e @ Error::NoBind) => Err((StatusCode::FORBIDDEN, Json(e.into()))),
            Err(e @ Error::EmptyResp) => Err((StatusCode::NO_CONTENT, Json(e.into()))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
        }
    }

    pub async fn transaction_records(
        Query(request::TransactionRecords {
            device_id,
            token,
            uid,
            school_code,
            offset,
            limit,
        }): Query<request::TransactionRecords>,
    ) -> ResultE<Json<response::TransactionRecords>> {
        let handler = build_handler(&device_id, &uid, &school_code, token.as_deref())?;

        match handler.transaction_records(offset, limit).await {
            Ok(v) => Ok(Json(v.into())),
            Err(e @ Error::EmptyResp) => Err((StatusCode::NO_CONTENT, Json(e.into()))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.into()))),
        }
    }
}
