//! Campus user API bindings

use super::*;
use campus::user::*;

impl CampusHandler {
    pub async fn query_card_balance(&self) -> Result<String> {
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

// ================
// ==== Models ====
// ================
