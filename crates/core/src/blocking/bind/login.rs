//! Campus(YSchool) app login

use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::io::Read;

use super::{check_response, url};
use crate::error::Error;
use crate::utils::{md5, pkcs7_padding};

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
}

/// Handle of login procedure
pub struct LoginHandler {
    pub device_id: String,

    client: Client,
}

impl LoginHandler {
    /// Create handler with generated UUID in place of `device_id`
    pub fn new() -> Result<Self, Error> {
        let device_id = uuid::Uuid::new_v4().to_string().to_uppercase();

        Ok(Self {
            client: init_app_sim_client(&device_id)?,
            device_id,
        })
    }

    /// Init handler by specific device id
    pub fn build(device_id: &str) -> Result<Self, Error> {
        Ok(Self {
            device_id: device_id.to_string(),
            client: init_app_sim_client(device_id)?,
        })
    }

    /// Init general request body
    pub fn basic_request_body(&self) -> Vec<(&str, &str)> {
        vec![
            ("appVersion", super::APP_VER),
            ("deviceId", &self.device_id),
            ("platform", super::PLATFORM),
            ("testAccount", "1"),
        ]
    }

    /// Return security token & level
    pub fn get_security_token(&self) -> Result<SecurityTokenInfo, Error> {
        let body = self.basic_request_body();

        let mut resp = self
            .client
            .post(url::campus::GET_SECURITY_TOKEN)
            .form(&body)
            .send()?;
        check_response(&mut resp)?;

        let resp_ser: BasicResponse<SecurityTokenInfo> = resp.json()?;
        if !resp_ser.success {
            return Err(Error::Runtime(format!(
                "Get security token failed: {}",
                resp_ser.message
            )));
        }

        match resp_ser.data {
            Some(v) => Ok(v),
            None => Err(Error::EmptyResp),
        }
    }

    /// Get image captcha
    ///
    /// Return image captcha base64 string
    pub fn get_captcha_image(&self, security_token: &str) -> Result<String, Error> {
        let mut body = self.basic_request_body();
        body.push(("securityToken", security_token));

        let mut resp = self
            .client
            .post(url::campus::GET_IMAGE_CAPTCHA)
            .form(&body)
            .send()?;
        check_response(&mut resp)?;

        let resp_ser: BasicResponse<String> = resp.json()?;
        if !resp_ser.success {
            Err(Error::Runtime(format!(
                "Get image captcha failed: {}",
                resp_ser.message
            )))
        } else {
            Ok(resp_ser.data.unwrap())
        }
    }

    /// Request to send login verification code SMS
    pub fn send_verification_code(
        &self,
        phone_num: &str,
        security_token: &str,
        captcha: Option<&str>,
    ) -> Result<bool, Error> {
        let mut body = self.basic_request_body();

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
            .post(url::campus::SEND_VERIFICATION_CODE)
            .form(&body)
            .send()?;
        check_response(&mut resp)?;

        /// Define data object
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Data {
            user_exists: bool,
        }

        let resp_ser: BasicResponse<Data> = resp.json()?;
        if !resp_ser.success {
            if resp_ser.status_code == 203 {
                if resp_ser.message == error_messages::BAD_PHONE_NUM
                    || resp_ser.message == error_messages::BAD_PHONE_NUM_FORMAT
                {
                    return Err(Error::BadPhoneNumber);
                }
                if resp_ser.message.starts_with(error_messages::TOO_FREQUENT)
                    || resp_ser.message == error_messages::FLOW_CONTROL
                    || resp_ser.message == error_messages::TOO_MANY_TRIES
                {
                    return Err(Error::Limited);
                }
            }

            return Err(Error::Runtime(format!(
                "Send verification code error: {{code: {}, message: {}}}",
                resp_ser.status_code, resp_ser.message
            )));
        }

        // User status
        let user_exists = resp_ser.data.unwrap().user_exists;

        Ok(user_exists)
    }

    /// Do login by verification code
    ///
    /// return [`LoginInfo`]
    pub fn do_login_by_code(&self, phone_num: &str, code: &str) -> Result<LoginInfo, Error> {
        let mut body = self.basic_request_body();
        body.push(("clientId", super::CLIENT_ID));
        body.push(("mobilePhone", phone_num));
        body.push(("osType", super::OS_TYPE));
        body.push(("osUuid", &self.device_id));
        body.push(("osVersion", super::OS_VERSION));
        body.push(("verificationCode", code));

        let mut resp = self
            .client
            .post(url::campus::DO_LOGIN_BY_CODE)
            .form(&body)
            .send()?;
        check_response(&mut resp)?;

        let mut buf = String::new();
        resp.read_to_string(&mut buf)?;
        // println!("resp: {}", buf);

        let resp_ser: BasicResponse<LoginInfo> = match serde_json::from_str(&buf) {
            Ok(v) => v,
            Err(e) => {
                return Err(Error::Runtime(format!(
                    "Parsing login response failed: {}\nData: {}",
                    e, buf
                )))
            }
        };

        if !resp_ser.success {
            if resp_ser.message.starts_with(error_messages::WRONG_SECRET) {
                return Err(Error::BadLoginSecret);
            }

            return Err(Error::Runtime(format!(
                "Login error: {{code: {}, msg: {}}}",
                resp_ser.status_code, resp_ser.message
            )));
        }
        let result = resp_ser.data.unwrap();

        Ok(result)
    }

    /// Do login by session token
    ///
    /// Bind to [`crate::url::campus::DO_LOGIN_BY_TOKEN`]
    ///
    /// Used to refresh token and get [`LoginInfo`]
    pub fn do_login_by_token(&self, uid: &str, token: &str) -> Result<LoginInfo, Error> {
        let mut body = self.basic_request_body();
        body.push(("clientId", super::CLIENT_ID));
        body.push(("osType", super::OS_TYPE));
        body.push(("osUuid", &self.device_id));
        body.push(("osVersion", super::OS_VERSION));
        body.push(("token", token));
        body.push(("ymId", uid));

        let mut resp = self
            .client
            .post(url::campus::DO_LOGIN_BY_TOKEN)
            .form(&body)
            .send()?;
        check_response(&mut resp)?;

        let mut buf = String::new();
        resp.read_to_string(&mut buf)?;

        let resp_ser: BasicResponse<LoginInfo> = match serde_json::from_str(&buf) {
            Ok(v) => v,
            Err(e) => {
                return Err(Error::Runtime(format!(
                    "Parsing login response failed: {}\nData: {}",
                    e, buf
                )))
            }
        };

        if !resp_ser.success {
            if resp_ser.message.starts_with(error_messages::WRONG_SECRET) {
                return Err(Error::BadLoginSecret);
            }

            return Err(Error::Runtime(format!(
                "Login error: {{code: {}, msg: {}}}",
                resp_ser.status_code, resp_ser.message
            )));
        }
        let result = resp_ser.data.unwrap();

        Ok(result)
    }

    /// Get the public key used to encrypt the password
    pub fn get_public_key(&self) -> Result<String, Error> {
        let body = self.basic_request_body();

        let mut resp = self
            .client
            .post(url::campus::GET_PUBLIC_KEY)
            .form(&body)
            .send()?;

        check_response(&mut resp)?;

        let mut buf = String::new();
        resp.read_to_string(&mut buf)?;

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct PublicKey {
            public_key: String,
        }

        let resp_ser: BasicResponse<PublicKey> = match serde_json::from_str(&buf) {
            Ok(v) => v,
            Err(e) => {
                return Err(Error::Runtime(format!(
                    "Parsing public key response failed: {}\nData: {}",
                    e, buf
                )))
            }
        };

        if !resp_ser.success {
            return Err(Error::Runtime(format!(
                "Get public key error: {{code: {}, msg: {}}}",
                resp_ser.status_code, resp_ser.message
            )));
        }

        Ok(resp_ser.data.unwrap().public_key)
    }

    /// Do login by password
    ///
    /// Only work on same device (by using same `deviceId`)
    ///
    /// Get the public key by [`Self::get_public_key`]
    ///
    /// The function will encrypt the password by [`crate::utils::encrypt_password`]
    ///
    /// ```no_run
    /// use yxy::blocking::bind::login::*;
    /// use yxy::error::Error;
    /// # fn main() -> Result<(), Error> {
    /// // Use the UUID of last login device
    /// let handler = LoginHandler::build("d3ae7e7e-9c98-4498-beda-78e9e342a389")?;
    /// let public_key = handler.get_public_key()?;
    /// let login_info = handler.do_login_by_password("18888888888", "password", &public_key)?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn do_login_by_password(
        &self,
        phone_num: &str,
        password: &str,
        public_key: &str,
    ) -> Result<LoginInfo, Error> {
        let mut body = self.basic_request_body();

        let encrypted_password = crate::utils::encrypt_password(password, public_key)?;

        body.push(("clientId", super::CLIENT_ID));
        body.push(("mobilePhone", phone_num));
        body.push(("osType", super::OS_TYPE));
        body.push(("osUuid", &self.device_id));
        body.push(("osVersion", super::OS_VERSION));
        body.push(("password", &encrypted_password));

        let mut resp = self
            .client
            .post(url::campus::DO_LOGIN_BY_PWD)
            .form(&body)
            .send()?;
        check_response(&mut resp)?;

        let mut buf = String::new();
        resp.read_to_string(&mut buf)?;

        let resp_ser: BasicResponse<LoginInfo> = match serde_json::from_str(&buf) {
            Ok(v) => v,
            Err(e) => {
                return Err(Error::Runtime(format!(
                    "Parsing login response failed: {}\nData: {}",
                    e, buf
                )))
            }
        };

        if !resp_ser.success {
            if resp_ser.message.starts_with(error_messages::WRONG_SECRET) {
                return Err(Error::BadLoginSecret);
            } else if resp_ser.message == error_messages::DEVICE_CHANGED {
                return Err(Error::DeviceChanged);
            }

            return Err(Error::Runtime(format!(
                "Login error: {{code: {}, msg: {}}}",
                resp_ser.status_code, resp_ser.message
            )));
        }
        let result = resp_ser.data.unwrap();

        Ok(result)
    }
}

/// Init App simulated client
///
/// ## Contains
/// - [`reqwest::blocking::Client`]
/// - 5s timeout
/// - UA header
pub fn init_app_sim_client(device_id: &str) -> Result<reqwest::blocking::Client, Error> {
    let builder: reqwest::blocking::ClientBuilder = reqwest::blocking::Client::builder();

    let result: reqwest::blocking::Client = builder
        .connect_timeout(std::time::Duration::new(5, 0))
        .user_agent(format!("{}{}", super::USER_AGENT, device_id))
        .build()?;

    Ok(result)
}

/// Generate app security token
///
/// `appSecurityToken` is the device id encrypted with `AES`.
pub fn app_security_token(security_token: &str, device_id: &str) -> Result<String, Error> {
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
    use crate::error::Error;

    #[test]
    fn test_app_security_token() -> Result<(), Error> {
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
