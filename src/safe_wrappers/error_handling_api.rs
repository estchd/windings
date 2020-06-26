use winapi::um::winnt::PVOID;

// Re-Exports of Functions that only required Type Conversions
pub use crate::type_wrappers::error_handling_api::{ERROR_MODE, THREAD_ERROR_MODE};
pub use crate::type_wrappers::error_handling_api::FatalAppExitA;
pub use crate::type_wrappers::error_handling_api::FatalAppExitW;
pub use crate::type_wrappers::error_handling_api::GetThreadErrorMode;
pub use crate::type_wrappers::error_handling_api::GetErrorMode;
pub use crate::type_wrappers::error_handling_api::GetLastError;
pub use crate::type_wrappers::error_handling_api::RaiseException;
pub use crate::type_wrappers::error_handling_api::RaiseFailFastException;
pub use crate::type_wrappers::error_handling_api::SetErrorMode;
pub use crate::type_wrappers::error_handling_api::SetLastError;
pub use crate::type_wrappers::error_handling_api::SetThreadErrorMode;
pub use crate::type_wrappers::error_handling_api::SetUnhandledExceptionFilter;
pub use crate::type_wrappers::error_handling_api::UnhandledExceptionFilter;
use crate::type_wrappers::error_handling_api::{RemoveVectoredContinueHandler, RemoveVectoredExceptionHandler, PlacementPosition, AddVectoredExceptionHandler};
use winapi::ctypes::c_void;

/// Handle to a Registered Vectored Exception Handler.
///
/// Documentation on Vectored Exception Handlers can be found [here](https://docs.microsoft.com/en-us/windows/win32/debug/vectored-exception-handling)
///
/// # Usage
///
/// This Structure really only serves to automatically Unregister the Handler.
///
/// # Note
///
/// This Handle implements the Drop Trait, so that Registered Handlers get Unregistered Automatically.
/// Since the Handle will need to be unregistered when the Handler Function goes out of Scope or is unloaded, a lifetime Parameter might be needed.
/// As yet i do not have a concrete implementation for this though.
///
/// The Fact that Vectored Exception Handlers are not Frame-based may warrant a restructuring of this Handler Struct
// TODO: Check if handlers to non Frame-based Entities require special implementations
pub struct VectoredExceptionHandler {
    handle: PVOID
}

impl VectoredExceptionHandler {
    /// # Warning Unimplemented
    ///
    /// This Function is not yet implemented.
    /// This Function will remain unimplemented until a good way to wrap FFI Callbacks is found
    ///
    /// # Note
    ///
    /// Registers a new Vectored Exception Handler
    ///
    /// # Arguments
    ///
    /// * `placement_position` - Specifies, where the Handler will be placed in the Exception Handler List
    /// * `handler` - The Handler Function that should be added
    ///
    /// # Return
    ///
    /// This Function returns a Result Containing a Handle to the Registered Exception Handler

    // TODO: Implement this Function
    pub fn add_vectored_exception_handler(placement_position: PlacementPosition) -> Result<VectoredExceptionHandler, ()> {
        unimplemented!();
        let handle = AddVectoredExceptionHandler(placement_position)?;
        return Ok(VectoredExceptionHandler{handle});
    }
}

impl Drop for VectoredExceptionHandler {
    /// Unregisters the Vectored Exception Handler
    #[inline]
    fn drop(&mut self) {
        // Since this Function Should only fail for a not registered handle, this expect should be safe
        // We also cannot return a Result since Drop must succeed
        RemoveVectoredExceptionHandler(self.handle).expect("A Registered Exception Handler could not be dropped.");
    }
}

/// Handle to a Registered Vectored Continue Handler
///
/// Documentation on Vectored Exception Handlers can be found [here](https://docs.microsoft.com/en-us/windows/win32/debug/vectored-exception-handling)
///
/// # Usage
///
/// This Structure really only serves to automatically Unregister the Handler
///
/// # Note
///
/// This Handle implements the Drop Trait, so that Registered Handlers get Unregistered Automatically.
/// Since the Handle will need to be unregistered when the Handler Function goes out of Scope or is unloaded, a lifetime Parameter might be needed.
/// As yet i do not have a concrete implementation for this though.
///
/// The Fact that Vectored Continue Handlers are not Frame-based may warrant a restructuring of this Handler Struct
// TODO: Check if handlers to non Frame-based Entities require special implementations
pub struct VectoredContinueHandler {
    handle: PVOID
}

impl VectoredContinueHandler {
    /// # Warning Unimplemented
    ///
    /// This Function is not yet implemented.
    /// This Function will remain unimplemented until a good way to wrap FFI Callbacks is found
    ///
    /// # Note
    ///
    /// Registers a new Vectored Continue Handler
    ///
    /// # Arguments
    ///
    /// * `placement_position` - Specifies, where the Handler will be placed in the Exception Handler List
    /// * `handler` - The Handler Function that should be added
    ///
    /// # Return
    ///
    /// This Function returns a Result Containing a Handle to the Registered Continue Handler

    // TODO: Implement this Function
    pub fn add_vectored_exception_handler(placement_position: PlacementPosition) -> Result<VectoredExceptionHandler, ()> {
        unimplemented!();
        let handle = AddVectoredExceptionHandler(placement_position)?;
        return Ok(VectoredExceptionHandler{handle});
    }
}

impl Drop for VectoredContinueHandler {
    /// Unregisters the Vectored Continue Handler
    #[inline]
    fn drop(&mut self) {
        // Since this Function Should only fail for a not registered handle, this expect should be safe
        // We also cannot return a Result since Drop must succeed
        RemoveVectoredContinueHandler(self.handle).expect("A Registered Continue Handler could not be dropped.");
    }
}