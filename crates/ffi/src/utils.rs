//! Utils

use std::ffi::{c_char, CStr};

/// Convert c-string to `&str`
///
/// ## Safety
/// Unchecked UTF-8 slice
#[allow(dead_code)]
pub unsafe fn c_string_to_str<'a>(c_str: *const c_char) -> &'a str {
    let c_str = CStr::from_ptr(c_str);
    std::str::from_utf8_unchecked(c_str.to_bytes())
}

/// Copy c-string into [`String`]
///
/// Copies [`CStr`] into new [`String`]
///
/// ## Safety
/// Unchecked UTF-8 String
#[allow(dead_code)]
pub unsafe fn copy_c_string_into_string(c_str: *const c_char) -> String {
    let c_str = CStr::from_ptr(c_str);
    String::from_utf8_unchecked(c_str.to_bytes().to_vec())
}

/// Copy `&str` to fixed-size `c_char` array
#[allow(dead_code)]
pub unsafe fn copy_str_to_char_array<const L: usize>(s: &str) -> [c_char; L] {
    let mut c = [0 as c_char; L];
    let len = s.as_bytes().len();
    if len > L - 1 {
        let slice = std::slice::from_raw_parts(s.as_ptr() as *mut c_char, L - 1);
        c[..L - 1].copy_from_slice(slice);
    } else {
        let slice = std::slice::from_raw_parts(s.as_ptr() as *mut c_char, len);
        c[..len].copy_from_slice(slice);
    }
    c
}
