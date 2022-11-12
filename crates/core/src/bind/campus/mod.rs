//! Campus(yiSchool) APP API bindings
//!
//! See [`login`] for authorize.

use super::*;
use crate::url::campus;

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
    pub fn build(token: &str, device_id: &str, uid: &str, school_code: &str) -> Result<Self> {
        let client = init_app_sim_client(device_id)?;

        Ok(Self {
            client,
            device_id: device_id.to_string(),
            token: token.to_string(),
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
