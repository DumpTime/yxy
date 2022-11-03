//! Electricity APIs

use super::*;
use crate::url::application::electricity::*;

use serde::{Deserialize, Serialize};

const BIND_TYPE: &str = "3";
const SUB_TYPE: &str = "100304";
const MD_TYPE: &str = "50426";

impl AppHandler {
    /// Query Bind infos
    ///
    /// Only return one binding info from list
    pub async fn binding_info(&self) -> Result<BindInfo> {
        let form = [("bindType", BIND_TYPE)];

        let mut resp = self.client.post(QUERY_BIND).form(&form).send().await?;
        check_response(&mut resp).await?;

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Response {
            pub status_code: i32,
            pub success: bool,
            pub total: Option<u32>,
            pub message: Option<String>,
            pub rows: Option<Vec<BindInfo>>,
        }

        let resp: Response = resp.json().await?;
        if !resp.success {
            if resp.status_code == 204 {
                return Err(Error::Auth("Unauthorized".to_string()));
            }
            return Err(Error::Runtime(format!(
                "Fail to query bind: {}",
                resp.message.unwrap()
            )));
        }

        // Take data
        if let Some(mut bind_info) = resp.rows {
            Ok(match bind_info.pop() {
                Some(v) => v,
                None => return Err(Error::NoBind),
            })
        } else {
            Err(Error::NoBind)
        }
    }

    /// Query electricity info
    ///
    /// Like surplus, subsidy, amount, etc.
    pub async fn surplus(&self, info: &RoomInfo) -> Result<SurplusInfo> {
        let mut resp = self.client.post(QUERY_SURPLUS).form(&info).send().await?;
        check_response(&mut resp).await?;

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Response {
            pub status_code: i32,
            pub success: bool,
            pub message: String,
            pub data: Option<SurplusInfo>,
        }
        let resp: Response = resp.json().await?;

        if !resp.success {
            if resp.status_code == 204 {
                return Err(Error::Auth("Unauthorized".to_string()));
            }
            return Err(Error::Runtime(format!(
                "Fail to query electricity: {}",
                resp.message
            )));
        }

        if let Some(v) = resp.data {
            Ok(v)
        } else {
            Err(Error::EmptyResp)
        }
    }

    /// Query my recharge records
    ///
    /// Returns [`MyRechargeRecord`] list
    pub async fn my_recharge_records(&self, page: u32) -> Result<Vec<MyRechargeRecord>> {
        let page = page.to_string();
        let form = [("currentPage", page.as_str()), ("subType", SUB_TYPE)];

        let mut resp = self
            .client
            .post(QUERY_MY_RECHARGE_RECORDS)
            .form(&form)
            .send()
            .await?;
        check_response(&mut resp).await?;

        let buf = resp.bytes().await?;

        let resp: Response<MyRechargeRecord> = match serde_json::from_slice(buf.as_ref()) {
            Ok(v) => v,
            Err(e) => return Err(Error::Deserialize(e, buf)),
        };

        if !resp.success {
            if resp.status_code == 204 {
                return Err(Error::Auth("Unauthorized".to_string()));
            }
            return Err(Error::Runtime(format!(
                "Fail to query recharge record: {}",
                resp.status_code
            )));
        } else if resp.total == 0 {
            return Err(Error::EmptyResp);
        }

        Ok(resp.rows)
    }

    pub async fn usage_records(
        &self,
        room_info: &RoomInfo,
        md_type: Option<&str>,
    ) -> Result<Vec<UsageRecord>> {
        let form = [
            ("mdtype", md_type.unwrap_or(MD_TYPE)),
            ("areaId", &room_info.area_id),
            ("buildingCode", &room_info.building_code),
            ("floorCode", &room_info.floor_code),
            ("roomCode", &room_info.room_code),
        ];

        let mut resp = self
            .client
            .post(QUERY_USAGE_RECORDS)
            .form(&form)
            .send()
            .await?;
        check_response(&mut resp).await?;

        let buf = resp.bytes().await?;

        let resp: Response<UsageRecord> = match serde_json::from_slice(buf.as_ref()) {
            Ok(v) => v,
            Err(e) => return Err(Error::Deserialize(e, buf)),
        };

        if !resp.success {
            if resp.status_code == 204 {
                return Err(Error::Auth("Unauthorized".to_string()));
            }
            return Err(Error::Runtime(format!(
                "Fail to query recharge record: {}",
                resp.status_code
            )));
        } else if resp.total == 0 {
            return Err(Error::EmptyResp);
        }

        Ok(resp.rows)
    }

    pub async fn recharge_records(
        &self,
        page: u32,
        room_info: &RoomInfo,
    ) -> Result<Vec<RechargeRecord>> {
        let page = page.to_string();
        let form = [
            ("currentPage", page.as_str()),
            ("subType", SUB_TYPE),
            ("areaId", &room_info.area_id),
            ("buildingCode", &room_info.building_code),
            ("floorCode", &room_info.floor_code),
            ("roomCode", &room_info.room_code),
        ];

        let mut resp = self
            .client
            .post(QUERY_RECHARGE_RECORDS)
            .form(&form)
            .send()
            .await?;
        check_response(&mut resp).await?;

        let buf = resp.bytes().await?;

        let resp: Response<RechargeRecord> = match serde_json::from_slice(buf.as_ref()) {
            Ok(v) => v,
            Err(e) => return Err(Error::Deserialize(e, buf)),
        };

        if !resp.success {
            if resp.status_code == 204 {
                return Err(Error::Auth("Unauthorized".to_string()));
            }
            return Err(Error::Runtime(format!(
                "Fail to query recharge record: {}",
                resp.status_code
            )));
        } else if resp.total == 0 {
            return Err(Error::EmptyResp);
        }

        Ok(resp.rows)
    }

    /// Create recharge transaction
    ///
    /// **Returns** transaction No. You can create cashier URL by [`crate::bind::pay::to_cashier`]
    ///
    /// `cztype` and `mdname` can be found in `EleTopUpType`. `cztype` may be type id. `mdname` is type human-readable type name, and it also can be customized.
    /// Default cztype = "50426", mdname = "照明用电".
    ///
    /// `submit_token` and `uid` is useless. `submit_token` can be any string or generated by `uuid::Uuid::new_v4().simple().to_string()` or [`crate::bind::app::common::get_submit_token()`].
    pub async fn recharge(
        &self,
        info: &RoomInfo,
        amount: u32,
        cztype: &str,
        mdname: &str,
        submit_token: &str,
        uid: &str,
    ) -> Result<String> {
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
            .post(RECHARGE)
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
            .send()
            .await?;
        check_response(&mut resp).await?;

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Response {
            pub status_code: i32,
            pub success: bool,
            pub message: String,
            /// Redirect URL
            pub data: Option<String>,
        }

        let resp_ser: Response = resp.json().await?;

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

// ====================
// ====== Models ======
// ====================

/// Electricity biding information
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindInfo {
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

impl From<&BindInfo> for RoomInfo {
    /// Extract [`RoomInfo`] from [`BindInfo`]
    ///
    /// Using [`Clone`] trait
    fn from(info: &BindInfo) -> Self {
        Self {
            area_id: info.area_id.to_string(),
            building_code: info.building_code.clone(),
            floor_code: info.floor_code.clone(),
            room_code: info.room_code.clone(),
        }
    }
}

impl From<BindInfo> for RoomInfo {
    /// Extract [`crate::RoomInfo`] from [`BindInfo`]
    fn from(info: BindInfo) -> Self {
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
pub struct SurplusInfo {
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
    pub surplus_list: Vec<SurplusDetail>,
    /// Top up type, usually contains only one element
    pub top_up_type_list: Vec<EleTopUpType>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurplusDetail {
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub status_code: i64,
    pub rows: Vec<T>,
    pub total: i64,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyRechargeRecord {
    pub id: String,
    pub order_no: String,
    pub pay_money: f64,
    pub total_money: f64,
    pub pay_type: String,
    pub pay_no: String,
    pub create_time: String,
    pub pay_status_str: String,
    pub sub_type: String,
    pub prod_name: String,
    pub pay_time: String,
    pub remark: String,
    pub logo: String,
    pub fee_money: f64,
    pub week: String,
    pub day_date: String,
    pub month: String,
    #[serde(rename = "centerOrderStatisticsVO")]
    pub center_order_statistics_vo: CenterOrderStatisticsVo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CenterOrderStatisticsVo {
    pub months: String,
    pub total_tran_money: String,
    pub total_real_money: String,
    pub total_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct UsageRecord {
    pub roomdm: String,
    pub datetime: String,
    pub used: String,
}

#[derive(Debug, Deserialize)]
pub struct RechargeRecord {
    pub roomdm: String,
    pub datetime: String,
    pub buytpe: String,
    pub buyusingtpe: String,
    pub money: String,
    pub issend: String,
}
