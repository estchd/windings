//! # Winbase
//!
//! Type Wrappers for the parts of Application Recovery and Restart included in the winbase.h Header
//!
//! Sadly, Bindings for this Header are incomplete in WINAPI.
//! This means, that this will remain incomplete for now.
//!
//! Link to the Header Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/_recovery/)

use winapi::shared::minwindef::BOOL;
use bitflags::_core::mem::MaybeUninit;
use crate::type_wrappers::type_conversion::convert_c_bool;

const S_OK: i32 = 0;

// This Function is already Safe
// TODO: Document this
wrap_noreturn_ffi_function!(
    /// Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-applicationrecoveryfinished)
    pub fn ApplicationRecoveryFinished(recovery_success: bool)
    FFI fn winapi::um::winbase::ApplicationRecoveryFinished;
    FFI args(recovery_success_ffi);

    IN:
        AUTO recovery_success => recove ry_success_ffi: bool => BOOL
);

// This Function is already Safe
// TODO: Document this
wrap_ffi_function!(
    /// Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-applicationrecoveryinprogress)
    pub fn ApplicationRecoveryInProgress()
    FFI fn winapi::um::winbase::ApplicationRecoveryInProgress;
    FFI args(cancelled) -> i32;

    OUT:
        AUTO cancelled_uninit, cancelled: BOOL => bool

    RETURN:
        OK VALUE: S_OK
);

/// Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getapplicationrecoverycallback)
// TODO: Document this
// TODO: Implement this
pub fn GetApplicationRecoveryCallback() {
    unimplemented!();
}

/// Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-registerapplicationrecoverycallback)
// TODO: Document this
// TODO: Implement this
pub fn RegisterApplicationRecoveryCallback() {
    unimplemented!();
}

wrap_ffi_function!(
    pub fn RegisterApplicationRecoveryCallback<T>(callback: i32, parameter: Option<&T>, pingInterval: u32, flags: u32)

);

// TODO: Document this
wrap_ffi_function!(
    /// Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-unregisterapplicationrecoverycallback)
    pub fn UnregisterApplicationRecoveryCallback()
    FFI fn winapi::um::winbase::UnregisterApplicationRecoveryCallback;
    FFI args() -> i32;

    RETURN:
        OK VALUE: S_OK
);