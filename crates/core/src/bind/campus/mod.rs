//! Campus(yiSchool) APP API bindings
//!
//! See [`login`] for authorize.

use super::*;
use crate::{url::campus, utils::gen_random_fake_md5};
use serde::Deserialize;

pub mod login;
pub mod user;

/// Handler for Campus API
pub struct CampusHandler {
    client: Client,
    device_id: String,
    /// Session token
    pub token: String,
    pub uid: String,
    pub school_code: String,
}

impl CampusHandler {
    /// Build handler by session token & device id
    pub fn build(
        device_id: &str,
        uid: &str,
        school_code: &str,
        token: Option<&str>,
    ) -> Result<Self> {
        let client = init_app_sim_client(device_id)?;

        Ok(Self {
            client,
            device_id: device_id.to_string(),
            token: {
                match token {
                    Some(v) => v.to_string(),
                    None => gen_random_fake_md5(),
                }
            },
            uid: uid.to_string(),
            school_code: school_code.to_string(),
        })
    }

    fn req_body(&self) -> Vec<(&str, &str)> {
        vec![
            ("appVersion", APP_VER),
            ("deviceId", &self.device_id),
            ("platform", PLATFORM),
            ("testAccount", "1"),
            ("token", &self.token),
            ("ymId", &self.uid),
            ("schoolCode", &self.school_code),
        ]
    }
}

/// Init App simulated client
///
/// ## Contains
/// - [`reqwest::Client`]
/// - 5s timeout
/// - UA header
pub fn init_app_sim_client(device_id: &str) -> Result<Client> {
    let builder = Client::builder();

    let result: Client = builder
        .connect_timeout(std::time::Duration::new(5, 0))
        .user_agent(format!("{}{}", USER_AGENT, device_id))
        .build()?;

    Ok(result)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommonResponse<D = (), R = ()> {
    pub status_code: i64,
    /// Error code
    pub biz_code: Option<i64>,
    pub message: String,
    pub success: bool,
    pub data: Option<D>,
    pub _rows: Option<R>,
}

/// # Returns
/// - `Err` for handled errors.
/// - `Ok(true)` for some error unhandled.
/// - `Ok(false)` for no error.
fn check_auth_status<D, R>(resp: &CommonResponse<D, R>) -> Result<bool> {
    if !resp.success {
        if resp.status_code == 204 {
            if let Some(code) = resp.biz_code {
                match code {
                    10010 => return Err(Error::AuthUserNotFound),
                    10011 => return Err(Error::AuthDeviceChanged),
                    _ => {}
                }
            }
        }
        return Ok(true); // Unhandled errors
    }
    Ok(false)
}
