//! API request bindings

pub mod app;
pub mod login;
pub mod pay;

use reqwest::blocking::Response;
use std::{io::Read, time::Duration};

use crate::error::Error;
use crate::url;

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

/// Build a default [`reqwest::blocking::Client`].
pub fn build_default_client() -> Result<reqwest::blocking::Client, Error> {
    let builder: reqwest::blocking::ClientBuilder = reqwest::blocking::Client::builder();
    let result: reqwest::blocking::Client = builder
        .connect_timeout(Duration::new(5, 0))
        .user_agent(USER_AGENT)
        .build()?;

    Ok(result)
}

/// Build non-redirect [`reqwest::blocking::Client`].
///
/// This client is used to request OAuth code.
pub fn build_non_redirect_client() -> Result<reqwest::blocking::Client, Error> {
    let builder: reqwest::blocking::ClientBuilder = reqwest::blocking::Client::builder();
    let result: reqwest::blocking::Client = builder
        .connect_timeout(Duration::new(5, 0))
        .user_agent(USER_AGENT)
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    Ok(result)
}

/// Check response status code.
fn check_response(res: &mut Response) -> Result<(), Error> {
    if !res.status().is_success() {
        let mut text = String::new();
        res.read_to_string(&mut text)?;
        return Err(Error::Runtime(format!(
            "Bad response: {}\nText: {}",
            res.status(),
            text,
        )));
    }

    Ok(())
}
