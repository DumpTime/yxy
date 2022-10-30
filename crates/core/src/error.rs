//! Library Errors

use thiserror::Error;

/// Library error type
#[derive(Debug, Error)]
pub enum Error {
    #[error("Runtime Error: {0}")]
    Runtime(String),
    #[error("Authorization Error: {0}")]
    Auth(String),
    #[error("Empty response.")]
    EmptyResp,
    #[error("No bind info.")]
    NoBind,
    #[error("Request limited.")]
    Limited,
    #[error("Invalid phone number.")]
    BadPhoneNumber,
    #[error("Invalid secrets.")]
    BadLoginSecret,
    #[error("Device changed")]
    DeviceChanged,

    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Rsa(#[from] rsa::errors::Error),
    #[error(transparent)]
    RsaPkcs(#[from] rsa::pkcs8::spki::Error),
    #[error(transparent)]
    Decode(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
