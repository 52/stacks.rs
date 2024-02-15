// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#[macro_export]
macro_rules! clarity {
    (@box $e:expr) => (Box::new($e) as Box<dyn $crate::clarity::Clarity>);
    (@empty, $i:ident) => ($crate::clarity::$i::new(vec![]));
    (Int, $x:expr) => ($crate::clarity::Int::new($x));
    (UInt, $x:expr) => ($crate::clarity::UInt::new($x));
    (Buffer, $x:expr) => ($crate::clarity::Buffer::new($x.to_vec()));
    (Buffer) => (@empty, Buffer);
    (True) => ($crate::clarity::True::new());
    (False) => ($crate::clarity::False::new());
    (PrincipalStandard, $x:expr) => ($crate::clarity::PrincipalStandard::new($x.to_string()));
    (PrincipalContract, $x:expr, $y:expr) => ($crate::clarity::PrincipalContract::new(($x.to_string(), $y.to_string())));
    (ResponseOk, $x:expr) => ($crate::clarity::ResponseOk::new(clarity!(@box $x)));
    (ResponseErr, $x:expr) => ($crate::clarity::ResponseErr::new(clarity!(@box $x)));
    (OptionalSome, $x:expr) => ($crate::clarity::OptionalSome::new(clarity!(@box $x)));
    (OptionalNone) => ($crate::clarity::OptionalNone::new());
    (List) => (clarity!(@empty, List));
    (Tuple) => (clarity!(@empty, Tuple));
    (StringAscii, $x:expr) => ($crate::clarity::StringAscii::new($x.to_string()));
    (StringUtf8, $x:expr) => ($crate::clarity::StringUtf8::new($x.to_string()));
    (FnArguments) => (clarity!(@empty, FnArguments));
    (LengthPrefixedStr, $x:expr) => ($crate::clarity::LengthPrefixedStr::new($x.to_string()));
    (List $(, $args:expr)*) => ($crate::clarity::List::new(vec![$(clarity!(@box $args)),*]));
    (Tuple $(, ($key:expr, $value:expr))*) => ($crate::clarity::Tuple::new(vec![$(($key.to_string(), clarity!(@box $value))),*]));
    (FnArguments $(, $args:expr)*) => ($crate::clarity::FnArguments::new(vec![$(clarity!(@box $args)),*]));
}

macro_rules! impl_clarity_primitive {
    ($name:ident, $value:ty, $id:ident) => {
        impl_clarity_primitive!(@common $name, $value, $id);
        impl $name {
            pub fn new(__value: $value) -> Self {
                $name { __value }
            }
        }
    };
    ($name:ident, $default:expr, $value:ty, $id:ident) => {
        impl_clarity_primitive!(@common $name, $value, $id);
        impl $name {
            pub fn new() -> Self {
                $name { __value: $default }
            }
        }
        impl ::std::default::Default for $name {
            fn default() -> Self {
                $name::new()
            }
        }
    };
    (@common $name:ident, $value:ty, $id:ident) => {
        pub struct $name {
            __value: $value,
        }
        impl $name {
            pub fn value(&self) -> &$value {
                &self.__value
            }
            pub fn into_value(self) -> $value {
                self.__value
            }
        }
        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.to_string() == other.to_string()
            }
        }
        impl ::std::cmp::Eq for $name {}
        impl $crate::clarity::Any for $name {
            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }
            fn into_any(self: Box<Self>) -> Box<dyn ::std::any::Any> {
                self
            }
        }
        impl $crate::clarity::Ident for $name {
            fn id() -> u8 {
                $id
            }
        }
        impl $crate::clarity::Clarity for $name {}
    };
}

macro_rules! impl_clarity_primitive_cast {
    ($value:ty) => {
        impl $crate::clarity::Cast for $value {
            fn cast<T: $crate::clarity::Clarity>(self) -> Result<T, Error> {
                if let Ok(value) = self.into_any().downcast::<T>() {
                    Ok(*value)
                } else {
                    Err(Error::BadDowncast)
                }
            }

            fn cast_as<T: $crate::clarity::Clarity>(&self) -> Result<&T, Error> {
                if let Some(value) = self.as_any().downcast_ref::<T>() {
                    Ok(value)
                } else {
                    Err(Error::BadDowncast)
                }
            }
        }
    };
    ($name:ident, $value:ty, $type_id:ident) => {
        impl_clarity_primitive!($name, $value, $type_id);
        impl $crate::clarity::Cast for $name {
            fn cast<T: $crate::clarity::Clarity>(self) -> Result<T, Error> {
                if let Ok(value) = self.into_value().into_any().downcast::<T>() {
                    Ok(*value)
                } else {
                    Err(Error::BadDowncast)
                }
            }

            fn cast_as<T: $crate::clarity::Clarity>(&self) -> Result<&T, Error> {
                if let Some(value) = self.value().as_any().downcast_ref::<T>() {
                    Ok(value)
                } else {
                    Err(Error::BadDowncast)
                }
            }
        }
    };
}

pub(crate) use impl_clarity_primitive;
pub(crate) use impl_clarity_primitive_cast;
