//! # Conversion
//!
//! This Module Contains functions and Macros to make Conversions between C FFI Types and Rust Types easier

use winapi::shared::minwindef::BOOL;
use std::ffi::{OsString, CString};
use std::os::windows::prelude::*;
use winapi::um::winnt::PVOID;
use winapi::ctypes::c_void;

/// Converts C Constant Based Enums into Rust Enums
///
/// The Resulting Enum Supports the Into Trait into the original Type as well as TryFrom the original Type
///
/// TryFrom is needed because C APIs may return non-specified Values

// TODO: Implement a Macro for Enums with a Default value instead of a TryFrom
#[macro_export]
macro_rules! CONST_TO_ENUM {
    {const_enum $name:ident, $original:path {$($new:ident = $old:path,)*}} =>
    {
        #[derive(Debug, Copy, Clone)]
        pub enum $name {
            $($new,)*
        }

        CONST_TO_ENUM!{const_enum_into $name, $original {$($new = $old,)*}}

        CONST_TO_ENUM!{const_enum_from $name, $original {$($new = $old,)*}}
    };

    {const_enum_into $name:ident, $original:path {$($new:ident = $old:path,)*}} =>
    {
        impl Into<$original> for $name {
            fn into(self) -> $original {
                return match self {
                    $($name::$new => $old,)*
                };
            }
        }
    };

    {const_enum_from $name:ident, $original:path {$($new:ident = $old:path,)*}} =>
    {
        impl std::convert::TryFrom<$original> for $name {
            type Error = $original;

            fn try_from(value: $original) -> Result<Self, Self::Error> {
                return match value {
                    $($old => Ok($name::$new),)*
                    _ => Err(value),
                }
            }
        }
    }
}

/// Converts C Style BOOLs to Rust booleans.
///
/// # Arguments
///
/// * `value` - The C Style BOOL that should be converted
///
/// # Return
///
/// This Function returns true for a non-zero value and false for a zero value
#[inline]
pub fn convert_c_bool(value: BOOL) -> bool {
    return value != 0;
}

/// Converts Rust Style booleans to C BOOLs.
///
/// # Arguments
///
/// * `value` - The Rust Style bool that should be converted
///
/// # Return
///
/// This Function returns 1 for a true value and 0 for a false value
#[inline]
pub fn convert_rust_bool(value: bool) -> BOOL {
    return match value {
        true => 1,
        false => 0
    };
}

/// Converts CStrings to Windows Null-Terminated WideStrings
///
/// # Arguments
///
/// * `value`- The CString that should be converted

// TODO: Make this Conversion better
#[inline]
pub fn convert_c_to_os_wide_string(value: CString) -> Vec<u16> {
    // This Conversion seems horribly inefficient but at the same time it seems to be the only way to do this
    // It seems to be the only way to do this since i have not found a way to directly convert between CString and OSString and Rust Strings can contain inner Null Chars
    let normal_string = convert_c_string_to_normal_string(value);

    // The Resulting OSString (and with that also the resulting Vector) should be Null-Terminated if the converted string is Null Terminated
    let os_string = OsString::from(normal_string);
    return os_string.encode_wide().collect();
}

/// Converts CStrings into a Null-Terminated Rust String
///
/// # Arguments
///
/// * `value` - The CString that should be converted

// TODO: Make this Conversion better or remove the need for this function
#[inline]
fn convert_c_string_to_normal_string(value: CString) -> String {
    // This Conversion should always work since CStrings are already valid UTF-8 Strings
    // The only thing to Test is that the Resulting String is also Null-Terminated (luckily the String Constructor doesn't remove Null Chars so this works)
    return String::from_utf8(Vec::from(value.as_bytes_with_nul())).expect("Error while Converting CString");
}

/// Converts References into void Pointers
///
/// # Arguments
///
/// * `reference` - The reference that should be converted
#[inline]
pub fn convert_reference_to_pvoid<T>(reference: &mut T) -> PVOID {
    return (reference as *mut T) as PVOID;
}

/// Tests for the convert_c_string_to_normal_string Function
#[cfg(test)]
mod test_convert_c_string_to_normal_string {
    use std::ffi::CString;
    use crate::type_wrappers::type_conversion::{convert_c_string_to_normal_string};

    /// Tests that the Converted String is Null-Terminated
    #[test]
    fn test_result_is_null_terminated() {
        let c_string = CString::new("Hello World").expect("Error while Creating CString");
        let string = convert_c_string_to_normal_string(c_string);

        assert!(string.ends_with("\0"));
    }


}

/// Tests for the convert_c_string_to_wide_string Function
#[cfg(test)]
mod test_c_string_to_wide_string {
    use std::ffi::CString;
    use crate::type_wrappers::type_conversion::convert_c_to_os_wide_string;

    /// Tests that the Converted String is Null-Terminated
    #[test]
    fn test_result_is_null_terminated() {
        let c_string = CString::new("Hello World").expect("Error while Creating CString");
        let wide_string = convert_c_to_os_wide_string(c_string);

        assert!(wide_string.ends_with(&[0u16]));
    }
}