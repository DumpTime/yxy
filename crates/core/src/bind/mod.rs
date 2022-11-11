//! API request bindings

use std::time::Duration;

use reqwest::{Client, Response};

use crate::error::Error;

pub mod app;
pub mod campus;
pub mod pay;

// Constant values
const OS_TYPE: &str = "iOS";
const MOBILE_TYPE: &str = "iPad8,6";
const OS_VERSION: &str = "15.6";
const APP_VER: &str = "411";
const APP_VER_NAME: &str = "4.2.1";
const PLATFORM: &str = "YUNMA_APP";
const USER_AGENT: &str = const_format::formatcp!(
    "{}/{}/{}/WKWebview ZJYXYwebviewbroswer ZJYXYIphone tourCustomer /yunmaapp.NET/{}/",
    OS_TYPE,
    OS_VERSION,
    MOBILE_TYPE,
    APP_VER_NAME,
);
const CLIENT_ID: &str = "65l09sfwa9ao2dc";

type Result<T> = std::result::Result<T, Error>;

/// Build a default [`reqwest::Client`].
pub fn build_default_client() -> Result<Client> {
    let builder = Client::builder();
    let result = builder
        .connect_timeout(Duration::new(5, 0))
        .user_agent(USER_AGENT)
        .build()?;

    Ok(result)
}

/// Build non-redirect [`reqwest::Client`].
///
/// This client is used to request OAuth code.
pub fn build_non_redirect_client() -> Result<Client> {
    let builder = Client::builder();
    let result = builder
        .connect_timeout(Duration::new(5, 0))
        .user_agent(USER_AGENT)
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    Ok(result)
}

/// Check response status code.
async fn check_response(res: &mut Response) -> Result<()> {
    if !res.status().is_success() {
        let text = res.chunk().await?;
        if let Some(text) = text {
            return Err(Error::Runtime(format!(
                "Bad response: ({}); {:?}",
                res.status(),
                text,
            )));
        } else {
            return Err(Error::Runtime(format!("Bad response: ({});", res.status())));
        }
    }

    Ok(())
}
