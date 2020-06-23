//! # Windings
//!
//! ## THIS CRATE IS HIGHLY UNSTABLE AND NOT MEANT FOR PRODUCTION CODE
//!
//! The Windings Crate aims to provide a safe wrapper for the Win32 API.
//!
//! Please note, that because of the safe wrapping of the API, Crate Functions and Modules may not map 1:1 to API Functions or Headers.
//! This shall be indicated in the Documentation of said Functions and Modules.
//!
//! Please also note, that because of the safe wrapping, some functions may not work exactly the same as API Functions.
//! This shall be avoided if at all possible and if necessary indicated in the Documentation
//!
//! As a final note, due to how things like FFI Callbacks may be wrapped, it may be necessary to use the Win32 API in ways that, if modified by an external library may lead to UB or worse.
//! This shall be indicated as well as avoided at all costs.


/// Wrapper Functions on top of WINAPI that convert Rust Types into FFI Types
#[macro_use]
mod type_wrappers;

/// The Actual Safe Wrappers for the Win32 API on top of the Type Wrappers
mod safe_wrappers;

// Module Re-Exports
pub use safe_wrappers::unknown;
pub use safe_wrappers::dxgi;

pub use safe_wrappers::error_handling_api;