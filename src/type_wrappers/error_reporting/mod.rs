//! # Windows Error Reporting
//!
//! Type Wrappers for the Windows Error Reporting Technology
//!
//! Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/_wer/)

/// Type Wrappers for the errorrep.h Header
///
/// Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/errorrep/)
pub mod error_report;

/// Type Wrappers for the werapi.h Header
///
/// Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/werapi/)
pub mod wer_api;
