//! Campus login APIs

use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;
use reqwest::Client;
use serde::Deserialize;

use super::*;
use crate::url::campus::login::*;
use crate::utils::{md5, pkcs7_padding};

/// Handle of login procedure
pub struct LoginHandler {
    client: Client,
    device_id: String,
}

impl LoginHandler {
    /// Create handler with generated UUID in place of `device_id`
    pub fn new() -> Result<Self> {
        let device_id = uuid::Uuid::new_v4().to_string().to_uppercase();

        Ok(Self {
            client: init_app_sim_client(&device_id)?,
            device_id,
        })
    }

    /// Init handler by specific device id
    pub fn build(device_id: String) -> Result<Self> {
        Ok(Self {
            client: init_app_sim_client(&device_id)?,
            device_id,
        })
    }

    fn req_body(&self) -> Vec<(&str, &str)> {
        vec![
            ("appVersion", APP_VER),
            ("deviceId", &self.device_id),
            ("platform", PLATFORM),
            ("testAccount", "1"),
        ]
    }

    /// Return security token & level
    pub async fn security_token(&self) -> Result<SecurityTokenInfo> {
        let body = self.req_body();

        let mut resp = self
            .client
            .post(GET_SECURITY_TOKEN)
            .form(&body)
            .send()
            .await?;
        check_response(&mut resp).await?;

        let resp: BasicResponse<SecurityTokenInfo> = resp.json().await?;
        if !resp.success {
            return Err(Error::Runtime(format!(
                "Get security token failed: ({}); {}",
                resp.status_code, resp.message,
            )));
        }

        match resp.data {
            Some(v) => Ok(v),
            None => Err(Error::EmptyResp),
        }
    }

    /// Get image captcha
    ///
    /// Return image captcha base64 string
    pub async fn captcha_image(&self, security_token: &str) -> Result<String> {
        let mut body = self.req_body();
        body.push(("securityToken", security_token));

        let mut resp = self
            .client
            .post(GET_IMAGE_CAPTCHA)
            .form(&body)
            .send()
            .await?;
        check_response(&mut resp).await?;

        let resp: BasicResponse<String> = resp.json().await?;
        if !resp.success {
            if resp.message == error_messages::BAD_TOKEN {
                return Err(Error::BadInput(resp.message));
            }

            Err(Error::Runtime(format!(
                "Get image captcha failed: ({}); {}",
                resp.status_code, resp.message
            )))
        } else if let Some(v) = resp.data {
            Ok(v)
        } else {
            Err(Error::EmptyResp)
        }
    }

    /// Request to send login verification code SMS
    pub async fn send_verification_code(
        &self,
        phone_num: &str,
        security_token: &str,
        captcha: Option<&str>,
    ) -> Result<bool> {
        let mut body = self.req_body();

        let app_security_token = app_security_token(security_token, &self.device_id)?; // Important

        body.push(("appSecurityToken", &app_security_token));
        body.push(("securityToken", security_token));
        body.push(("sendCount", "1"));
        body.push(("mobilePhone", phone_num));

        // If image captcha required
        if let Some(v) = captcha {
            body.push(("imageCaptchaValue", v));
        }

        let mut resp = self
            .client
            .post(SEND_VERIFICATION_CODE)
            .form(&body)
            .send()
            .await?;
        check_response(&mut resp).await?;

        /// Define data object
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Data {
            user_exists: bool,
        }

        let resp: BasicResponse<Data> = resp.json().await?;
        if !resp.success {
            if resp.status_code == 203 {
                if resp.message == error_messages::BAD_PHONE_NUM
                    || resp.message == error_messages::BAD_PHONE_NUM_FORMAT
                {
                    return Err(Error::BadPhoneNumber);
                }
                if resp.message.starts_with(error_messages::TOO_FREQUENT)
                    || resp.message == error_messages::FLOW_CONTROL
                    || resp.message == error_messages::TOO_MANY_TRIES
                {
                    return Err(Error::Limited);
                }
            }

            if resp.message == error_messages::VERIFICATION_EXPIRED {
                return Err(Error::BadInput(resp.message));
            }

            return Err(Error::Runtime(format!(
                "Send verification code error: ({}); {}",
                resp.status_code, resp.message
            )));
        }

        // User status
        if let Some(v) = resp.data {
            Ok(v.user_exists)
        } else {
            Err(Error::EmptyResp)
        }
    }

    /// Do login by verification code
    ///
    /// return [`LoginInfo`]
    pub async fn login_by_code(&self, phone_num: &str, code: &str) -> Result<LoginInfo> {
        let mut body = self.req_body();
        body.push(("clientId", super::CLIENT_ID));
        body.push(("mobilePhone", phone_num));
        body.push(("osType", super::OS_TYPE));
        body.push(("osUuid", &self.device_id));
        body.push(("osVersion", super::OS_VERSION));
        body.push(("verificationCode", code));

        let mut resp = self
            .client
            .post(DO_LOGIN_BY_CODE)
            .form(&body)
            .send()
            .await?;
        check_response(&mut resp).await?;

        let buf = resp.bytes().await?;

        let resp: BasicResponse<LoginInfo> = match serde_json::from_slice(buf.as_ref()) {
            Ok(v) => v,
            Err(e) => return Err(Error::Deserialize(e, buf)),
        };

        if !resp.success {
            if resp.message.starts_with(error_messages::WRONG_SECRET) {
                return Err(Error::BadLoginSecret);
            }

            return Err(Error::Runtime(format!(
                "Login error: ({}); {}",
                resp.status_code, resp.message
            )));
        }

        if let Some(v) = resp.data {
            Ok(v)
        } else {
            Err(Error::EmptyResp)
        }
    }

    /// Do login in silent
    ///
    /// Bind to [`DO_LOGIN_BY_TOKEN`]
    ///
    /// **token** is optional.
    /// If `None` is provided, a random one will be generated.
    ///
    /// The set of UID-DeviceID binding is the key to authentication.
    ///
    /// Used to get new [`LoginInfo`] (contains new token)
    /// Also can be used to check specific device user login status.
    pub async fn silent_login(&self, uid: &str, token: Option<&str>) -> Result<LoginInfo> {
        let mut body = self.req_body();
        body.push(("clientId", super::CLIENT_ID));
        body.push(("osType", super::OS_TYPE));
        body.push(("osUuid", &self.device_id));
        body.push(("osVersion", super::OS_VERSION));
        body.push(("ymId", uid));

        let mut resp = if let Some(t) = token {
            body.push(("token", t));
            self.client
                .post(DO_LOGIN_BY_TOKEN)
                .form(&body)
                .send()
                .await?
        } else {
            let t = gen_random_fake_md5();
            body.push(("token", &t));
            self.client
                .post(DO_LOGIN_BY_TOKEN)
                .form(&body)
                .send()
                .await?
        };

        check_response(&mut resp).await?;

        let buf = resp.bytes().await?;

        let resp: CommonResponse<LoginInfo> = match serde_json::from_slice(buf.as_ref()) {
            Ok(v) => v,
            Err(e) => return Err(Error::Deserialize(e, buf)),
        };

        if check_auth_status(&resp)? {
            return Err(Error::Runtime(format!(
                "Login error: ({}); {}",
                resp.status_code,
                resp.message.unwrap_or_default(),
            )));
        }

        if let Some(v) = resp.data {
            Ok(v)
        } else {
            Err(Error::EmptyResp)
        }
    }

    /// Get the public key used to encrypt the password
    pub async fn public_key(&self) -> Result<String> {
        let body = self.req_body();

        let mut resp = self.client.post(GET_PUBLIC_KEY).form(&body).send().await?;

        check_response(&mut resp).await?;

        let buf = resp.bytes().await?;

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct PublicKey {
            public_key: String,
        }

        let resp: BasicResponse<PublicKey> = match serde_json::from_slice(buf.as_ref()) {
            Ok(v) => v,
            Err(e) => return Err(Error::Deserialize(e, buf)),
        };

        if !resp.success {
            return Err(Error::Runtime(format!(
                "Get public key error: ({}); {}",
                resp.status_code, resp.message
            )));
        }

        if let Some(v) = resp.data {
            Ok(v.public_key)
        } else {
            Err(Error::EmptyResp)
        }
    }

    /// Do login by password
    ///
    /// Only work on same device (by using same `deviceId`)
    ///
    /// Get the public key by [`Self::public_key`]
    ///
    /// The function will encrypt the password by [`crate::utils::encrypt_password`]
    ///
    /// ```no_run
    /// use yxy::bind::campus::login::*;
    /// use yxy::error::Error;
    /// # async fn run() -> Result<(), Error> {
    /// // Use the UUID of last login device
    /// let handler = LoginHandler::build("d3ae7e7e-9c98-4498-beda-78e9e342a389".to_string())?;
    /// let public_key = handler.public_key().await?;
    /// let login_info = handler.do_login_by_password("18888888888", "password", &public_key).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn login_by_password(
        &self,
        phone_num: &str,
        password: &str,
        public_key: &str,
    ) -> Result<LoginInfo> {
        let mut body = self.req_body();

        let encrypted_password = crate::utils::encrypt_password(password, public_key)?;

        body.push(("clientId", super::CLIENT_ID));
        body.push(("mobilePhone", phone_num));
        body.push(("osType", super::OS_TYPE));
        body.push(("osUuid", &self.device_id));
        body.push(("osVersion", super::OS_VERSION));
        body.push(("password", &encrypted_password));

        let mut resp = self.client.post(DO_LOGIN_BY_PWD).form(&body).send().await?;
        check_response(&mut resp).await?;

        let buf = resp.bytes().await?;

        let resp: BasicResponse<LoginInfo> = match serde_json::from_slice(buf.as_ref()) {
            Ok(v) => v,
            Err(e) => return Err(Error::Deserialize(e, buf)),
        };

        if !resp.success {
            if resp.message.starts_with(error_messages::WRONG_SECRET) {
                return Err(Error::BadLoginSecret);
            } else if resp.message == error_messages::DEVICE_CHANGED {
                return Err(Error::AuthDeviceChanged);
            }

            return Err(Error::Runtime(format!(
                "Login error: ({}); {}",
                resp.status_code, resp.message
            )));
        }

        if let Some(v) = resp.data {
            Ok(v)
        } else {
            Err(Error::EmptyResp)
        }
    }
}

/// Generate app security token
///
/// `appSecurityToken` is the device id encrypted with `AES`.
pub fn app_security_token(security_token: &str, device_id: &str) -> Result<String> {
    let key = GenericArray::clone_from_slice(security_token[..16].as_bytes());
    let cipher = Aes128::new(&key);

    let text = base64::decode(security_token[32..].as_bytes())?;

    let mut blocks = Vec::new();
    (0..text.len()).step_by(16).for_each(|x| {
        blocks.push(GenericArray::clone_from_slice(text[x..x + 16].as_ref()));
    });

    cipher.decrypt_blocks(&mut blocks);

    let t: Vec<u8> = blocks.iter().flatten().map(|&x| x as u8).collect();

    let last = *t.last().unwrap();
    let index: usize = t.len() - usize::from(last);
    let t_final: String = t[..index].iter().map(|&x| x as char).collect();

    let time_stamp = chrono::prelude::Local::now().timestamp();

    let stage_1 = md5(format!(
        "{}|YUNMA_APP|{}|{}|{}",
        device_id,
        t_final,
        time_stamp,
        super::APP_VER_NAME
    ))
    .to_uppercase();

    let stage_2 = md5(stage_1).to_uppercase();

    let stage_3 = format!(
        "{}|YUNMA_APP|{}|{}|{}|{}",
        device_id,
        t_final,
        time_stamp,
        super::APP_VER_NAME,
        stage_2
    );

    let padded_text = pkcs7_padding(&stage_3, 16);

    let mut blocks_2 = Vec::new();
    (0..padded_text.len()).step_by(16).for_each(|x| {
        blocks_2.push(GenericArray::clone_from_slice(
            padded_text[x..x + 16].as_ref(),
        ));
    });

    cipher.encrypt_blocks(&mut blocks_2);

    let encrypted_text: Vec<u8> = blocks_2.iter().flatten().map(|&x| x as u8).collect();

    let stage_4 = base64::encode(encrypted_text);

    Ok(stage_4)
}

/// Random yunma style device id generator
///
/// ```text
/// yunmaf0a1f70b83774ecf94b2e94900b6cefb
/// ```
pub fn gen_device_id() -> String {
    let mut uuid = uuid::Uuid::new_v4().simple().to_string();
    uuid.insert_str(0, "yunma");

    uuid
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_app_security_token() -> Result<()> {
        let result = app_security_token(
            "ce295733862b93cb376efef661c21b4dEW6CpH8wFHp/RvViKZiJ8A==",
            "12345678",
        )?;
        assert_eq!("RxTdUD90Eg91tGZHyhTKwjX9v3fH8WWGgQ3vQ5CuiC", &result[..42]);

        Ok(())
    }

    #[test]
    fn test_gen_device_id() {
        let id = gen_device_id();
        assert_eq!(id.len(), 37);
        assert!(id.starts_with("yunma"));
    }
}

// =====================
// ====== Models ======
// =====================

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BasicResponse<T> {
    pub status_code: i32,
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityTokenInfo {
    /// Level 0: No captcha required.
    pub level: u8,
    pub security_token: String,
}

/// Login result
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfo {
    /// UID
    pub id: String,
    /// App session token
    pub token: String,
    pub account: String,
    pub account_encrypt: String,
    pub mobile_phone: String,
    /// 1 as male, 0 as female
    pub sex: Option<i8>,
    pub school_code: Option<String>,
    pub school_name: Option<String>,
    pub qrcode_pay_type: Option<u8>,
    pub user_name: Option<String>,
    pub user_type: Option<String>,
    pub job_no: Option<String>,
    pub user_idcard: Option<String>,
    pub identity_no: Option<String>,
    pub user_class: Option<String>,
    pub real_name_status: i32,
    /// register time
    pub regiser_time: Option<String>,
    pub bind_card_status: i32,
    pub last_login: String,
    pub head_img: String,
    pub device_id: String,
    pub test_account: i32,
    pub join_newactivity_status: i32,
    pub is_new: Option<i8>,
    pub create_status: i32,
    pub eacct_status: i32,
    pub school_classes: Option<i32>,
    pub school_nature: Option<i32>,
    pub platform: String,
    /// Unknown usage
    pub uu_token: String,
    pub qrcode_private_key: String,
    pub bind_card_rate: Option<i32>,
    pub points: Option<i32>,
    pub school_identity_type: Option<i32>,
    pub alumni_flag: Option<i32>,
    /// Some json extensions
    pub ext_json: Option<String>,
}

/// Define login error response messages
mod error_messages {
    pub const WRONG_SECRET: &str = "您已输错";
    pub const BAD_PHONE_NUM: &str = "请输入正确的手机号";
    pub const BAD_PHONE_NUM_FORMAT: &str = "手机号码格式错误";
    pub const TOO_FREQUENT: &str = "经过你的";
    pub const TOO_MANY_TRIES: &str = "发送超限，请明天再来";
    pub const FLOW_CONTROL: &str = "触发号码天级流控";
    pub const DEVICE_CHANGED: &str = "设备已更换";
    pub const VERIFICATION_EXPIRED: &str = "验证码已失效";
    pub const BAD_TOKEN: &str = "token无效";
}
