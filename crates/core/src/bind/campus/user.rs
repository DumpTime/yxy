//! Campus user API bindings

use super::*;
use campus::user::*;

impl CampusHandler {
    /// Query campus card balance
    ///
    /// Returns in string, like "20.01"
    pub async fn card_balance(&self) -> Result<String> {
        let body = self.req_body();

        let mut resp = self
            .client
            .post(QUERY_CARD_BALANCE)
            .form(&body)
            .send()
            .await?;
        check_response(&mut resp).await?;

        let resp: CommonResponse<String> = resp.json().await?;
        if check_auth_status(&resp)? {
            return Err(Error::Runtime(format!(
                "Fail to query card balance: ({}); {}",
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

    /// Query consumption records
    ///
    /// The `query_time` parameter expects a `yyyymmdd` style time string. e.g. "20220101"  
    pub async fn consumption_records(&self, query_time: &str) -> Result<Vec<ConsumptionRecord>> {
        let mut body = self.req_body();
        body.push(("queryTime", query_time));

        let mut resp = self
            .client
            .post(QUERY_CONSUMPTION_RECORDS)
            .form(&body)
            .send()
            .await?;
        check_response(&mut resp).await?;

        let resp: CommonResponse<(), Vec<ConsumptionRecord>> = resp.json().await?;
        if check_auth_status(&resp)? {
            return Err(Error::Runtime(format!(
                "Fail to query consumption records: ({}); {}",
                resp.status_code,
                resp.message.unwrap_or_default(),
            )));
        }

        if let Some(v) = resp.rows {
            if v.is_empty() {
                return Err(Error::EmptyResp);
            }
            Ok(v)
        } else {
            Err(Error::EmptyResp)
        }
    }
}

// ================
// ==== Models ====
// ================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsumptionRecord {
    #[serde(rename = "type")]
    pub row_type: String,
    pub time: String,
    pub serialno: String,
    pub fee_name: String,
    pub money: String,
    pub dealtime: String,
    pub address: String,
}
