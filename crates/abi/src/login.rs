//! App Login

use super::*;

use yxy::LoginHandler;

extern_c_destructor!(LoginHandler);

/// ## Safety
/// C-ABI usage only
#[no_mangle]
pub unsafe extern "C" fn new_login_handler() -> *mut LoginHandler {
    let handler = match LoginHandler::new() {
        Ok(handler) => Box::new(handler),
        Err(_) => return std::ptr::null_mut(),
    };
    Box::into_raw(handler)
}

/// Build [`LoginHandler`] by provided `device_id`
///
/// ## Safety
/// C-ABI usage only
#[no_mangle]
pub unsafe extern "C" fn build_login_handler(device_id: *mut c_char) -> *mut LoginHandler {
    check_null_return_null!(device_id);

    let device_id = c_string_to_str(device_id);
    let handler = match LoginHandler::build(device_id) {
        Ok(handler) => Box::new(handler),
        Err(_) => return std::ptr::null_mut(),
    };
    Box::into_raw(handler)
}

/// Security token result
///
/// - `token: *mut c_char`: token c-string
/// - `level: c_int`: security level, 0: no captcha required, 1: captcha required
#[repr(C)]
#[derive(Destruct)]
pub struct SecurityToken {
    pub level: c_int,       // 0: no captcha required, 1: captcha required
    pub token: *mut c_char, // c-string of token
}

extern_c_destructor!(SecurityToken);

/// Get security token
///
/// ## Safety
/// C-ABI usage only
#[no_mangle]
pub unsafe extern "C" fn get_security_token(handler: *const LoginHandler) -> *mut SecurityToken {
    check_null_return_null!(handler);

    let handler = &*handler;
    let token = match handler.get_security_token() {
        Ok(token) => token,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null_mut();
        }
    };
    let token = Box::new(SecurityToken {
        level: token.level as c_int,
        token: CString::new(token.security_token)
            .unwrap_or_default()
            .into_raw(),
    });
    Box::into_raw(token)
}

/// Get captcha image
///
/// ## Safety
/// C-ABI usage only
#[no_mangle]
pub unsafe extern "C" fn get_captcha_image(
    handler: *const LoginHandler,
    security_token: *const c_char,
) -> *mut c_char {
    check_null_return_null!(handler, security_token);

    let handler = &*handler;
    let token = c_string_to_str(security_token);
    let image = match handler.get_captcha_image(token) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null_mut();
        }
    };

    CString::new(image).unwrap_or_default().into_raw()
}

/// Send SMS verification code
///
/// ## Inputs
/// - `handler: *const login_handler`: Pointer of [`LoginHandler`]
/// - `security_token: *const c_char`: c-string of security token
/// - `captcha: *const c_char`: c-string of captcha.
/// If captcha input `NULL`, it means no captcha is required.
///
/// ## Returns
/// - `c_int`: `0` on success, `1` on user is not exist(registered), otherwise error code
///
/// ## Safety
/// C-ABI usage only
#[no_mangle]
pub unsafe extern "C" fn send_verification_code(
    handler: *const LoginHandler,
    phone_number: *const c_char,
    security_token: *const c_char,
    captcha: *const c_char,
) -> c_int {
    check_null_return_x!(-2, handler, phone_number, security_token);
    let handler = &*handler;

    let captcha = if captcha.is_null() {
        None
    } else {
        Some(c_string_to_str(captcha))
    };
    match handler.send_verification_code(
        c_string_to_str(phone_number),
        c_string_to_str(security_token),
        captcha,
    ) {
        Ok(v) => {
            if v {
                0 // Success
            } else {
                1 // Return 1 if user is not exist
            }
        }
        Err(e) => {
            eprintln!("{e}");
            match e {
                yxy::error::Error::BadPhoneNumber => -10, // Bad phone number
                yxy::error::Error::VerificationLimit => -11, // Too may requests, limited
                _ => -1,                                  // Return -1 on other error
            }
        }
    }
}

/// Login Info
///
/// - `token: *mut c_char`: session token c-string
/// - `bind_card_status: c_int`: 0: not, 1: yes
#[repr(C)]
#[derive(Destruct)]
pub struct LoginInfo {
    uid: *mut c_char,
    token: *mut c_char,
    device_id: *mut c_char,
    bind_card_status: c_int,
}

extern_c_destructor!(LoginInfo);

/// Do login by SMS verification code
///
/// ## Safety
/// C-ABI usage only
#[no_mangle]
pub unsafe extern "C" fn do_login_by_code(
    handler: *const LoginHandler,
    phone_number: *const c_char,
    code: *const c_char,
) -> *mut LoginInfo {
    check_null_return_null!(handler, phone_number, code);

    let handler = &*handler;
    let info = match handler.do_login_by_code(c_string_to_str(phone_number), c_string_to_str(code))
    {
        Ok(info) => info,
        Err(e) => {
            eprintln!("{e}");
            return std::ptr::null_mut();
        }
    };
    let info = Box::new(LoginInfo {
        uid: CString::new(info.id).unwrap_or_default().into_raw(),
        token: CString::new(info.token).unwrap_or_default().into_raw(),
        device_id: CString::new(info.device_id).unwrap_or_default().into_raw(),
        bind_card_status: info.bind_card_status as c_int,
    });
    Box::into_raw(info)
}
