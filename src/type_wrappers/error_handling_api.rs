#![allow(non_snake_case, dead_code)]

use std::ffi::{CString, OsStr, OsString};
use crate::type_wrappers::type_conversion::{convert_c_to_os_wide_string, convert_c_bool};
use std::convert::{TryInto, TryFrom};
use bitflags::*;
use std::error::Error;
use bitflags::_core::mem::MaybeUninit;
use winapi::shared::minwindef::{BOOL, DWORD, ULONG};
use winapi::um::winnt::PVOID;
use std::panic::resume_unwind;


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
///
/// This Function returns the calling Threads last Error Code
// This Function is already Safe
#[inline]
pub fn GetLastError() -> u32 {
    let value: u32;
    unsafe {
        value = winapi::um::errhandlingapi::GetLastError();
    }
    return value;
}

/// Wrapping Function for the SetLastError Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
///
/// This Function sets the Threads last Error Code
///
/// # Arguments
///
/// * `error_code` - The Error Code that should be set
// This Function is already Safe
#[inline]
pub fn SetLastError(error_code: u32) {
    unsafe {
        winapi::um::errhandlingapi::SetLastError(error_code);
    }
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
    return match convert_c_bool(succeeded) {
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

// This is needed here because it's not included in WINAPI
const FAIL_FAST_GENERATE_EXCEPTION_ADDRESS: u32 = 1;

/// # Warning Unimplemented
///
/// This Function is not yet implemented.
/// This is because the Exception Record and Context Structs are not well Documented and seem to require additional wrapping
///
/// Documentation can be found [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-raisefailfastexception)

// TODO: Implement this Method
#[inline]
pub fn RaiseFailFastException(exception_record: u32, context: u32, generate_exception_address: bool) {
    unimplemented!();
}

/// Specifies where the VectoredHandler should be placed
#[repr(u32)]
pub enum PlacementPosition {
    /// The Vectored Handler will be placed at the Front of the Handler List
    EmplaceFirst = 0u32,

    /// The Vectored Handler will be placed at the Back of the Handler List
    EmplaceLast = 1u32
}

/// # Warning Unimplemented
///
/// This Function is not yet implemented.
/// This is because i have not yet found a good way to wrap FFI Callbacks.
///
/// # Note
///
/// The Function returns a Handle to the registered handler which has to be unregistered when the registered function goes out of scope so a safe wrapper will probably be needed.
///
/// Documentation can be found [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-addvectoredcontinuehandler).
/// An overview about Vectored Exception Handling can be found [here](http://bytepointer.com/resources/pietrek_vectored_exception_handling.htm).
/// A Reference on Continue Handling has yet to be found, the best information i can find is [this thread](https://reverseengineering.stackexchange.com/questions/14992/what-are-the-vectored-continue-handlers).
/// It seems like Vectored Continue Handlers are called after an Exception Handler returns EXCEPTION_CONTINUE_EXECUTION.
///
/// # Arguments
///
/// * `placement_position` - Specifies, where the Handler will be placed in the Exception Handler List
///
/// * `handler` - Pointer to the Handler Function that should be placed in the Vector
///
/// # Return
///
/// This function returns a Handle to the Placed Exception Handler
/// Said Handler must be unregistered with RemoveVectoredContinueHandler before the Handler Function goes out of scope (for example, before the Functions DLL is unloaded)

// TODO: Implement this Method
#[inline]
pub fn AddVectoredContinueHandler(placement_position: PlacementPosition) -> Result<PVOID, ()> {
    unimplemented!()
}

/// # Warning Unimplemented
///
/// This Function is not yet implemented.
/// This is because i have not yet found a good way to wrap FFI Callbacks
///
/// # Note
///
/// The Function returns a Handle to the registered handler which has to be unregistered when the registered function goes out of scope so a safe wrapper will probably be needed
///
/// Documentation can be found [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-addvectoredexceptionhandler)
/// An overview about Vectored Exception Handling can be found [here](http://bytepointer.com/resources/pietrek_vectored_exception_handling.htm)
///
/// # Arguments
///
/// * `placement_position` - Specifies, where the Handler will be placed in the Exception Handler List
///
/// * `handler` - Pointer to the Handler Function that should be placed in the Vector
///
/// # Return
///
/// This function returns a Handle to the Placed Exception Handler if the Function Succeeds
/// Said Handler must be unregistered with RemoveVectoredExceptionHandler before the Handler Function goes out of scope (for example, before the Functions DLL is unloaded)

// TODO: Implement this Method
#[inline]
pub fn AddVectoredExceptionHandler(placement_position: PlacementPosition) -> Result<PVOID, ()> {
    unimplemented!()
}

/// Wrapping Function for the RemoveVectoredContinueHandler Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-removevectoredcontinuehandler)
///
/// This Function Unregisters a previously added Vectored Continue Handler
///
/// # Arguments
///
/// * `handle` - The Handle to the Vectored Continue Handler
///
/// # Return
///
/// This Function returns a Result indicating Success
#[inline]
pub fn RemoveVectoredContinueHandler(handle: PVOID) -> Result<(),()> {
    let result: u32;
    unsafe {
        result = winapi::um::errhandlingapi::RemoveVectoredContinueHandler(handle);
    }
    return match result {
        0 => Err(()),
        _ => Ok(())
    }
}

/// Wrapping Function for the RemoveVectoredExceptionHandler Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-removevectoredexceptionhandler)
///
/// This Function Unregisters a previously added Vectored Exception Handler
///
/// # Arguments
///
/// * `handle` - The Handle to the Vectored Exception Handler
///
/// # Return
///
/// This Function returns a Result indicating Success
#[inline]
pub fn RemoveVectoredExceptionHandler(handle: PVOID) -> Result<(),()> {
    let result: u32;
    unsafe {
        result = winapi::um::errhandlingapi::RemoveVectoredExceptionHandler(handle);
    }

    return match result {
        0 => Err(()),
        _ => Ok(())
    }
}

/// # Warning Unimplemented
///
/// This Function is not yet implemented.
/// This is because i have not yet found a good way to wrap the FFI Callbacks
///
/// # Note
///
/// This Function may be implemented by storing the Function Pointer as a global and using a static function.
/// This is not preferred though since that would lead to UB when the Function is called by external Libraries
///
/// Documentation can be found [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setunhandledexceptionfilter)

// TODO: Implement this Function
#[inline]
pub fn SetUnhandledExceptionFilter() {
    unimplemented!()
}


/// # Warning Unimplemented
///
/// This Function is not yet implemented.
/// This is because the Documentation is unclear as to whether this Function is supposed to be called by the User at all
///
/// # Note
///
/// It seems to be the Case that this Function is supposed to be called by the user, but only within some Filter Expression of an Exception Handler.
/// If this references the UnhandledExceptionFilter set by the SetUnhandledExceptionFilter Function warrants further research, although it seems unlikely.
/// The Documentation also describes this as an Application defined Function, which seems weird and should be investigated, especially since WINAPI defines it as an external binding.
///
/// Documentation can be found [here](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-unhandledexceptionfilter)

// TODO: Find out more about this Function
#[inline]
pub fn UnhandledExceptionFilter() {
    unimplemented!()
}

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
        let mut error_mode: ERROR_MODE = ERROR_MODE::empty();
        error_mode.insert(ERROR_MODE::NOALIGNMENTFAULTEXCEPT);
        error_mode.insert(ERROR_MODE::NOOPENFILEERRORBOX);
        SetErrorMode(error_mode);
        assert_eq!(GetErrorMode(), error_mode);
    }

    #[test]
    fn test_thread_error_mode_get_set_pair() {
        let mut error_mode: THREAD_ERROR_MODE = THREAD_ERROR_MODE::empty();
        error_mode.insert(THREAD_ERROR_MODE::NOGPFAULTERRORBOX);
        error_mode.insert(THREAD_ERROR_MODE::NOOPENFILEERRORBOX);
        let set_result = SetThreadErrorMode(error_mode).unwrap();
        assert_eq!(GetThreadErrorMode(), error_mode);

    }
}