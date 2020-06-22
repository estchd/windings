#[macro_export]
macro_rules! CONST_TO_ENUM {
    {const_enum $name:ident, $original:path {$($new:ident = $old:path,)*}} =>
    {
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
            type Error = ();

            fn try_from(value: $original) -> Result<Self, Self::Error> {
                return match value {
                    $($old => Ok($name::$new),)*
                    _ => Err(()),
                }
            }
        }
    }
}