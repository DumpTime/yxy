use serde::{Deserialize, Serialize};

pub mod auth {
    use super::*;

    #[derive(Deserialize)]
    pub struct Query {
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

    impl Response {
        pub fn build((t, v): (String, yxy::UserInfo)) -> Self {
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
        use super::*;

        #[derive(Deserialize)]
        pub struct TokenQuery {
            /// Session Token
            pub token: String,
        }

        #[derive(Deserialize)]
        pub struct RoomInfoRequest {
            pub token: String,
            pub area_id: String,
            pub building_code: String,
            pub floor_code: String,
            pub room_code: String,
        }

        impl RoomInfoRequest {
            pub fn split(self) -> (String, yxy::RoomInfo) {
                (
                    self.token,
                    yxy::RoomInfo {
                        area_id: self.area_id,
                        building_code: self.building_code,
                        floor_code: self.floor_code,
                        room_code: self.room_code,
                    },
                )
            }
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

        impl Response {
            pub fn build(mut v: yxy::SurplusInfo) -> Self {
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
        use super::*;

        #[derive(Deserialize)]
        pub struct Query {
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

        impl Response {
            pub fn build(v: yxy::BindInfo) -> Self {
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
}
