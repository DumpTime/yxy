//! Application Common APIs
use crate::blocking::bind::check_response;
use crate::error::Error;

use reqwest::blocking::Client;
use serde::Deserialize;

/// Get common submit token (formal)
pub fn get_submit_token(client: &Client, uid: &str) -> Result<String, Error> {
    let form = [("ymId", uid)];
    let mut res = client
        .post(crate::url::application::GET_SUBMIT_TOKEN)
        .form(&form)
        .send()?;
    check_response(&mut res)?;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        pub status_code: i32,
        pub success: bool,
        pub message: String,
        pub data: Option<String>,
    }

    let resp_ser: Response = res.json()?;

    if !resp_ser.success {
        return Err(Error::Runtime(format!(
            "Fail to get submit token ({}): {}",
            resp_ser.status_code, resp_ser.message
        )));
    }

    Ok(resp_ser.data.unwrap())
}
