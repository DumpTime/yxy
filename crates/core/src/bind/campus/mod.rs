//! Campus(yiSchool) APP API bindings
//!
//! See [`login`] for authorize.

use super::*;

pub mod card;
pub mod login;

impl BasicRequestBody for CampusHandler {
    fn device_id(&self) -> &str {
        &self.device_id
    }
}

trait BasicRequestBody {
    fn req_body(&self) -> Vec<(&str, &str)> {
        vec![
            ("appVersion", APP_VER),
            ("deviceId", self.device_id()),
            ("platform", PLATFORM),
            ("testAccount", "1"),
        ]
    }

    fn device_id(&self) -> &str;
}

/// Handler for Campus API
pub struct CampusHandler {
    client: Client,
    device_id: String,
    pub token: String,
}

impl CampusHandler {
    pub fn build(token: &str, device_id: &str) -> Result<Self> {
        let client = init_app_sim_client(device_id)?;

        Ok(Self {
            client,
            device_id: device_id.to_string(),
            token: token.to_string(),
        })
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

    let result: reqwest::Client = builder
        .connect_timeout(std::time::Duration::new(5, 0))
        .user_agent(format!("{}{}", super::USER_AGENT, device_id))
        .build()?;

    Ok(result)
}
