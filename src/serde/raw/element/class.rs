use crate::error::Error;

// C# Class.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Class {
    // Primitive classes.
    // Notice there is no 0th primitive type.
    Bool,
    Byte,
    Char,
    // Notice there is no 4th primitive type.
    Decimal,
    Double,
    Short,
    Int,
    Long,
    SByte,
    Float,
    TimeSpan,
    DateTime,
    UShort,
    UInt,
    ULong,
    Null,
    String,

    // Runtime classes.
    Object,
    Dictinoary,
    GenericEqualityComparer,
    KeyValuePairArray,
    KeyValuePair,
}

impl Class {
    // From the constructor of BinaryCommon (@06002AE8).
    fn primitive(code: u8) -> Result<Self, Error> {
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

    fn runtime(name: &str) -> Result<Self, Error> {
        match name {
            "System.Collections.Generic.Dictionary`2[[System.String, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089],[System.Object, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]" => {
                Ok(Self::Dictinoary)
            }
            "System.Collections.Generic.GenericEqualityComparer`1[[System.String, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]" => {
                Ok(Self::GenericEqualityComparer)
            }
            "System.Collections.Generic.KeyValuePair`2[[System.String, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089],[System.Object, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]][]" => {
                Ok(Self::KeyValuePairArray)
            }
            "System.Collections.Generic.KeyValuePair`2[[System.String, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089],[System.Object, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]" => {
                Ok(Self::KeyValuePair)
            }
            _ => Err(Error::InvalidRuntimeClass(name.to_owned())),
        }
    }
}
