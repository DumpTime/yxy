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

        let buf = resp.bytes().await?;

        let resp: CommonResponse<String> = match serde_json::from_slice(buf.as_ref()) {
            Ok(v) => v,
            Err(e) => return Err(Error::Deserialize(e, buf)),
        };

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

        let buf = resp.bytes().await?;

        let resp: CommonResponse<(), Vec<ConsumptionRecord>> =
            match serde_json::from_slice(buf.as_ref()) {
                Ok(v) => v,
                Err(e) => return Err(Error::Deserialize(e, buf)),
            };

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

    /// Qeury campus APP account transaction records
    ///
    /// Pay attention to distinguish it from [`Self::consumption_records`].
    ///
    /// Query results will be paginated (Using `offset` and `limit`).
    pub async fn transaction_records(&self, offset: u32, limit: u32) -> Result<TransactionRecords> {
        let mut body = self.req_body();
        let offset = offset.to_string();
        let limit = limit.to_string();
        body.push(("offset", &offset));
        body.push(("limit", &limit));

        let mut resp = self
            .client
            .post(QUERY_TRANSACTION_RECORDS)
            .form(&body)
            .send()
            .await?;
        check_response(&mut resp).await?;

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Response {
            pub status_code: serde_json::Value,
            pub message: String,
            pub data: Option<TransactionRecords>,
        }

        let buf = resp.bytes().await?;

        let resp: Response = match serde_json::from_slice(buf.as_ref()) {
            Ok(v) => v,
            Err(e) => return Err(Error::Deserialize(e, buf)),
        };

        match resp.status_code.as_i64() {
            Some(0) => {}
            Some(203) => return Err(Error::NoBind),
            Some(204) => return Err(Error::Auth("Unauthorized".to_string())),
            None => match resp.status_code.as_str() {
                Some("204") => return Err(Error::Auth("Unauthorized".to_string())),
                _ => {
                    return Err(Error::Runtime(format!(
                        "Fail to query transaction records: ({}); {}",
                        resp.status_code, resp.message,
                    )))
                }
            },
            _ => {
                return Err(Error::Runtime(format!(
                    "Fail to query transaction records: ({}); {}",
                    resp.status_code, resp.message,
                )))
            }
        }

        match resp.data {
            Some(v) if v.total != 0 => Ok(v),
            _ => Err(Error::EmptyResp),
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRecords {
    pub total: i64,
    pub trade_details: Vec<TransactionDetail>,
    pub trade_counts: Vec<TransactionCount>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    pub icon_url: String,
    pub real_money: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionCount {
    pub count_month: String,
    pub total_num: i64,
    pub total_income_amount: i64,
    pub total_expend_amount: i64,
}
