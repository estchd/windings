#![allow(non_snake_case)]

use std::ffi::{CString, OsStr, OsString};
use crate::type_wrappers::conversion::{convert_c_to_os_wide_string, convert_bool};
use std::convert::{TryInto, TryFrom};
use bitflags::*;
use std::error::Error;
use bitflags::_core::mem::MaybeUninit;
use winapi::shared::minwindef::{BOOL, DWORD, ULONG};


bitflags!(
    /// Process Error Mode Bitflags
    pub struct ERROR_MODE : winapi::shared::minwindef::UINT {
        const FAILCRITICALERRORS = winapi::um::winbase::SEM_FAILCRITICALERRORS;
        const NOALIGNMENTFAULTEXCEPT = winapi::um::winbase::SEM_NOALIGNMENTFAULTEXCEPT;
        const NOGPFAULTERRORBOX = winapi::um::winbase::SEM_NOGPFAULTERRORBOX;
        const NOOPENFILEERRORBOX = winapi::um::winbase::SEM_NOOPENFILEERRORBOX;
});


bitflags!(
    /// Thread Error Mode Bitflags
    ///
    /// # Note
    ///
    /// ERROR_MODE and THREAD_ERROR_MODE are the same Bitflags in principle but since THREAD_ERROR_MODE does not support NOALIGNMENTFAULTEXCEPT this had to be made into separate Bitflags
    pub struct THREAD_ERROR_MODE : winapi::shared::minwindef::UINT {
        const FAILCRITICALERRORS = winapi::um::winbase::SEM_FAILCRITICALERRORS;
        const NOGPFAULTERRORBOX = winapi::um::winbase::SEM_NOGPFAULTERRORBOX;
        const NOOPENFILEERRORBOX = winapi::um::winbase::SEM_NOOPENFILEERRORBOX;
});

/// Wrapping Function for the GetLastError Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
///
/// # Return
/// This Function returns the calling Threads last Error Code
// This Function is already Safe, although a type wrapping for the Error Code may be considered
#[inline]
pub fn GetLastError() -> u32 {
    let value: u32;
    unsafe {
        value = winapi::um::errhandlingapi::GetLastError();
    }
    return value;
}

/// Wrapping Function for the FatalAppExitA Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-fatalappexita)
///
/// This Function Displays an Error Message Box and terminates the Application when the Message box is closed
///
/// # Arguments
///
/// * `message_text` - The Text that should be displayed inside the message box
// This Function is already Safe
#[inline]
pub fn FatalAppExitA(message_text: CString) {
    unsafe {
        winapi::um::errhandlingapi::FatalAppExitA(0,message_text.as_ptr());
    }
}

/// Wrapping Function for the FatalAppExitW Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-fatalappexitw)
///
/// This Function Displays an Error Message Box and terminates the Application when the Message box is closed
///
/// # Arguments
///
/// * `message_text` - The Text that should be displayed inside the message box
///
/// # Note
///
/// Due to the Fact that OSStrings can contain inner Null-Characters and are not Null-Terminated, the Function instead accepts a CString.
/// This means, that for the Moment this Function does the same thing as its ASCII String Equivalent, although it will be less performant because of the needed String Conversion

// This Function is already Safe
// TODO: Make this Function work with OSStrings
#[inline]
pub fn FatalAppExitW(message_text: CString) {
    let os_string = convert_c_to_os_wide_string(message_text);
    unsafe {
        winapi::um::errhandlingapi::FatalAppExitW(0,os_string.as_ptr());
    }
}

/// Wrapping Function for the GetErrorMode Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-geterrormode)
///
/// # Return
///
/// This Function returns the Process Error Mode
///
/// # Note
///
/// The Microsoft Documentation indicates that the Function treats the Error Mode as an enum and returns one of the possibilities.
/// Testing has revealed, that this is not true and that the Error Mode possibilities are instead treated as Bitflags

// This Function is already Safe
// TODO: Additional Testing to ensure that the above assumption is indeed true and the test results where not just a fluke
#[inline]
pub fn GetErrorMode() -> ERROR_MODE {
    let error_mode: u32;
    unsafe {
        error_mode = winapi::um::errhandlingapi::GetErrorMode();
    }

    return ERROR_MODE::from_bits(error_mode).expect("Invalid Error Mode Flags");
}

/// Wrapping Function for the SetErrorMode Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-seterrormode)
///
/// This Function sets the Process Error Mode
///
/// # Arguments
///
/// * `mode` - The ERROR_MODE that should be set
///
/// # Note
///
/// The Microsoft Documentation indicates that the Function treats the Error Mode as an enum and expects only one of the possibilities.
/// Testing has revealed, that this is not true and that the Error Mode possibilities are instead treated as Bitflags

// This Function is already Safe
// TODO: Additional Testing to ensure that the above assumption is indeed true and the test results where not just a fluke
#[inline]
pub fn SetErrorMode(mode: ERROR_MODE) -> ERROR_MODE {
    let previous_error_mode: u32;
    unsafe {
        previous_error_mode = winapi::um::errhandlingapi::SetErrorMode(mode.bits);
    }

    return ERROR_MODE::from_bits(previous_error_mode).expect("Invalid Error Mode Flags");
}

/// Wrapping Function for the GetThreadErrorMode Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getthreaderrormode)
///
/// # Return
///
/// This Function returns the Calling Threads Error Mode
///
/// # Note
///
/// The Microsoft Documentation indicates that the Function treats the Error Mode as an enum and returns one of the possibilities.
/// Testing has revealed, that this is not true and that the Error Mode possibilities are instead treated as Bitflags

// This Function is already Safe
// TODO: Additional Testing to ensure that the above assumption is indeed true and the test results where not just a fluke
#[inline]
pub fn GetThreadErrorMode() -> THREAD_ERROR_MODE {
    let error_mode: u32;
    unsafe {
        error_mode = winapi::um::errhandlingapi::GetThreadErrorMode();
    }
    return THREAD_ERROR_MODE::from_bits(error_mode).expect("Invalid Error Mode Flags");
}

/// Wrapping Function for the SetErrorMode Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-seterrormode)
///
/// This Function sets the Calling Threads Error Mode
///
/// # Arguments
///
/// * `mode` - The Error Mode that should be set
///
/// # Note
///
/// The Microsoft Documentation indicates that the Function treats the Error Mode as an enum and expects only one of the possibilities.
/// Testing has revealed, that this is not true and that the Error Mode possibilities are instead treated as Bitflags

// This Function is already Safe
// TODO: Additional Testing to ensure that the above assumption is indeed true and the test results where not just a fluke
#[inline]
pub fn SetThreadErrorMode(mode: THREAD_ERROR_MODE) -> Result<THREAD_ERROR_MODE, u32> {
    let mut previous_error_mode: MaybeUninit<DWORD> = MaybeUninit::uninit();
    let succeeded: BOOL;
    unsafe {
        succeeded = winapi::um::errhandlingapi::SetThreadErrorMode(mode.bits, (&mut previous_error_mode).as_mut_ptr())
    }
    return match convert_bool(succeeded) {
        true => {
            let init_error_mode: u32;
            unsafe {
                init_error_mode = previous_error_mode.assume_init();
            }
            Ok(THREAD_ERROR_MODE::from_bits(init_error_mode).expect("Invalid Error Mode Flags"))
        },
        false => {
            Err(GetLastError())
        }
    }
}

/// Wrapping Function for the RaiseException Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-raiseexception)
///
/// This Function Raises an Exception in the Calling Thread
///
/// # Arguments
///
/// * `exception_code` - The Error Code of the Exception that should be raised. This Error Code is Application Defined
/// * `is_continuable` - Indicates whether this Exception should be continuable
/// * `exception_arguments` - Pointers to the Exception Arguments that should be passed with the Exception

// This Function is already Safe
#[inline]
pub fn RaiseException(exception_code: u32, is_continuable: bool, exception_arguments: &[usize]) {
    let continuable_flag = match is_continuable {
        true => 0,
        false => winapi::um::minwinbase::EXCEPTION_NONCONTINUABLE_EXCEPTION
    };
    unsafe {
        winapi::um::errhandlingapi::RaiseException(exception_code, continuable_flag, exception_arguments.len() as u32, exception_arguments.as_ptr())
    }
}

/*
#[inline]
pub fn RaiseFailFastException(exception_record: &EXCEPTION_)
*/

#[cfg(test)]
mod test {
    use crate::type_wrappers::error_handling_api::{GetErrorMode, GetThreadErrorMode, SetErrorMode, SetThreadErrorMode};
    use crate::type_wrappers::error_handling_api::{ERROR_MODE,THREAD_ERROR_MODE};
    use bitflags::_core::ptr::null_mut;

    #[test]
    fn test_raise_exception() {
        unsafe {
            winapi::um::errhandlingapi::RaiseException(1u32, 0u32, 0u32, null_mut());
        }
    }

    #[test]
    fn test_error_mode() {
        let error_mode = GetErrorMode();
        let error_uint: u32 = error_mode.bits;
        println!("{:?}, {}",error_mode,error_uint);
    }

    #[test]
    fn test_thread_error_mode() {
        let error_mode = GetThreadErrorMode();
        let error_uint: u32 = error_mode.bits;
        println!("{:?}, {}",error_mode, error_uint);
    }

    #[test]
    fn test_error_mode_get_set_pair() {
        let mut error_mode: ERROR_MODE = ERROR_MODE::from_bits(0).unwrap();
        error_mode.insert(ERROR_MODE::NOALIGNMENTFAULTEXCEPT);
        error_mode.insert(ERROR_MODE::NOOPENFILEERRORBOX);
        SetErrorMode(error_mode);
        assert_eq!(GetErrorMode(), error_mode);
    }

    #[test]
    fn test_thread_error_mode_get_set_pair() {
        let mut error_mode: THREAD_ERROR_MODE = THREAD_ERROR_MODE::from_bits(0).unwrap();
        error_mode.insert(THREAD_ERROR_MODE::NOGPFAULTERRORBOX);
        error_mode.insert(THREAD_ERROR_MODE::NOOPENFILEERRORBOX);
        let set_result = SetThreadErrorMode(error_mode).unwrap();
        assert_eq!(GetThreadErrorMode(), error_mode);

    }
}