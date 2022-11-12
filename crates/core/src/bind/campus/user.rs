//! Campus user API bindings

use serde::Deserialize;

use super::*;
use campus::user::*;

mod error_message {
    pub const DEVICE_CHANGED: &str = "[deviceId changed]";
    pub const USER_NOT_FOUND: &str = "[user no find]";
}

impl CampusHandler {
    pub async fn query_card_balance(&self) -> Result<String> {
        let body = self.req_body();

        let mut resp = self
            .client
            .post(QUERY_CARD_BALANCE)
            .form(&body)
            .send()
            .await?;
        check_response(&mut resp).await?;

        let resp: Response<String> = resp.json().await?;
        if check_auth_status(&resp)? {
            return Err(Error::Runtime(format!(
                "Fail to query card balance: ({}); {}",
                resp.status_code, resp.message
            )));
        }

        if let Some(v) = resp.data {
            Ok(v)
        } else {
            Err(Error::EmptyResp)
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response<T> {
    pub status_code: i64,
    pub message: String,
    pub success: bool,
    pub data: Option<T>,
}

/// # Returns
/// - `Err` for handled errors.
/// - `Ok(true)` for some error unhandled.
/// - `Ok(false)` for no error.
fn check_auth_status<T>(resp: &Response<T>) -> Result<bool> {
    if !resp.success {
        if resp.status_code == 204 {
            let msg = &resp.message;
            if msg.ends_with(error_message::DEVICE_CHANGED) {
                return Err(Error::DeviceChanged);
            }
            if msg.ends_with(error_message::USER_NOT_FOUND) {
                return Err(Error::UserNotFound);
            }
        }
        return Ok(true);
    }
    Ok(false)
}

// ================
// ==== Models ====
// ================
