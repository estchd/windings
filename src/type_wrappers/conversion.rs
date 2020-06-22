use winapi::shared::minwindef::BOOL;
use std::ffi::{OsString, CString};
use std::os::windows::prelude::*;

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

#[inline]
pub fn convert_bool(value: BOOL) -> bool {
    return value != 0;
}

#[inline]
pub fn convert_c_to_os_wide_string(value: CString) -> Vec<u16> {
    //This Conversion seems horribly inefficient but at the same time it seems to be the only way to do this
    //TODO: Make this Conversion better
    let normal_string = convert_c_string_to_normal_string(value);
    let os_string = OsString::from(normal_string);
    return os_string.encode_wide().collect();
}

#[inline]
fn convert_c_string_to_normal_string(value: CString) -> String {
    //TODO: Make this Conversion better
    return String::from_utf8(Vec::from(value.as_bytes_with_nul())).expect("Error while Converting CString");
}

#[cfg(test)]
mod test {
    use std::ffi::CString;
    use crate::type_wrappers::conversion::{convert_c_string_to_normal_string, convert_c_to_os_wide_string};

    #[test]
    fn test_c_string_to_normal_string_ends_with_null() {
        let c_string = CString::new("Hello World").expect("Error while Creating CString");
        let string = convert_c_string_to_normal_string(c_string);

        assert!(string.ends_with("\0"));
    }

    #[test]
    fn test_c_string_to_wide_string_ends_with_null() {
        let c_string = CString::new("Hello World").expect("Error while Creating CString");
        let wide_string = convert_c_to_os_wide_string(c_string);

        assert!(wide_string.ends_with(&[0u16]));
    }
}