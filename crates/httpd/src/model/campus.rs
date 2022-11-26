use serde::{Deserialize, Serialize};

pub mod login {
    use yxy::SecurityTokenInfo;

    use super::*;

    #[derive(Deserialize)]
    pub struct DeviceIDQuery {
        pub device_id: String,
    }

    #[derive(Serialize)]
    pub struct SecurityTokenResponce {
        pub level: u8,
        pub token: String,
    }

    impl From<SecurityTokenInfo> for SecurityTokenResponce {
        fn from(v: SecurityTokenInfo) -> Self {
            Self {
                level: v.level,
                token: v.security_token,
            }
        }
    }
}
