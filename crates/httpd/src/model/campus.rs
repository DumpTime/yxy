use serde::{Deserialize, Serialize};

pub mod login {
    use super::*;

    pub mod request {
        use super::*;

        #[derive(Deserialize)]
        pub struct DeviceID {
            pub device_id: String,
        }

        #[derive(Deserialize)]
        pub struct CaptchaImage {
            pub security_token: String,
            pub device_id: String,
        }

        #[derive(Deserialize)]
        pub struct SendVerificationCode {
            pub security_token: String,
            pub device_id: String,
            pub phone_num: String,
            pub captcha: Option<String>,
        }

        #[derive(Deserialize)]
        pub struct LoginByCode {
            pub device_id: String,
            pub phone_num: String,
            pub code: String,
        }

        #[derive(Deserialize)]
        pub struct SilentLogin {
            pub device_id: String,
            pub uid: String,
            pub token: Option<String>,
        }

        #[derive(Deserialize)]
        pub struct LoginByPassword {
            pub device_id: String,
            pub phone_num: String,
            pub password: String,
            pub public_key: String,
        }
    }

    pub mod response {
        use yxy::SecurityTokenInfo;

        use super::*;

        #[derive(Serialize)]
        pub struct SecurityToken {
            pub level: u8,
            pub token: String,
        }

        impl From<SecurityTokenInfo> for SecurityToken {
            fn from(v: SecurityTokenInfo) -> Self {
                Self {
                    level: v.level,
                    token: v.security_token,
                }
            }
        }

        #[derive(Serialize)]
        pub struct CaptchaImage {
            pub img: String,
        }

        #[derive(Serialize)]
        pub struct SendVerificationCode {
            pub user_exists: bool,
        }

        #[derive(Serialize)]
        pub struct LoginInfo {
            pub uid: String,
            /// App session token
            pub token: String,
            pub device_id: String,
            /// 1 as male, 0 as female
            pub sex: Option<i8>,
            pub school_code: Option<String>,
            pub school_name: Option<String>,
            pub school_classes: Option<i32>,
            pub school_nature: Option<i32>,
            pub user_name: Option<String>,
            pub user_type: Option<String>,
            pub job_no: Option<String>,
            pub user_idcard: Option<String>,
            pub identity_no: Option<String>,
            pub user_class: Option<String>,
            pub real_name_status: i32,
            pub regiser_time: Option<String>,
            pub bind_card_status: i32,
            pub last_login: String,
            pub test_account: i32,
            pub is_new: Option<i8>,
            pub create_status: i32,
            pub platform: String,
            pub bind_card_rate: Option<i32>,
            pub points: Option<i32>,
            pub school_identity_type: Option<i32>,
            /// Some json extensions
            pub extra_json: Option<String>,
        }

        impl From<yxy::LoginInfo> for LoginInfo {
            fn from(v: yxy::LoginInfo) -> Self {
                Self {
                    uid: v.id,
                    token: v.token,
                    device_id: v.device_id,
                    sex: v.sex,
                    school_code: v.school_code,
                    school_name: v.school_name,
                    school_classes: v.school_classes,
                    school_nature: v.school_nature,
                    user_name: v.user_name,
                    user_type: v.user_type,
                    job_no: v.job_no,
                    user_idcard: v.user_idcard,
                    identity_no: v.identity_no,
                    user_class: v.user_class,
                    real_name_status: v.real_name_status,
                    regiser_time: v.regiser_time,
                    bind_card_status: v.bind_card_status,
                    last_login: v.last_login,
                    test_account: v.test_account,
                    is_new: v.is_new,
                    create_status: v.create_status,
                    platform: v.platform,
                    bind_card_rate: v.bind_card_rate,
                    points: v.points,
                    school_identity_type: v.school_identity_type,
                    extra_json: v.ext_json,
                }
            }
        }

        #[derive(Serialize)]
        pub struct PublicKey {
            pub key: String,
        }
    }
}
