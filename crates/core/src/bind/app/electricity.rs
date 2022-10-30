//! Electricity APIs

use super::AppHandler;
use crate::error::Error;

use serde::{Deserialize, Serialize};

/// Electricity biding information
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EleBindInfo {
    pub id: String,
    pub school_code: String,
    pub school_name: String,
    pub job_no: String,
    pub user_name: String,
    pub bind_type_str: String,
    pub area_id: String,
    pub area_name: String,
    pub building_code: String,
    pub building_name: String,
    pub floor_code: String,
    pub floor_name: String,
    pub room_code: String,
    pub room_name: String,
    pub create_time: String,
    pub is_allow_change: u8,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInfo {
    pub area_id: String,
    pub building_code: String,
    pub floor_code: String,
    pub room_code: String,
}

impl From<&EleBindInfo> for RoomInfo {
    /// Extract [`crate::RoomInfo`] from [`crate::EleBindInfo`]
    ///
    /// Using [`Clone`] trait
    fn from(info: &EleBindInfo) -> Self {
        Self {
            area_id: info.area_id.to_string(),
            building_code: info.building_code.clone(),
            floor_code: info.floor_code.clone(),
            room_code: info.room_code.clone(),
        }
    }
}

impl From<EleBindInfo> for RoomInfo {
    /// Extract [`crate::RoomInfo`] from [`crate::EleBindInfo`]
    fn from(info: EleBindInfo) -> Self {
        Self {
            area_id: info.area_id,
            building_code: info.building_code,
            floor_code: info.floor_code,
            room_code: info.room_code,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectricityInfo {
    pub school_code: String,
    pub area_id: String,
    pub building_code: String,
    pub floor_code: String,
    pub room_code: String,
    /// Room name
    pub display_room_name: String,
    /// Unknown usage
    pub remind: String,
    /// Total surplus
    pub soc: f32,
    /// Total surplus amount
    pub total_soc_amount: f32,
    pub is_allow_change: u8,
    pub show_type: u8,
    pub record_show: u8,
    pub style: u8,
    /// Surplus details, usually contains only one element
    pub surplus_list: Vec<EleSurplus>,
    /// Top up type, usually contains only one element
    pub top_up_type_list: Vec<EleTopUpType>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EleSurplus {
    pub surplus: f32,
    pub amount: f32,
    pub subsidy: f32,
    pub subsidy_amount: f32,
    pub total_surplus: f32,
    pub mdtype: String,
    pub mdname: String,
    pub room_status: String,
}

/// Type of electricity top up
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EleTopUpType {
    /// type name
    pub mdname: String,
    /// type id
    pub cztype: String,
}

impl AppHandler {
    /// Query Bind infos
    ///
    /// Only return one bind info from list
    pub fn query_electricity_binding(&self) -> Result<EleBindInfo, Error> {
        let form = [("bindType", "3")];

        let mut resp = self
            .client
            .post(crate::url::application::QUERY_BIND)
            .form(&form)
            .send()?;

        crate::bind::check_response(&mut resp)?;

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Response {
            pub status_code: i32,
            pub success: bool,
            pub total: Option<u32>,
            pub message: Option<String>,
            pub rows: Option<Vec<EleBindInfo>>,
        }

        let resp_ser: Response = resp.json()?;
        if !resp_ser.success {
            if resp_ser.status_code == 204 {
                return Err(Error::Auth("Unauthorized".to_string()));
            }
            return Err(Error::Runtime(format!(
                "Fail to query bind: {}",
                resp_ser.message.unwrap()
            )));
        }

        // Take data
        if let Some(mut bind_info) = resp_ser.rows {
            Ok(match bind_info.pop() {
                Some(v) => v,
                None => return Err(Error::NoBind),
            })
        } else {
            Err(Error::NoBind)
        }
    }

    /// Query electricity info
    pub fn query_electricity(&self, info: &RoomInfo) -> Result<ElectricityInfo, Error> {
        let mut resp = self
            .client
            .post(crate::url::application::QUERY_ELECTRICITY)
            .json(&info)
            .send()?;

        crate::bind::check_response(&mut resp)?;

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Response {
            pub status_code: i32,
            pub success: bool,
            pub message: String,
            pub data: Option<ElectricityInfo>,
        }
        let resp_ser: Response = resp.json()?;

        if !resp_ser.success {
            if resp_ser.status_code == 204 {
                return Err(Error::Auth("Unauthorized".to_string()));
            }
            return Err(Error::Runtime(format!(
                "Fail to query electricity: {}",
                resp_ser.message
            )));
        }

        if let Some(v) = resp_ser.data {
            Ok(v)
        } else {
            Err(Error::EmptyResp)
        }
    }

    /// Create recharge transaction
    ///
    /// **Returns** transaction No. You can create cashier URL by [`crate::bind::pay::to_cashier`]
    ///
    /// `cztype` and `mdname` can be found in `EleTopUpType`. `cztype` may be type id. `mdname` is type human-readable type name, and it also can be customized.
    /// Default cztype = "50426", mdname = "照明用电".
    ///
    /// `submit_token` and `uid` is useless. `submit_token` can be any string or generated by `uuid::Uuid::new_v4().simple().to_string()` or [`crate::bind::app::common::get_submit_token()`].
    pub fn recharge_electricity(
        &self,
        info: &RoomInfo,
        amount: u32,
        cztype: &str,
        mdname: &str,
        submit_token: &str,
        uid: &str,
    ) -> Result<String, Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Request<'a, 'b> {
            pub area_id: &'a str,
            pub building_code: &'a str,
            pub floor_code: &'a str,
            pub room_code: &'a str,
            pub money: u32,
            pub cztype: &'b str,
            pub mdname: &'b str,
            pub ym_id: &'b str,
            pub submit_token: &'b str,
        }

        let mut resp = self
            .client
            .post(crate::url::application::RECHARGE_ELECTRICITY)
            .form(&Request {
                area_id: &info.area_id,
                building_code: &info.building_code,
                floor_code: &info.floor_code,
                room_code: &info.room_code,
                money: amount,
                cztype,
                mdname,
                submit_token,
                ym_id: uid,
            })
            .send()?;

        crate::bind::check_response(&mut resp)?;

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Response {
            pub status_code: i32,
            pub success: bool,
            pub message: String,
            /// Redirect URL
            pub data: Option<String>,
        }

        let resp_ser: Response = resp.json()?;

        if !resp_ser.success {
            if resp_ser.status_code == 204 {
                return Err(Error::Auth("Unauthorized".to_string()));
            }
            return Err(Error::Runtime(format!(
                "Fail to recharge electricity: {}",
                resp_ser.message
            )));
        }

        let callback_url = reqwest::Url::parse(&resp_ser.data.unwrap()).unwrap();

        for (key, value) in callback_url.query_pairs() {
            if key == "tran_no" {
                return Ok(value.to_string());
            }
        }

        Err(Error::EmptyResp)
    }
}
