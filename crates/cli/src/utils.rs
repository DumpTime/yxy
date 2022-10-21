//! Some utils

use reqwest::blocking::Client;
use serde::Deserialize;

const SERVER_CHAN: &str = "https://sctapi.ftqq.com/";

pub fn push_message(
    key: &str,
    title: &str,
    desp: &str,
) -> Result<(String, String), yxy::error::Error> {
    let client = Client::new();
    let resp = client
        .post(&format!("{}{}.send", SERVER_CHAN, key))
        .form(&vec![("title", title), ("desp", desp)])
        .send()?;

    #[derive(Debug, Deserialize)]
    struct Response {
        code: i32,
        message: String,
        data: Option<Data>,
    }

    #[derive(Debug, Deserialize)]
    struct Data {
        pushid: String,
        readkey: String,
    }

    let resp_ser: Response = resp.json()?;
    if resp_ser.code != 0 {
        return Err(yxy::error::Error::Runtime(format!(
            "ServerChan message push failed: {}",
            resp_ser.message
        )));
    }

    match resp_ser.data {
        Some(v) => Ok((v.pushid, v.readkey)),
        None => Err(yxy::error::Error::EmptyResp),
    }
}
