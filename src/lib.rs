// Safe Rust wrapper around a small C++ string-utilities library.


//! Two layers live in this one file: 1.Private ffi module & 2. Public safe functions(str_length,to_uppercase etc.)

// 1.Private ffi module:
// All FFI calls (raw pointers, manual memory management) are confined to
//! the private [`ffi`] module and the `unsafe` blocks inside this file.
//! Private ffi module just pulls in the raw, unsafe bindings bindgen generated. Nothing outside this file can see it.
//! Everything exported from the crate root is 100% safe Rust.



// 2. Public safe functions (str_length, str_reverse, count_vowels, to_uppercase):
// each one converts the Rust &str input into a C-compatible CString
//  calls the unsafe FFI function inside a small unsafe block
// and for the two that return allocated memory, copies the result into an owned Rust String and frees the C buffer.


// ## Byte vs. Unicode semantics
// The underlying C++ functions operate on raw bytes (`char*`), not Unicode
// codepoints:
// - [`str_length`] returns the length in **bytes**, matching C's `strlen`.

// - [`str_reverse`] reverses the **byte order** of the string. For ASCII
//   input this is exactly what you'd expect. For input containing
//   multi-byte UTF-8 characters, a byte-level reversal breaks those
//   sequences apart — so instead of returning corrupted text, this wrapper
//   validates the C++ output with `String::from_utf8` and returns

//   [`Error::InvalidUtf8`] if reversal produced invalid UTF-8. This is a
//   deliberate safety choice: better to surface an error than hand back
//   a `String` that isn't valid UTF-8.

// - [`to_uppercase`] only upper-cases ASCII letters; bytes >= 0x80 (i.e.
//   all continuation/lead bytes of multi-byte UTF-8 sequences) are passed
//   through untouched, so UTF-8 validity is preserved.

// - [`count_vowels`] counts ASCII vowels only (`a/e/i/o/u`, case-insensitive).

mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::ffi::{CString, NulError};
use std::os::raw::c_char;

/// Errors that can occur when calling into the C++ library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The input `&str` contained an interior NUL byte and cannot be
    /// represented as a C string.
    #[error("input contains an interior NUL byte and cannot be passed to the C library: {0}")]
    InteriorNul(#[from] NulError),

    /// The C++ side returned a null pointer (e.g. allocation failure).
    #[error("the underlying C++ function returned a null pointer")]
    NullPointer,

    /// The C++ side produced bytes that are not valid UTF-8 once decoded
    /// back into a Rust `String` (can happen when reversing strings that
    /// contain multi-byte UTF-8 characters ).
    #[error("result from C++ was not valid UTF-8: {0}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
}

/// Converts a Rust `&str` into an owned `CString`, mapping failure into
/// our `Error` type. Centralizing this keeps every public function's
/// `unsafe` footprint tiny and uniform.
fn to_cstring(input: &str) -> Result<CString, Error> {
    Ok(CString::new(input)?)
}

/// Takes ownership of a `char*` allocated by the C++ side, copies it into
/// a Rust `String` (validating UTF-8), and frees the original C buffer.
/// Returns `Error::NullPointer` if `ptr` is null instead of dereferencing it.
///
/// # Safety
/// `ptr` must either be null or a valid, NUL-terminated buffer that was
/// allocated by the C++ library with `malloc` (i.e. returned from
/// `str_reverse` or `to_uppercase`) and not yet freed.
unsafe fn owned_c_string_to_rust(ptr: *mut c_char) -> Result<String, Error> {
    if ptr.is_null() {
        return Err(Error::NullPointer);
    }
    let c_str = std::ffi::CStr::from_ptr(ptr);
    let bytes = c_str.to_bytes().to_vec(); // copy out before freeing
    ffi::proxie_free_string(ptr);
    Ok(String::from_utf8(bytes)?)
}

/// Returns the length of `input` in bytes (matching C's `strlen`
/// semantics — see module docs for why this is bytes, not chars).
pub fn str_length(input: &str) -> Result<usize, Error> {
    let c_input = to_cstring(input)?;
    // SAFETY: c_input is a valid, NUL-terminated buffer owned by us for
    // the duration of this call. str_length only reads from it.
    let len = unsafe { ffi::str_length(c_input.as_ptr()) };
    Ok(len as usize)
}

/// Returns a byte-reversed copy of `input`.
///
/// Returns `Error::InvalidUtf8` if `input` contains multi-byte UTF-8
/// characters whose byte order does not survive a raw reversal (see
/// module docs).
pub fn str_reverse(input: &str) -> Result<String, Error> {
    let c_input = to_cstring(input)?;
    // SAFETY: c_input is valid for the duration of this call. The
    // returned pointer is either null or a freshly malloc'd, NUL-terminated
    // buffer that we take ownership of and free via owned_c_string_to_rust.
    let result_ptr = unsafe { ffi::str_reverse(c_input.as_ptr()) };
    unsafe { owned_c_string_to_rust(result_ptr) }
}

/// Counts ASCII vowels (`a/e/i/o/u`, case-insensitive) in `input`.
pub fn count_vowels(input: &str) -> Result<i32, Error> {
    let c_input = to_cstring(input)?;
    // SAFETY: c_input is valid for the duration of this read-only call.
    let count = unsafe { ffi::count_vowels(c_input.as_ptr()) };
    Ok(count)
}

/// Returns a copy of `input` with ASCII letters upper-cased. Non-ASCII
/// bytes (e.g. multi-byte UTF-8 characters) are left untouched.
pub fn to_uppercase(input: &str) -> Result<String, Error> {
    let c_input = to_cstring(input)?;
    // SAFETY: c_input is valid for the duration of this call; the returned
    // pointer is owned and freed exactly as in str_reverse above.
    let result_ptr = unsafe { ffi::to_uppercase(c_input.as_ptr()) };
    unsafe { owned_c_string_to_rust(result_ptr) }
}
