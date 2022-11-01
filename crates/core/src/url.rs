//! API URLs

use const_format::concatcp as cc;

pub mod auth {
    pub const BASE_URL: &str = "https://auth.xiaofubao.com";

    pub const OAUTH_URL: &str = super::cc!(BASE_URL, "/authoriz/getCodeV2");
}

/// Application URLs
pub mod application {
    pub const BASE_URL: &str = "https://application.xiaofubao.com";

    /// Auth & user info
    /// - Authorize by OAuth code
    /// - Get user info by UID and session
    pub const GET_USER_FOR_AUTHORIZE: &str = super::cc!(BASE_URL, "/app/login/getUser4Authorize");

    pub const QUERY_BIND: &str = super::cc!(BASE_URL, "/app/electric/queryBind");

    pub const QUERY_ELECTRICITY: &str = super::cc!(BASE_URL, "/app/electric/queryISIMSRoomSurplus");

    pub const QUERY_MY_RECHARGE_RECORDS: &str =
        super::cc!(BASE_URL, "/app/order/bussisdw/queryListData");

    pub const RECHARGE_ELECTRICITY: &str = super::cc!(BASE_URL, "/app/electric/recharge.htm");

    pub const GET_SUBMIT_TOKEN: &str = super::cc!(BASE_URL, "/center/common/token/get.htm");
}

/// Campus APP API URLs
pub mod campus {
    /// *may be a typo*
    pub const BASE_URL: &str = "https://compus.xiaofubao.com";

    pub const GET_SECURITY_TOKEN: &str = super::cc!(BASE_URL, "/common/security/token");

    pub const SEND_VERIFICATION_CODE: &str =
        super::cc!(BASE_URL, "/compus/user/sendLoginVerificationCode");

    pub const GET_IMAGE_CAPTCHA: &str = super::cc!(BASE_URL, "/common/security/imageCaptcha");

    pub const DO_LOGIN_BY_CODE: &str = super::cc!(BASE_URL, "/login/doLoginByVerificationCode");

    pub const DO_LOGIN_BY_TOKEN: &str = super::cc!(BASE_URL, "/login/doLoginBySilent");

    pub const GET_PUBLIC_KEY: &str = super::cc!(BASE_URL, "/login/getPublicKey");

    pub const DO_LOGIN_BY_PWD: &str = super::cc!(BASE_URL, "/login/doLoginByPwd");
}

/// Payment URLs
pub mod pay {
    pub const BASE_URL: &str = "https://pay.xiaofubao.com";

    pub const TO_CASHIER: &str = super::cc!(BASE_URL, "/pay/unified/toCashier.shtml");
}
