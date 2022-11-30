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

#[derive(Deserialize)]
pub struct BasicInfo {
    pub device_id: String,
    pub token: Option<String>,
    pub uid: String,
    pub school_code: String,
}

pub mod user {
    use super::*;

    pub mod request {
        use super::*;

        #[derive(Deserialize)]
        pub struct ConsumptionRecords {
            pub device_id: String,
            pub token: Option<String>,
            pub uid: String,
            pub school_code: String,
            pub query_time: String,
        }

        #[derive(Deserialize)]
        pub struct TransactionRecords {
            pub device_id: String,
            pub token: Option<String>,
            pub uid: String,
            pub school_code: String,
            pub offset: u32,
            pub limit: u32,
        }
    }

    pub mod response {
        use yxy::bind::campus::user;

        use super::*;

        #[derive(Serialize)]
        pub struct CardBalance {
            pub balance: String,
        }

        #[derive(Serialize)]
        pub struct ConsumptionRecord {
            #[serde(rename = "type")]
            pub row_type: String,
            pub fee_name: String,
            pub time: String,
            pub serial_no: String,
            pub money: String,
            pub deal_time: String,
            pub address: String,
        }

        #[derive(Serialize)]
        pub struct ConsumptionRecords(Vec<ConsumptionRecord>);

        impl From<Vec<user::ConsumptionRecord>> for ConsumptionRecords {
            fn from(v: Vec<user::ConsumptionRecord>) -> Self {
                let collection = v
                    .into_iter()
                    .map(|x| ConsumptionRecord {
                        row_type: x.row_type,
                        fee_name: x.fee_name,
                        time: x.time,
                        serial_no: x.serialno,
                        money: x.money,
                        deal_time: x.dealtime,
                        address: x.address,
                    })
                    .collect::<Vec<ConsumptionRecord>>();

                Self(collection)
            }
        }

        #[derive(Serialize)]
        pub struct TransactionRecords {
            pub total: i64,
            pub trade_details: Vec<TransactionDetail>,
            pub trade_counts: Vec<TransactionCount>,
        }
        #[derive(Serialize)]
        pub struct TransactionDetail {
            pub tran_no: String,
            pub create_time: String,
            pub pay_time: Option<String>,
            pub tran_money: i64,
            pub prod_name: String,
            pub tran_state: i64,
            pub tran_state_name: String,
            pub refund_state: i64,
            pub refund_state_name: String,
            pub pay_name: Option<String>,
            pub week_name: String,
            pub application_id: String,
            pub real_money: i64,
        }

        #[derive(Serialize)]
        pub struct TransactionCount {
            pub count_month: String,
            pub total_num: i64,
            pub total_income_amount: i64,
            pub total_expend_amount: i64,
        }

        impl From<user::TransactionRecords> for TransactionRecords {
            fn from(v: user::TransactionRecords) -> Self {
                Self {
                    total: v.total,
                    trade_details: v
                        .trade_details
                        .into_iter()
                        .map(|x| TransactionDetail {
                            tran_no: x.tran_no,
                            create_time: x.create_time,
                            pay_time: x.pay_time,
                            tran_money: x.tran_money,
                            prod_name: x.prod_name,
                            tran_state: x.tran_state,
                            tran_state_name: x.tran_state_name,
                            refund_state: x.refund_state,
                            refund_state_name: x.refund_state_name,
                            pay_name: x.pay_name,
                            week_name: x.week_name,
                            application_id: x.application_id,
                            real_money: x.real_money,
                        })
                        .collect(),
                    trade_counts: v
                        .trade_counts
                        .into_iter()
                        .map(|x| TransactionCount {
                            count_month: x.count_month,
                            total_num: x.total_num,
                            total_income_amount: x.total_income_amount,
                            total_expend_amount: x.total_expend_amount,
                        })
                        .collect(),
                }
            }
        }
    }
}
