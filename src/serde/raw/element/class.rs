use std::str::FromStr;

use strum_macros::{Display, EnumString};

use crate::error::Error;
#[cfg(feature = "log")]
use crate::serde::de::log::Log;

// C# Class.
#[allow(unused)] // TODO: We will comment out all unused variants in the end.
#[derive(Debug, Clone, Copy, Display, EnumString)]
pub(crate) enum Class {
    // Primitive classes.

    // Notice there is no 0th primitive type.
    #[strum(to_string = "bool")]
    #[strum(disabled)]
    Bool,

    #[strum(to_string = "byte")]
    #[strum(disabled)]
    Byte,

    #[strum(to_string = "char")]
    #[strum(disabled)]
    Char,

    // Notice there is no 4th primitive type.
    #[strum(to_string = "decimal")]
    #[strum(disabled)]
    Decimal,

    #[strum(to_string = "double")]
    #[strum(disabled)]
    Double,

    #[strum(to_string = "short")]
    #[strum(disabled)]
    Short,

    #[strum(to_string = "int")]
    #[strum(disabled)]
    Int,

    #[strum(to_string = "long")]
    #[strum(disabled)]
    Long,

    #[strum(to_string = "sbyte")]
    #[strum(disabled)]
    SByte,

    #[strum(to_string = "float")]
    #[strum(disabled)]
    Float,

    #[strum(disabled)]
    TimeSpan,

    #[strum(disabled)]
    DateTime,

    #[strum(to_string = "ushort")]
    #[strum(disabled)]
    UShort,

    #[strum(to_string = "uint")]
    #[strum(disabled)]
    UInt,

    #[strum(to_string = "ulong")]
    #[strum(disabled)]
    ULong,

    #[strum(to_string = "null")]
    #[strum(disabled)]
    Null,

    #[strum(to_string = "string")]
    #[strum(disabled)]
    String,

    // Runtime classes.
    #[strum(to_string = "Dictionary<string, object>")]
    #[strum(
        serialize = "System.Collections.Generic.Dictionary`2[[System.String, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089],[System.Object, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]"
    )]
    Dictinoary,

    // TODO: #[strum(to_string = "?")]
    #[strum(
        serialize = "System.Collections.Generic.GenericEqualityComparer`1[[System.String, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]"
    )]
    GenericEqualityComparer,

    // TODO: #[strum(to_string = "?")]
    #[strum(
        serialize = "System.Collections.Generic.KeyValuePair`2[[System.String, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089],[System.Object, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]][]"
    )]
    KeyValuePairArray,

    // TODO: #[strum(to_string = "?")]
    #[strum(
        serialize = "System.Collections.Generic.KeyValuePair`2[[System.String, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089],[System.Object, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]"
    )]
    KeyValuePair,
}

impl Class {
    // From the constructor of BinaryCommon (@06002AE8).
    pub(crate) fn primitive(code: u8) -> Result<Self, Error> {
        match code {
            1 => Ok(Self::Bool),
            2 => Ok(Self::Byte),
            3 => Ok(Self::Char),
            5 => Ok(Self::Decimal),
            6 => Ok(Self::Double),
            7 => Ok(Self::Short),
            8 => Ok(Self::Int),
            9 => Ok(Self::Long),
            10 => Ok(Self::SByte),
            11 => Ok(Self::Float),
            12 => Ok(Self::TimeSpan),
            13 => Ok(Self::DateTime),
            14 => Ok(Self::UShort),
            15 => Ok(Self::UInt),
            16 => Ok(Self::ULong),
            17 => Ok(Self::Null),
            18 => Ok(Self::String),
            _ => Err(Error::InvalidPrimitiveClass(code)),
        }
    }

    pub(crate) fn runtime(name: &str) -> Result<Self, Error> {
        Self::from_str(name).map_err(|_| Error::InvalidRuntimeClass(name.to_owned()))
    }
}

#[cfg(feature = "log")]
impl Log for Class {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            Class::Bool => unimplemented!("this class was considered to be non-existent in saves"),
            Class::Byte => unimplemented!("this class was considered to be non-existent in saves"),
            Class::Char => unimplemented!("this class was considered to be non-existent in saves"),
            Class::Decimal => {
                unimplemented!("this class was considered to be non-existent in saves")
            }
            Class::Double => {
                unimplemented!("this class was considered to be non-existent in saves")
            }
            Class::Short => unimplemented!("this class was considered to be non-existent in saves"),
            Class::Int => unimplemented!("this class was considered to be non-existent in saves"),
            Class::Long => unimplemented!("this class was considered to be non-existent in saves"),
            Class::SByte => unimplemented!("this class was considered to be non-existent in saves"),
            Class::Float => unimplemented!("this class was considered to be non-existent in saves"),
            Class::TimeSpan => {
                unimplemented!("this class was considered to be non-existent in saves")
            }
            Class::DateTime => {
                unimplemented!("this class was considered to be non-existent in saves")
            }
            Class::UShort => {
                unimplemented!("this class was considered to be non-existent in saves")
            }
            Class::UInt => unimplemented!("this class was considered to be non-existent in saves"),
            Class::ULong => unimplemented!("this class was considered to be non-existent in saves"),
            Class::Null => unimplemented!("this class was considered to be non-existent in saves"),
            Class::String => {
                unimplemented!("this class was considered to be non-existent in saves")
            }
            Class::Dictinoary
            | Class::GenericEqualityComparer
            | Class::KeyValuePairArray
            | Class::KeyValuePair => self.to_string().into_bytes(),
        }
    }

    fn to_log(&self) -> String {
        self.to_string()
    }
}
