//! # Werapi.h
//!
//! This module provides Type Wrapping for Functions in the werapi.h Header
//!
//! Sadly, much of werapi.h does not have FFI Bindings in WINAPI yet, so this module will remain incomplete for now

#![allow(non_snake_case, dead_code)]

use std::ffi::CString;
use crate::type_wrappers::type_conversion::{convert_c_to_os_wide_string, convert_rust_bool, convert_reference_to_pvoid};
use winapi::um::winnt::{PVOID, HANDLE};
use winapi::ctypes::c_void;
use winapi::shared::minwindef::DWORD;
use bitflags::bitflags;
use std::mem::MaybeUninit;


/// Wrapping Function for the WerAddExcludedApplication Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/werapi/nf-werapi-weraddexcludedapplication)
///
/// This Function Excludes an Application from Windows Error Reporting
///
/// # Arguments
///
/// * `application_name` - The Name of the Application that should be excluded
/// * `all_users` - Whether this Application should be excluded for all Users
///
/// # Return
///
/// `S_OK` - This should be returned on Success
/// `E_ACCESSDENIED` - This should be returned if the Process does not have the permission to write to the registry
///
/// # Notes
///
/// If all_uses is true, the excluded list is stored under HKEY_LOCAL_MACHINE in the Registry.
/// If it is false, it is stored under HKEY_CURRENT_USER in the Registry.

// TODO: Consider Type Wrapping the HRESULT
#[inline]
pub fn WerAddExcludedApplication(application_name: CString, all_users: bool) -> i32 {
    let result: i32;
    let wide_name = convert_c_to_os_wide_string(application_name);
    unsafe {
        result = winapi::um::werapi::WerAddExcludedApplication(wide_name.as_ptr(), all_users as i32);
    }
    return result;
}

/// Wrapping Function for the WerRemoveExcludedApplication Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/werapi/nf-werapi-werremoveexcludedapplication)
///
/// This Function Removes a previously excluded Application from Windows Error Reporting
///
/// # Arguments
///
/// * `application_name` - The Name of the Application that should be removed from the exclusion List
/// * `all_users` - Whether this Application should be removed for all Users
///
/// # Return
///
/// `S_OK` - On Success
/// `E_ACCESSDENIED` - This should be returned if the Process does not have the permission to write to the registry
///
/// # Notes
///
/// If all_uses is true, then the application is removed from the list stored under HKEY_LOCAL_MACHINE in the Registry.
/// If it is false, then it is removed from the list stored under HKEY_CURRENT_USER in the Registry.

// TODO: Consider Type Wrapping the HRESULT
#[inline]
pub fn WerRemoveExcludedApplication(application_name: CString, all_users: bool) -> i32 {
    let result: i32;
    let wide_name = convert_c_to_os_wide_string(application_name);
    unsafe {
        result = winapi::um::werapi::WerRemoveExcludedApplication(wide_name.as_ptr(), all_users as i32);
    }
    return result;
}

/// Wrapping Function for the WerRegisterRuntimeExceptionModule Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/werapi/nf-werapi-werregisterruntimeexceptionmodule)
///
/// This Function Registers a custom Runtime Exception Handler
///
/// # Arguments
///
/// * `dll_name` -  The Name of the DLL that should be loaded
/// * `context` - A Pointer to Arbitrary Context Information, that is passed to the Handlers Callback Function
///
/// # Return
///
/// `S_OK` - On Success
/// `WER_E_INVALID_STATE` - Process State is invalid for Registering the Module
/// `ERROR_INSUFFICIENT_BUFFER` - Too many registered Exception Modules
///
/// # Notes
///
/// The Context Pointer probably requires Safe Wrapping, but i am not sure how exactly to wrap it yet.
/// It might be considered to treat a Succeeded Registering as a Handle with a Drop Trait.

// TODO: Consider Type Wrapping the HRESULT
// TODO: Consider Wrapping the context pointer
#[inline]
pub fn WerRegisterRuntimeExceptionModule(dll_name: CString, context: PVOID) -> i32 {
    let result: i32;
    let wide_name = convert_c_to_os_wide_string(dll_name);
    unsafe {
        result = winapi::um::werapi::WerRegisterRuntimeExceptionModule(wide_name.as_ptr(),context);
    }
    return result;
}

/// Wrapping Function for the WerUnregisterRuntimeExceptionModule Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/werapi/nf-werapi-werunregisterruntimeexceptionmodule)
///
/// This Function Unregisters a custom Runtime Exception Handler
///
/// # Arguments
///
/// * `dll_name` -  The Name of the DLL that should be unloaded
/// * `context` - A Pointer to Arbitrary Context Information, that was passed to the Handlers Callback Function
///
/// # Return
///
/// `S_OK` - On Success
/// `WER_E_INVALID_STATE` - Process State is invalid for Registering the Module
/// `WER_E_NOT_FOUND` - No Module with this Name was Registered
///
/// # Notes
///
/// The Context Pointer probably requires Safe Wrapping, but i am not sure how exactly to wrap it yet.
/// I'm also not really sure, why this pointer is needed here.
/// It might be considered to treat a Succeeded Registering as a Handle with a Drop Trait.

// TODO: Consider Type Wrapping the HRESULT
// TODO: Consider Wrapping the context pointer
#[inline]
pub fn WerUnregisterRuntimeExceptionModule(dll_name: CString, context: PVOID) -> i32 {
    let result: i32;
    let wide_name = convert_c_to_os_wide_string(dll_name);
    unsafe {
        result = winapi::um::werapi::WerUnregisterRuntimeExceptionModule(wide_name.as_ptr(), context);
    }
    return result;
}

/// Wrapping Function for the WerRegisterMemoryBlock Function.
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/werapi/nf-werapi-werregistermemoryblock)
///
/// This Function Registers a Memory Block for collection by Windows Error Reporting.
///
/// # Arguments
///
/// * `reference` - A Reference to the Memory Block that should be registered
///
/// # Return
///
/// `S_OK` - On Success
/// `WER_E_INVALID_STATE` - If the Process State is invalid for Registering Memory Blocks
/// `ERROR_INSUFFICIENT_BUFFER` - If there are already too many Memory Blocks Registered
///
/// # Note
///
/// It has yet to be determined if a reference is sufficient here or if there is a need to register arbitrary memory locations with arbitrary sizes.
/// A point can be made though, that said arbitrary memory locations can be made into a reference beforehand.

// TODO: Consider Type Wrapping the HRESULT
// TODO: Safe Wrapping this into a Handle Struct with Drop and lifetime semantics
// TODO: Determine, if this syntax can accommodate Array Blocks
#[inline]
pub fn WerRegisterMemoryBlock<T>(reference: &mut T) -> i32 where T: Sized {
    let block_size = std::mem::size_of::<T>() as DWORD;
    let pointer = convert_reference_to_pvoid(reference);
    let result: i32;
    unsafe {
        result = winapi::um::werapi::WerRegisterMemoryBlock(pointer, block_size);
    }
    return result;
}

/// Wrapping Function for the WerUnregisterMemoryBlock Function.
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/werapi/nf-werapi-werunregistermemoryblock)
///
/// This Function Unregisters a Memory Block for collection by Windows Error Reporting that has been previously registered.
///
/// # Arguments
///
/// * `reference` - A Reference to the Memory Block that should be unregistered
///
/// # Return
///
/// `S_OK` - On Success
/// `WER_E_INVALID_STATE` - If the Process State is invalid for Registering Memory Blocks
/// `WER_E_NOT_FOUND` - If the Memory Block has not been previously registered
///
/// # Note
///
/// It has yet to be determined if a reference is sufficient here or if there is a need to unregister arbitrary memory locations.
/// A point can be made though, that said arbitrary memory locations can be made into a reference beforehand.

// TODO: Consider Type Wrapping the HRESULT
// TODO: Safe Wrapping this into a Handle Struct with Drop and Lifetime semantics
#[inline]
pub fn WerUnregisterMemoryBlock<T>(reference: &mut T) -> i32 where T: Sized {
    let pointer = convert_reference_to_pvoid(reference);
    let result: i32;
    unsafe {
        result = winapi::um::werapi::WerUnregisterMemoryBlock(pointer);
    }
    return result;
}

// Values extracted from C++ Visual Studio rather than Documentation or WINAPI
const WER_FILE_ANONYMOUS_DATA: u32 = 2;
const WER_FILE_DELETE_WHEN_DONE: u32 = 1;

bitflags!(
    pub struct FileFlags : u32 {
        const FILE_ANONYMOUS_DATA = WER_FILE_ANONYMOUS_DATA;
        const FILE_DELETE_WHEN_DONE = WER_FILE_DELETE_WHEN_DONE;
});

CONST_TO_ENUM!(const_enum REGISTER_FILE_TYPE, winapi::um::werapi::WER_REGISTER_FILE_TYPE {
    Max = winapi::um::werapi::WerRegFileTypeMax,
    Other = winapi::um::werapi::WerRegFileTypeOther,
    UserDocument = winapi::um::werapi::WerRegFileTypeUserDocument,
});

/// Wrapping Function for the WerRegisterFile Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/werapi/nf-werapi-werregisterfile)
///
/// This Function Registers a File for collection by Windows Error Reporting
///
/// # Arguments
///
/// * `file_path` - Full Path to the File
/// * `file_type` - The Type of the File
/// * `file_flags` - Flags specifying how the File should be handled
///
/// # Return
///
/// `S_OK` - On Success
/// `WER_E_INVALID_STATE` - If the Process State is Invalid for Registering a File
/// `ERROR_INSUFFICIENT_BUFFER` - If the number of Registered Files Exceeds the Limit

// TODO: Consider Wrapping the HRESULT
#[inline]
pub fn WerRegisterFile(file_path: CString, file_type: REGISTER_FILE_TYPE, file_flags: FileFlags) -> i32 {
    let wide_path = convert_c_to_os_wide_string(file_path);
    let result: i32;
    unsafe {
        result = winapi::um::werapi::WerRegisterFile(wide_path.as_ptr(),file_type.into(),file_flags.bits());
    }
    return result;
}

/// Wrapping Function for the WerUnregisterFile Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/werapi/nf-werapi-werunregisterfile)
///
/// This Function Unregisters a File from collection by Windows Error Reporting
///
/// # Arguments
///
/// * `file_path` - Full Path to the File
///
/// # Return
///
/// `S_OK` - On Success
/// `WER_E_INVALID_STATE` - If the Process State is Invalid for Registering a File
/// `WER_E_NOT_FOUND` - If the File has not been registered for Collection

// TODO: Consider Wrapping the HRESULT
#[inline]
pub fn WerUnregisterFile(file_path: CString) -> i32 {
    let wide_path = convert_c_to_os_wide_string(file_path);
    let result: i32;
    unsafe {
        result = winapi::um::werapi::WerUnregisterFile(wide_path.as_ptr());
    }
    return result;
}

// Copied from WINAPI, needed as they are not present in the current WINAPI release, but it seems like they will be included in the next release
pub const WER_FAULT_REPORTING_FLAG_NOHEAP: DWORD = 1;
pub const WER_FAULT_REPORTING_FLAG_QUEUE: DWORD = 2;
pub const WER_FAULT_REPORTING_FLAG_DISABLE_THREAD_SUSPENSION: DWORD = 4;
pub const WER_FAULT_REPORTING_FLAG_QUEUE_UPLOAD: DWORD = 8;
pub const WER_FAULT_REPORTING_ALWAYS_SHOW_UI: DWORD = 16;
pub const WER_FAULT_REPORTING_NO_UI: DWORD = 32;
pub const WER_FAULT_REPORTING_FLAG_NO_HEAP_ON_QUEUE: DWORD = 64;
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_CRASH: DWORD = 128;
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_HANG: DWORD = 256;
pub const WER_FAULT_REPORTING_CRITICAL: DWORD = 512;
pub const WER_FAULT_REPORTING_DURABLE: DWORD = 1024;

bitflags!(
    pub struct FaultReportingFlags : DWORD {
        const NOHEAP = WER_FAULT_REPORTING_FLAG_NOHEAP;
        const QUEUE = WER_FAULT_REPORTING_FLAG_QUEUE;
        const DISABLE_THREAD_SUSPENSION = WER_FAULT_REPORTING_FLAG_DISABLE_THREAD_SUSPENSION;
        const QUEUE_UPLOAD = WER_FAULT_REPORTING_FLAG_QUEUE_UPLOAD;
        const ALWAYS_SHOW_UI = WER_FAULT_REPORTING_ALWAYS_SHOW_UI;
});

/// Wrapping Function for the WerGetFlags Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/werapi/nf-werapi-wergetflags)
///
/// This Function gets the Windows Error Reporting Settings Flags for a Process.
///
/// # Arguments
///
/// * `process` - Handle to the Process for which the Settings should be fetched
///
/// # Return
///
/// On Success, returns the fetched Flags.
/// On Error, returns the Error Code.

// TODO: Consider Wrapping the HRESULT
#[inline]
pub fn WerGetFlags(process: HANDLE) -> Result<FaultReportingFlags, i32> {
    let mut flags = MaybeUninit::<DWORD>::uninit();
    let result: i32;
    unsafe {
        result = winapi::um::werapi::WerGetFlags(process, flags.as_mut_ptr());
    }
    return match result {
        0 => {
            let flags = unsafe { flags.assume_init() };
            let flags = FaultReportingFlags::from_bits(flags).expect("Unknown Flags");
            Ok(flags)
        }
        _ => Err(result)
    }
}

/// Wrapping Function for the WerSetFlags Function
/// See the Documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/werapi/nf-werapi-wersetflags)
///
/// This Function sets the Windows Error Reporting Settings Flags for the current process.
///
/// # Return
///
/// Returns the HRESULT

// TODO: Consider Wrapping the HRESULT
#[inline]
pub fn WerSetFlags(flags: FaultReportingFlags) -> i32 {
    let result: i32;
    unsafe {
        result = winapi::um::werapi::WerSetFlags(flags.bits);
    }
    return result;
}