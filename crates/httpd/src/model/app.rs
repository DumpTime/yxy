use serde::{Deserialize, Serialize};

pub mod auth {
    use yxy::UserInfo;

    use super::*;

    #[derive(Deserialize)]
    pub struct Request {
        pub uid: String,
    }

    #[derive(Serialize)]
    pub struct Response {
        /// Session Token
        pub token: String,
        /// UID
        pub id: String,
        pub mobile_phone: String,
        pub sex: i8,
        pub platform: String,
        pub third_openid: String,
        pub school_code: Option<String>,
        pub school_name: Option<String>,
        pub user_name: Option<String>,
        pub user_type: Option<String>,
        pub job_no: Option<String>,
        pub user_id_card: Option<String>,
        pub user_class: Option<String>,
        pub bind_card_status: Option<i8>,
    }

    impl From<(String, UserInfo)> for Response {
        fn from((t, v): (String, UserInfo)) -> Self {
            Self {
                token: t,
                id: v.id,
                mobile_phone: v.mobile_phone,
                sex: v.sex,
                platform: v.platform,
                third_openid: v.third_openid,
                school_code: v.school_code,
                school_name: v.school_name,
                user_name: v.user_name,
                user_type: v.user_type,
                job_no: v.job_no,
                user_id_card: v.user_idcard,
                user_class: v.user_class,
                bind_card_status: v.bind_card_status,
            }
        }
    }
}

pub mod electricity {
    use super::*;

    pub mod subsidy {
        use yxy::{RoomInfo, SurplusInfo};

        use super::*;

        #[derive(Deserialize)]
        pub struct RoomInfoRequest {
            pub token: String,
            pub area_id: String,
            pub building_code: String,
            pub floor_code: String,
            pub room_code: String,
        }

        impl RoomInfoRequest {
            pub fn split(self) -> (String, RoomInfo) {
                (
                    self.token,
                    RoomInfo {
                        area_id: self.area_id,
                        building_code: self.building_code,
                        floor_code: self.floor_code,
                        room_code: self.room_code,
                    },
                )
            }
        }

        #[derive(Deserialize)]
        pub struct TokenRequest {
            /// Session Token
            pub token: String,
        }

        #[derive(Serialize)]
        pub struct Response {
            pub school_code: String,
            pub area_id: String,
            pub building_code: String,
            pub floor_code: String,
            pub room_code: String,
            /// Room name
            pub display_room_name: String,
            /// Total surplus
            pub soc: f32,
            /// Total surplus amount
            pub soc_amount: f32,

            pub surplus: Option<f32>,
            pub surplus_amount: Option<f32>,
            pub subsidy: Option<f32>,
            pub subsidy_amount: Option<f32>,

            /// Type
            pub md_type: Option<String>,
            pub md_name: Option<String>,
            pub room_status: Option<String>,
        }

        impl From<SurplusInfo> for Response {
            fn from(mut v: SurplusInfo) -> Self {
                if let Some(s) = v.surplus_list.pop() {
                    Self {
                        school_code: v.school_code,
                        area_id: v.area_id,
                        building_code: v.building_code,
                        floor_code: v.floor_code,
                        room_code: v.room_code,
                        display_room_name: v.display_room_name,
                        soc: v.soc,
                        soc_amount: v.total_soc_amount,
                        surplus: Some(s.surplus),
                        surplus_amount: Some(s.amount),
                        subsidy: Some(s.subsidy),
                        subsidy_amount: Some(s.subsidy_amount),
                        md_type: Some(s.mdtype),
                        md_name: Some(s.mdname),
                        room_status: Some(s.room_status),
                    }
                } else {
                    Self {
                        school_code: v.school_code,
                        area_id: v.area_id,
                        building_code: v.building_code,
                        floor_code: v.floor_code,
                        room_code: v.room_code,
                        display_room_name: v.display_room_name,
                        soc: v.soc,
                        soc_amount: v.total_soc_amount,
                        surplus: None,
                        surplus_amount: None,
                        subsidy: None,
                        subsidy_amount: None,
                        md_type: None,
                        md_name: None,
                        room_status: None,
                    }
                }
            }
        }
    }

    pub mod bind {
        use yxy::BindInfo;

        use super::*;

        #[derive(Deserialize)]
        pub struct Request {
            /// Session Token
            pub token: String,
        }

        #[derive(Serialize)]
        pub struct Response {
            pub id: String,
            pub school_code: String,
            pub school_name: String,
            pub area_id: String,
            pub area_name: String,
            pub building_code: String,
            pub building_name: String,
            pub floor_code: String,
            pub floor_name: String,
            pub room_code: String,
            pub room_name: String,
            pub bind_type: String,
            pub create_time: String,
        }

        impl From<BindInfo> for Response {
            fn from(v: BindInfo) -> Self {
                Self {
                    id: v.id,
                    school_code: v.school_code,
                    school_name: v.school_name,
                    area_id: v.area_id,
                    area_name: v.area_name,
                    building_code: v.building_code,
                    building_name: v.building_name,
                    floor_code: v.floor_code,
                    floor_name: v.floor_name,
                    room_code: v.room_code,
                    room_name: v.room_name,
                    bind_type: v.bind_type_str,
                    create_time: v.create_time,
                }
            }
        }
    }

    pub mod consumption {
        use yxy::{RoomInfo, UsageRecord};

        use super::*;

        #[derive(Deserialize)]
        pub struct Request {
            /// Session Token
            pub token: String,
            pub area_id: String,
            pub building_code: String,
            pub floor_code: String,
            pub room_code: String,
            pub md_type: String,
        }

        #[derive(Serialize)]
        pub struct Response(Vec<Record>);

        #[derive(Serialize)]
        pub struct Record {
            pub room_dm: String,
            pub datetime: String,
            pub used: String,
        }

        impl Request {
            pub fn split(self) -> (String, String, RoomInfo) {
                (
                    self.token,
                    self.md_type,
                    RoomInfo {
                        area_id: self.area_id,
                        building_code: self.building_code,
                        floor_code: self.floor_code,
                        room_code: self.room_code,
                    },
                )
            }
        }

        impl From<UsageRecord> for Record {
            fn from(v: UsageRecord) -> Self {
                Self {
                    room_dm: v.roomdm,
                    datetime: v.datetime,
                    used: v.used,
                }
            }
        }

        impl From<Vec<UsageRecord>> for Response {
            fn from(v: Vec<UsageRecord>) -> Self {
                Self(v.into_iter().map(Record::from).collect())
            }
        }
    }

    pub mod recharge {
        use yxy::{RechargeRecord, RoomInfo, UserRechargeRecord};

        use super::*;

        #[derive(Deserialize)]
        pub struct ByUserRequest {
            pub token: String,
            pub page: u32,
            /// Transaction creation time
            pub time: Option<String>,
        }

        #[derive(Deserialize)]
        pub struct ByRoomRequest {
            pub token: String,
            pub area_id: String,
            pub building_code: String,
            pub floor_code: String,
            pub room_code: String,
            pub page: u32,
        }

        impl ByRoomRequest {
            pub fn split(self) -> (String, u32, RoomInfo) {
                (
                    self.token,
                    self.page,
                    RoomInfo {
                        area_id: self.area_id,
                        building_code: self.building_code,
                        floor_code: self.floor_code,
                        room_code: self.room_code,
                    },
                )
            }
        }

        #[derive(Serialize)]
        pub struct ByRoomResponse(Vec<RoomRecord>);

        #[derive(Serialize)]
        pub struct ByUserResponse {
            records: Vec<UserRecord>,
            months: String,
            total_transaction_money: String,
            total_real_money: String,
            total_count: i64,
        }

        #[derive(Serialize)]
        pub struct UserRecord {
            pub id: String,
            pub order_no: String,
            pub pay_money: f64,
            pub total_money: f64,
            pub pay_no: String,
            pub pay_type: String,
            pub create_time: String,
            pub pay_status_str: String,
            pub sub_type: String,
            pub prod_name: String,
            pub pay_time: String,
            pub remark: String,
            pub fee_money: f64,
            pub week: String,
            pub day: String,
            pub month: String,
        }

        impl From<UserRechargeRecord> for UserRecord {
            fn from(v: UserRechargeRecord) -> Self {
                Self {
                    id: v.id,
                    order_no: v.order_no,
                    pay_money: v.pay_money,
                    total_money: v.total_money,
                    pay_no: v.pay_no,
                    pay_type: v.pay_type,
                    create_time: v.create_time,
                    pay_status_str: v.pay_status_str,
                    sub_type: v.sub_type,
                    prod_name: v.prod_name,
                    pay_time: v.pay_time,
                    remark: v.remark,
                    fee_money: v.fee_money,
                    week: v.week,
                    day: v.day_date,
                    month: v.month,
                }
            }
        }

        impl TryFrom<Vec<UserRechargeRecord>> for ByUserResponse {
            type Error = ();

            fn try_from(mut v: Vec<UserRechargeRecord>) -> Result<Self, ()> {
                if let Some(r) = v.pop() {
                    // Extract common data
                    let mut result = Self {
                        months: r.center_order_statistics_vo.months,
                        total_count: r.center_order_statistics_vo.total_count,
                        total_real_money: r.center_order_statistics_vo.total_real_money,
                        total_transaction_money: r.center_order_statistics_vo.total_tran_money,
                        records: Vec::<_>::new(),
                    };

                    result.records.push(UserRecord {
                        id: r.id,
                        order_no: r.order_no,
                        pay_money: r.pay_money,
                        pay_type: r.pay_type,
                        total_money: r.total_money,
                        pay_no: r.pay_no,
                        create_time: r.create_time,
                        pay_status_str: r.pay_status_str,
                        sub_type: r.sub_type,
                        prod_name: r.prod_name,
                        pay_time: r.pay_time,
                        remark: r.remark,
                        fee_money: r.fee_money,
                        week: r.week,
                        day: r.day_date,
                        month: r.month,
                    });

                    result
                        .records
                        .append(&mut v.into_iter().map(UserRecord::from).collect());

                    Ok(result)
                } else {
                    Err(())
                }
            }
        }

        #[derive(Serialize)]
        pub struct RoomRecord {
            pub room_dm: String,
            pub datetime: String,
            pub using_type: String,
            pub money: String,
            pub is_send: String,
        }

        impl From<RechargeRecord> for RoomRecord {
            fn from(v: RechargeRecord) -> Self {
                Self {
                    room_dm: v.roomdm,
                    datetime: v.datetime,
                    using_type: v.buyusingtpe,
                    money: v.money,
                    is_send: v.issend,
                }
            }
        }

        impl From<Vec<RechargeRecord>> for ByRoomResponse {
            fn from(v: Vec<RechargeRecord>) -> Self {
                Self(v.into_iter().map(RoomRecord::from).collect())
            }
        }
    }
}
