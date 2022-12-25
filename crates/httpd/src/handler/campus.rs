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
        Err(e) => Err(Json((StatusCode::BAD_REQUEST.as_u16(), e).into())),
    }
}

impl TryFrom<BasicInfo> for CampusHandler {
    type Error = Json<ErrorResponse>;

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
            Err(e) => Err(Json((StatusCode::BAD_REQUEST.as_u16(), e).into())),
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
            Err(e) => Err(Json((StatusCode::BAD_REQUEST.as_u16(), e).into())),
        }
    }

    pub async fn security_token(
        Query(request::DeviceID { device_id }): Query<request::DeviceID>,
    ) -> HttpResult<response::SecurityToken> {
        let handler = build_handler(device_id)?;

        match handler.security_token().await {
            Ok(v) => success_result(v.into()),
            Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
        }
    }

    pub async fn captcha_image(
        Query(request::CaptchaImage {
            security_token,
            device_id,
        }): Query<request::CaptchaImage>,
    ) -> HttpResult<response::CaptchaImage> {
        let handler = build_handler(device_id)?;

        match handler.captcha_image(&security_token).await {
            Ok(v) => success_result(response::CaptchaImage { img: v }),
            Err(e @ Error::BadInput(_)) => error_result(StatusCode::BAD_REQUEST.as_u16(), e),
            Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
        }
    }

    pub async fn send_verification_code(
        Json(request::SendVerificationCode {
            security_token,
            device_id,
            phone_num,
            captcha,
        }): Json<request::SendVerificationCode>,
    ) -> HttpResult<response::SendVerificationCode> {
        let handler = build_handler(device_id)?;
        if security_token.len() < 32 {
            return error_result(
                StatusCode::BAD_REQUEST.as_u16(),
                Error::BadInput("security_token".to_string()),
            );
        }
        match handler
            .send_verification_code(&phone_num, &security_token, captcha.as_deref())
            .await
        {
            Ok(v) => success_result(response::SendVerificationCode { user_exists: v }),
            Err(e @ Error::BadInput(_)) => error_result(StatusCode::BAD_REQUEST.as_u16(), e),
            Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
        }
    }

    pub async fn login_by_code(
        Json(request::LoginByCode {
            device_id,
            phone_num,
            code,
        }): Json<request::LoginByCode>,
    ) -> HttpResult<response::LoginInfo> {
        let handler = build_handler(device_id)?;

        match handler.login_by_code(&phone_num, &code).await {
            Ok(v) => success_result(v.into()),
            Err(e @ Error::BadLoginSecret) => error_result(StatusCode::FORBIDDEN.as_u16(), e),
            Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
        }
    }

    pub async fn silent_login(
        Json(request::SilentLogin {
            device_id,
            uid,
            token,
        }): Json<request::SilentLogin>,
    ) -> HttpResult<response::LoginInfo> {
        let handler = build_handler(device_id)?;

        match handler.silent_login(&uid, token.as_deref()).await {
            Ok(v) => success_result(v.into()),
            Err(e @ Error::AuthUserNotFound) => error_result(StatusCode::FORBIDDEN.as_u16(), e),
            Err(e @ Error::AuthDeviceChanged) => error_result(StatusCode::FORBIDDEN.as_u16(), e),
            Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
        }
    }

    pub async fn public_key(
        Query(request::DeviceID { device_id }): Query<request::DeviceID>,
    ) -> HttpResult<response::PublicKey> {
        let handler = build_handler(device_id)?;

        match handler.public_key().await {
            Ok(v) => success_result(response::PublicKey { key: v }),
            Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
        }
    }

    pub async fn login_by_password(
        Json(request::LoginByPassword {
            device_id,
            phone_num,
            password,
            public_key,
        }): Json<request::LoginByPassword>,
    ) -> HttpResult<response::LoginInfo> {
        let handler = build_handler(device_id)?;

        match handler
            .login_by_password(&phone_num, &password, &public_key)
            .await
        {
            Ok(v) => success_result(v.into()),
            Err(e @ Error::BadLoginSecret) => error_result(StatusCode::FORBIDDEN.as_u16(), e),
            Err(e @ Error::AuthDeviceChanged) => error_result(StatusCode::FORBIDDEN.as_u16(), e),
            Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
        }
    }
}

pub mod user {
    use super::*;
    use campus::user::*;

    pub async fn card_balance(Query(info): Query<BasicInfo>) -> HttpResult<response::CardBalance> {
        let handler: CampusHandler = info.try_into()?;

        match handler.card_balance().await {
            Ok(v) => success_result(response::CardBalance { balance: v }),
            Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
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
    ) -> HttpResult<response::ConsumptionRecords> {
        let handler = build_handler(&device_id, &uid, &school_code, token.as_deref())?;

        match handler.consumption_records(&query_time).await {
            Ok(v) => success_result(v.into()),
            Err(e @ Error::NoBind) => error_result(StatusCode::FORBIDDEN.as_u16(), e),
            Err(e @ Error::EmptyResp) => error_result(StatusCode::NO_CONTENT.as_u16(), e),
            Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
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
    ) -> HttpResult<response::TransactionRecords> {
        let handler = build_handler(&device_id, &uid, &school_code, token.as_deref())?;

        match handler.transaction_records(offset, limit).await {
            Ok(v) => success_result(v.into()),
            Err(e @ Error::EmptyResp) => error_result(StatusCode::NO_CONTENT.as_u16(), e),
            Err(e) => error_result(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e),
        }
    }
}
