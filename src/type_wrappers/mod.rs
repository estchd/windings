//! # Type Wrappers
//!
//! This module is used to provide a wrapper on top of WINAPI that uses Rust Types instead of C FFI Types.
//!
//! It is not necessarily intended to make the api safe with there Wrappers (types like HWND or COM pointers, that require special handling and ownership are left as is).
//!
//! Instead, this module is used to separate FFI Type Conversion and Safe Wrapping.


/// Functions and Macros used to convert Types between C FFI and Rust
#[macro_use]
pub mod type_conversion;

pub mod window;
pub mod window_class;

/// Type Wrappers for the errhandlingapi.h Header
pub mod error_handling_api;

/// Type Wrappers for the Windows Error Reporting Technology
///
/// See Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/_wer/)
pub mod error_reporting;

/// Type Wrappers for the Trace Logging Technology
///
/// See Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/_tracelogging/)
pub mod trace_logging;

/// Type Wrappers for the Event Collector Technology
///
/// See Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/_wec/)
pub mod event_collector;

/// Type Wrappers for the Application Recovery and Restart Technology
///
/// See Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/_recovery/)
pub mod app_recovery_restart;

/// Type Wrappers for the Performance Counters Technology
///
/// See Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/_perf/)
pub mod performance_counters;