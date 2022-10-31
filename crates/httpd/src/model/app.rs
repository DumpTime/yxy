use serde::{Deserialize, Serialize};

pub mod auth {
    use super::*;

    #[derive(Deserialize)]
    pub struct Query {
        pub uid: String,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        pub token: String,
        pub user_info: yxy::UserInfo,
    }
}

pub mod electricity {
    use super::*;

    #[derive(Deserialize)]
    pub struct ByTokenQuery {
        pub token: String,
    }
}
