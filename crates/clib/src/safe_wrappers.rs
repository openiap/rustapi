#![warn(missing_docs)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::ffi::{c_char, CStr};
/// Converts a C-style string (`*const c_char`) to a Rust `String`.
///
/// # Arguments
///
/// * `c_char` - A raw pointer to a C-style string.
///
/// # Returns
///
/// * `String` - The converted Rust `String`. Returns an empty string (`""`) if the pointer is null
///              or if the conversion to UTF-8 fails.
///
/// # Safety
///
/// This function assumes that the pointer is valid and points to a null-terminated string.
/// If the pointer is null or if the string is not valid UTF-8, an empty string is returned.
#[tracing::instrument(skip_all)]
pub fn c_char_to_str(c_char: *const c_char) -> String {
    if c_char.is_null() {
        return "".to_string();
    }
    unsafe {
        // SAFETY: We checked that `c_char` is not null, and `CStr::from_ptr` 
        // is safe to use as long as the pointer is valid and null-terminated.
        match CStr::from_ptr(c_char).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => "".to_string(),
        }
    }
}
/// Safely converts a mutable raw pointer to a mutable reference.
///
/// # Safety
/// This function assumes the pointer is valid and properly aligned.
/// It returns `None` if the pointer is null.
///
/// # Arguments
///
/// * `obj` - A mutable raw pointer to the object.
///
/// # Returns
///
/// * `Option<&'a mut T>` - A mutable reference wrapped in `Some` if the pointer is valid, otherwise `None`.
#[inline(always)]
#[tracing::instrument(skip_all)]
pub fn safe_wrapper<'a, T>(obj: *mut T) -> Option<&'a mut T> {
    if obj.is_null() {
        None
    } else if (obj as usize) % std::mem::align_of::<T>() != 0 {
        eprintln!("Pointer is not properly aligned");
        None
    } else {
        unsafe { Some(&mut *obj ) }
    }
}