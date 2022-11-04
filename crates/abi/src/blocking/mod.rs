//! # Warning: Not actively maintained
//! This feature is not enabled by default.

use crate::check_null_return;
use std::ffi::*;

pub mod app;
pub mod auth;
pub mod login;

/// Free c-string
///
/// Deallocate c-style string to avoid memory leak.
///
/// ## Safety
/// C-ABI usage only
///
#[no_mangle]
pub unsafe extern "C" fn destruct_c_string(ptr: *mut c_char) {
    check_null_return!(ptr);

    drop(CString::from_raw(ptr));
}
