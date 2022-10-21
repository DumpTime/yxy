//! # yxy-abi
//!
//! C-style yxy bindings
//!
//! ## Safety
//! Every pointer return from rust side should be manually freed by `free_*` prefix functions.

pub mod app;
pub mod auth;
pub mod login;
mod utils;

use ffi_destruct::{extern_c_destructor, Destruct};
use std::ffi::*;
use utils::*;

/// Error code definition
#[repr(C)]
pub enum ErrorCode {
    /// No error
    OK = 0,

    /// Generic error
    ERROR = -1,

    /// Invalid inputs or operation
    EInvalid = -2,
    /// Authentication failed
    EAuth = -3,
    /// No bind info etc.
    ENoFound = -4,

    /// Bad phone number
    EPhoneNum = -10,
    /// Limit of SMS verification code sent
    ELimited = -11,
    /// Error verification code & captcha code
    EVeriCode = -12,
    /// Failed to get captcha image
    ECaptcha = -13,
}

#[macro_export]
macro_rules! check_null_return_null {
    ($($arg:expr),+) => {
        if $($arg.is_null() ||)* false {
            eprintln!("Invalid input");
            return std::ptr::null_mut();
        }
    };
}

#[macro_export]
macro_rules! check_null_return {
    ($($arg:expr),+) => {
        if $($arg.is_null() ||)* false {
            eprintln!("Invalid input");
            return;
        }
    };
}

#[macro_export]
macro_rules! check_null_return_x {
    ($x:expr,$($arg:expr),+) => {
        if $($arg.is_null() ||)* false {
            eprintln!("Invalid input");
            return $x;
        }
    };
}

/// Free c-string
///
/// Deallocate c-style string to avoid memory leak.
///
/// ## Safety
/// C-ABI usage only
#[no_mangle]
pub unsafe extern "C" fn destruct_c_string(ptr: *mut c_char) {
    check_null_return!(ptr);

    drop(CString::from_raw(ptr));
}
