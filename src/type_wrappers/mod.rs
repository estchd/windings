//! # Type Wrappers
//!
//! This module is used to provide a wrapper on top of WINAPI that uses Rust Types instead of C FFI Types.
//!
//! It is not necessarily intended to make the api safe (types like HWND or COM pointers, that require special handling and ownership are left as is).
//!
//! Instead, this module is used to separate FFI Type Conversion and Safe Wrapping.


/// Functions and Macros used to convert Types between C FFI and Rust
#[macro_use]
pub mod conversion;

pub mod window;
pub mod window_class;

/// Type Wrappers for the errhandlingapi.h Header
pub mod error_handling_api;