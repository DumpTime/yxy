//! API URLs

use const_format::concatcp as cc;

pub mod auth {
    use super::*;

    pub const BASE_URL: &str = "https://auth.xiaofubao.com";

    pub const OAUTH_URL: &str = cc!(BASE_URL, "/authoriz/getCodeV2");
}

/// Application URLs
pub mod application {
    use super::*;

    pub const BASE_URL: &str = "https://application.xiaofubao.com";

    /// Auth & user info
    /// - Authorize by OAuth code
    /// - Get user info by UID and session
    pub const GET_USER_FOR_AUTHORIZE: &str = cc!(BASE_URL, "/app/login/getUser4Authorize");

    pub mod electricity {
        use super::*;

        pub const QUERY_BIND: &str = cc!(BASE_URL, "/app/electric/queryBind");

        pub const QUERY_SURPLUS: &str = cc!(BASE_URL, "/app/electric/queryISIMSRoomSurplus");

        pub const QUERY_USAGE_RECORDS: &str = cc!(BASE_URL, "/app/electric/getISIMSRecords");

        pub const QUERY_USER_RECHARGE_RECORDS: &str =
            cc!(BASE_URL, "/app/order/bussisdw/queryListData");

        pub const QUERY_ROOM_RECHARGE_RECORDS: &str =
            cc!(BASE_URL, "/app/electric/queryISIMSRoomBuyRecord");

        pub const RECHARGE: &str = cc!(BASE_URL, "/app/electric/recharge.htm");
    }

    pub const GET_SUBMIT_TOKEN: &str = cc!(BASE_URL, "/center/common/token/get.htm");
}

/// Campus APP API URLs
pub mod campus {
    use super::*;

    /// *may be a typo*
    pub const BASE_URL: &str = "https://compus.xiaofubao.com";

    pub mod login {
        use super::*;

        pub const GET_SECURITY_TOKEN: &str = cc!(BASE_URL, "/common/security/token");

        pub const SEND_VERIFICATION_CODE: &str =
            cc!(BASE_URL, "/compus/user/sendLoginVerificationCode");

        pub const GET_IMAGE_CAPTCHA: &str = cc!(BASE_URL, "/common/security/imageCaptcha");

        pub const DO_LOGIN_BY_CODE: &str = cc!(BASE_URL, "/login/doLoginByVerificationCode");

        pub const DO_LOGIN_BY_TOKEN: &str = cc!(BASE_URL, "/login/doLoginBySilent");

        pub const GET_PUBLIC_KEY: &str = cc!(BASE_URL, "/login/getPublicKey");

        pub const DO_LOGIN_BY_PWD: &str = cc!(BASE_URL, "/login/doLoginByPwd");
    }

    pub mod user {
        use super::*;

        pub const QUERY_CARD_BALANCE: &str = cc!(BASE_URL, "/compus/user/getCardMoney");

        pub const QUERY_CONSUMPTION_RECORDS: &str = cc!(BASE_URL, "/compus/user/queryNo");

        pub const QUERY_TRANSACTION_RECORDS: &str =
            cc!(BASE_URL, "/routepay/route/order/queryTradePage");
    }
}

/// Payment URLs
pub mod pay {
    use super::*;

    pub const BASE_URL: &str = "https://pay.xiaofubao.com";

    pub const TO_CASHIER: &str = cc!(BASE_URL, "/pay/unified/toCashier.shtml");
}
