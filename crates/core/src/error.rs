//! Library Errors

use Error::*;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Runtime(String),
    Auth(String),
    OAuthError,
    AuthExpired,
    Request(reqwest::Error),
    Rsa(rsa::errors::Error),
    EmptyResp,
    NoBind,
    VerificationLimit,
    BadPhoneNumber,
    BadLoginSecret,
    DeviceChanged,
    RsaPkcs(rsa::pkcs8::spki::Error),
    Decode(std::string::FromUtf8Error),
    Base64Decode(base64::DecodeError),
    Serde(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IO(e) => write!(f, "IO error: {}", e),
            Runtime(e) => write!(f, "Runtime error: {}", e),
            Auth(e) => write!(f, "Authorization error: {}", e),
            OAuthError => write!(f, "OAuth error"),
            Request(e) => write!(f, "Request error: {}", e),
            EmptyResp => write!(f, "Get empty response"),
            AuthExpired => write!(f, "Authorization expired"),
            Rsa(e) => write!(f, "RSA crypto error: {}", e),
            RsaPkcs(e) => write!(f, "RSA crypto error: {}", e),
            Decode(e) => write!(f, "Decode error: {}", e),
            Base64Decode(e) => write!(f, "Decode error: {}", e),
            Serde(e) => write!(f, "Serde error: {}", e),
            NoBind => write!(f, "No bind info"),
            VerificationLimit => write!(f, "Verification limited, maybe too many requests"),
            BadPhoneNumber => write!(f, "Bad phone number"),
            BadLoginSecret => write!(f, "Bad Login secret code"),
            DeviceChanged => write!(f, "Device changed"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Request(e)
    }
}

impl From<rsa::errors::Error> for Error {
    fn from(e: rsa::errors::Error) -> Self {
        Self::Rsa(e)
    }
}

impl From<rsa::pkcs8::spki::Error> for Error {
    fn from(e: rsa::pkcs8::spki::Error) -> Self {
        Self::RsaPkcs(e)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::Decode(e)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Self::Base64Decode(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e)
    }
}
