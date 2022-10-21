//! Application APIs
//!
//! You should authorize before using any application API.

pub mod auth;
pub mod common;
pub mod electricity;

use reqwest::cookie::Jar;
use std::{sync::Arc, time::Duration};

use crate::error::Error;

const APP_ID: &str = "1810181825222034";
const SESSION_KEY: &str = "shiroJID";

/// Authorized session handle
#[derive(Debug)]
pub struct AppHandler {
    client: reqwest::blocking::Client,
}

impl AppHandler {
    /// Using session string to create
    pub fn build(session: &str) -> Result<Self, Error> {
        Ok(Self {
            client: {
                // Store session in cookie jar
                let jar = Jar::default();
                jar.add_cookie_str(
                    &format!("{}={}", SESSION_KEY, session),
                    &reqwest::Url::parse(crate::url::application::BASE_URL).unwrap(),
                );

                reqwest::blocking::Client::builder()
                    .connect_timeout(Duration::new(5, 0))
                    .user_agent(super::USER_AGENT)
                    .cookie_provider(Arc::new(jar))
                    .build()?
            },
        })
    }
}
