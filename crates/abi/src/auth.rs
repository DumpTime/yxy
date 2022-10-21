//! Authorize & AppHandler

use super::*;

use yxy::AppHandler;

extern_c_destructor!(AppHandler);

/// Application Authorization
///
/// # Inputs
/// - `uid: *const c_char`: uid c-string, UTF-8
/// # Returns
/// - `*mut c_char`: token c-string, UTF-8. Return nullptr on error.
///
/// # Safety
/// C-ABI usage only
#[no_mangle]
pub unsafe extern "C" fn app_auth(uid: *const c_char) -> *mut c_char {
    check_null_return_null!(uid);
    let uid = c_string_to_str(uid);

    match yxy::wrapper::app_auth(uid) {
        Ok((ses, _)) => CString::new(ses).unwrap_or_default().into_raw(),
        Err(e) => {
            eprintln!("{e}");
            std::ptr::null_mut()
        }
    }
}

/// Build [`AppHandler`] by exist session token
///
/// ## Safety
/// C-ABI usage only
#[no_mangle]
pub unsafe extern "C" fn build_app_handler(token: *const c_char) -> *mut AppHandler {
    check_null_return_null!(token);
    let session = c_string_to_str(token);

    match AppHandler::build(session) {
        Ok(handler) => Box::into_raw(Box::new(handler)),
        Err(e) => {
            eprintln!("{e}");
            std::ptr::null_mut()
        }
    }
}

/// Get [`AppHandler`] by UID with default query
///
/// ## Safety
/// C-ABI usage only
#[no_mangle]
pub unsafe extern "C" fn get_app_handler(uid: *const c_char) -> *mut AppHandler {
    check_null_return_null!(uid);
    let uid = c_string_to_str(uid);

    match AppHandler::build_by_uid(uid) {
        Ok(handler) => Box::into_raw(Box::new(handler)),
        Err(e) => {
            eprintln!("{e}");
            std::ptr::null_mut()
        }
    }
}
