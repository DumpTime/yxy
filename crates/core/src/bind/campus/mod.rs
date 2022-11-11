//! Campus(yiSchool) APP API bindings
//!
//! See [`login`] for authorize.

use super::*;

pub mod card;
pub mod login;

/// TODO
pub struct CampusHandler {
    _client: Client,
    _device_id: String,
    pub token: String,
}

/// TODO
impl CampusHandler {
    pub fn build(_token: &str, _device_id: &str) -> Self {
        todo!();
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
