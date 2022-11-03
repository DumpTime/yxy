//! Application authorize APIs

use reqwest::{
    cookie::{Cookie, Jar},
    Client,
};
use serde::Deserialize;
use std::{sync::Arc, time::Duration};

use super::*;

/// Extract code value from redirection URL query string
///
/// # Example match URL
///
/// ```text
/// https://application.xiaofubao.com/?code=b3cb4e67111b453488d826ba4397d921&errCode=0&status=null
/// ```
///
/// If matched, return the `code` value
fn extract_code(url: &str) -> Option<String> {
    match reqwest::Url::parse(url) {
        Ok(url) => {
            let query = url.query_pairs();

            for (key, value) in query {
                if key == "code" {
                    return Some(value.to_string());
                }
            }

            None
        }
        Err(_) => None,
    }
}

/// Request and extract code from redirect `Location`
///
/// # Example request
/// ```http
/// GET /authoriz/getCodeV2?bindSkip=1&authType=2&appid=1810181825222034&callbackUrl=https%3A%2F%2Fapplication.xiaofubao.com%2F&unionid=1234567890&schoolCode=2333 HTTP/1.1
/// User-Agent: Mozilla/5.0 (Linux; Android 11; Android for arm64; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/66.0.3359.158 Mobile Safari/537.36 ZJYXYwebviewbroswer ZJYXYAndroid tourCustomer/yunmaapp.NET/4.2.0/
/// X-Requested-With: cn.com.yunma.school.app
/// Host: auth.xiaofubao.com
/// Connection: close
/// ```
///
/// # Example response
/// ```http
/// HTTP/1.1 302 Found
/// Server: SCDN
/// Date: XXXXXXXXXXX
/// Content-Length: 0
/// Connection: close
/// X-Content-Type-Options: nosniff
/// X-XSS-Protection: 1; mode=block
/// Cache-Control: no-cache, no-store, max-age=0, must-revalidate
/// Pragma: no-cache
/// Expires: 0
/// Location: https://application.xiaofubao.com/?code=d15246f70c1d48658231153c75feb73f&errCode=0&
/// Content-Language: en-US
/// Strict-Transport-Security: max-age=604800; includeSubDomains;
/// ```
///
/// **Must use a none redirect policy client**
pub async fn get_oauth_code(client: &Client, id: &str) -> Result<String> {
    let query = [
        ("bindSkip", "1"),
        ("authType", "2"),
        ("appid", super::APP_ID),
        (
            "callbackUrl",
            &format!("{}/", crate::url::application::BASE_URL),
        ),
        ("unionid", id),
    ];

    let response = client
        .get(crate::url::auth::OAUTH_URL)
        .query(&query)
        .send()
        .await?;

    if !response.status().is_redirection() {
        return Err(Error::Auth("OAuth failed.".to_string()));
    }

    let header_location = match response.headers().get("Location") {
        Some(header) => header,
        None => return Err(Error::EmptyResp),
    };

    match extract_code(header_location.to_str().unwrap()) {
        Some(t) => Ok(t),
        None => Err(Error::EmptyResp),
    }
}

/// Authorize the handler and fetch user infos
pub async fn authorize(client: &Client, code: &str) -> Result<(String, UserInfo)> {
    // Form data
    let params = [("code", code)];

    let mut response = client
        .post(crate::url::application::GET_USER_FOR_AUTHORIZE)
        .form(&params)
        .send()
        .await?;

    crate::bind::check_response(&mut response).await?;

    let cookies: Vec<Cookie> = response.cookies().collect();

    // get session
    match cookies.iter().find(|x| x.name() == super::SESSION_KEY) {
        Some(v) => {
            let session = v.value().to_string();
            let resp = response.bytes().await?;

            let resp: AuthResponse = match serde_json::from_slice(resp.as_ref()) {
                Ok(v) => v,
                Err(e) => return Err(Error::Deserialize(e, resp)),
            };
            if !resp.success {
                return Err(Error::Auth(resp.message));
            }
            match resp.data {
                Some(v) => Ok((session, v)),
                None => Err(Error::Auth(resp.message)),
            }
        }
        None => {
            let resp_ser: AuthResponse = response.json().await?;
            Err(Error::Auth(resp_ser.message))
        }
    }
}

impl super::AppHandler {
    /// Create new app handler with authorize
    pub async fn build_by_uid(uid: &str) -> Result<Self> {
        // Store session in cookie jar
        let jar = Jar::default();

        let client = Client::builder()
            .connect_timeout(Duration::new(5, 0))
            .user_agent(crate::bind::USER_AGENT)
            .redirect(reqwest::redirect::Policy::none())
            .cookie_provider(Arc::new(jar))
            .build()?;

        let code = get_oauth_code(&client, uid).await?;

        // Form data
        let params = [("code", code.as_str())];

        let mut response = client
            .post(crate::url::application::GET_USER_FOR_AUTHORIZE)
            .form(&params)
            .send()
            .await?;

        crate::bind::check_response(&mut response).await?;

        let resp = response.bytes().await?;

        let resp: AuthResponse = match serde_json::from_slice(resp.as_ref()) {
            Ok(v) => v,
            Err(e) => return Err(Error::Deserialize(e, resp)),
        };
        if !resp.success {
            return Err(Error::Auth(resp.message));
        }

        Ok(Self { client })
    }

    /// Get user info
    pub async fn user_info(&self) -> Result<UserInfo> {
        // Form data
        let params = [("userId", rand::random::<u8>())];

        let mut response = self
            .client
            .post(crate::url::application::GET_USER_FOR_AUTHORIZE)
            .form(&params)
            .send()
            .await?;
        crate::bind::check_response(&mut response).await?;

        let resp: AuthResponse = response.json().await?;
        if !resp.success {
            return Err(Error::Runtime(format!(
                "Get user info failed: {}",
                resp.message
            )));
        }
        match resp.data {
            Some(v) => Ok(v),
            None => Err(Error::Runtime(format!(
                "Get user info failed: {}",
                resp.message
            ))),
        }
    }
}

// ====================
// ====== Models ======
// ====================

/// Authorize API response definition
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthResponse {
    // status_code: i32,
    message: String,
    success: bool,
    data: Option<UserInfo>,
}

/// User info provided by platform
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    /// UID: User ID
    pub id: String,
    pub mobile_phone: String,
    pub sex: i8,
    pub test_account: i8,
    pub platform: String,
    pub third_openid: String,
    pub school_code: Option<String>,
    pub school_name: Option<String>,
    pub user_name: Option<String>,
    pub user_type: Option<String>,
    pub job_no: Option<String>,
    pub user_idcard: Option<String>,
    pub user_class: Option<String>,
    pub bind_card_status: Option<i8>,
}
