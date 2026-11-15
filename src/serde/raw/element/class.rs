use strum_macros::Display;

#[cfg(feature = "log")]
use crate::serde::de::log::Log;
use crate::{error::Error, serde::raw::element::leb128_str::Leb128Str};

// C# class.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Class {
    Primitive(PrimitiveClass),
    Runtime(RuntimeClass),
}

impl Class {
    pub(crate) fn primitive(code: u8) -> Result<Self, Error> {
        let class = PrimitiveClass::try_from(code)?;
        Ok(Self::Primitive(class))
    }

    pub(crate) fn runtime(name: Leb128Str) -> Result<Self, Error> {
        let class = RuntimeClass::try_from_full_name(name)
            .map_err(|_| Error::InvalidRuntimeClass(name.to_string()))?;
        Ok(Self::Runtime(class))
    }
}

#[cfg(feature = "log")]
impl Log for Class {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Class::Primitive(primitive_class) => primitive_class.to_bytes(),
            Class::Runtime(runtime_class) => runtime_class.to_bytes(),
        }
    }

    fn to_log(&self) -> String {
        match self {
            Class::Primitive(primitive_class) => primitive_class.to_log(),
            Class::Runtime(runtime_class) => runtime_class.to_log(),
        }
    }
}

// C# primitive class.
#[derive(Debug, Clone, Copy, Display)]
pub(crate) enum PrimitiveClass {
    //
    // Notice there is no the 0th primitive type.
    //
    #[strum(to_string = "bool")]
    Bool,

    #[strum(to_string = "byte")]
    Byte,

    #[strum(to_string = "char")]
    Char,

    //
    // Notice there is no the 4th primitive type.
    //
    #[strum(to_string = "decimal")]
    Decimal,

    #[strum(to_string = "double")]
    Double,

    #[strum(to_string = "short")]
    Short,

    #[strum(to_string = "int")]
    Int,

    #[strum(to_string = "long")]
    Long,

    #[strum(to_string = "sbyte")]
    SByte,

    #[strum(to_string = "float")]
    Float,

    TimeSpan,

    DateTime,

    #[strum(to_string = "ushort")]
    UShort,

    #[strum(to_string = "uint")]
    UInt,

    #[strum(to_string = "ulong")]
    ULong,

    #[strum(to_string = "null")]
    Null,

    #[strum(to_string = "string")]
    String,
}

impl TryFrom<u8> for PrimitiveClass {
    type Error = Error;

    // From the constructor of BinaryCommon (@06002AE8).
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
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
            _ => Err(Self::Error::InvalidPrimitiveClass(value)),
        }
    }
}

#[cfg(feature = "log")]
impl Log for PrimitiveClass {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Bool => unimplemented!("this class was considered to be non-existent in saves"),
            Self::Byte => unimplemented!("this class was considered to be non-existent in saves"),
            Self::Char => unimplemented!("this class was considered to be non-existent in saves"),
            Self::Decimal => {
                unimplemented!("this class was considered to be non-existent in saves")
            }
            Self::Double => unimplemented!("this class was considered to be non-existent in saves"),
            Self::Short => unimplemented!("this class was considered to be non-existent in saves"),
            Self::Int => unimplemented!("this class was considered to be non-existent in saves"),
            Self::Long => unimplemented!("this class was considered to be non-existent in saves"),
            Self::SByte => unimplemented!("this class was considered to be non-existent in saves"),
            Self::Float => unimplemented!("this class was considered to be non-existent in saves"),
            Self::TimeSpan => {
                unimplemented!("this class was considered to be non-existent in saves")
            }
            Self::DateTime => {
                unimplemented!("this class was considered to be non-existent in saves")
            }
            Self::UShort => unimplemented!("this class was considered to be non-existent in saves"),
            Self::UInt => unimplemented!("this class was considered to be non-existent in saves"),
            Self::ULong => unimplemented!("this class was considered to be non-existent in saves"),
            Self::Null => unimplemented!("this class was considered to be non-existent in saves"),
            Self::String => unimplemented!("this class was considered to be non-existent in saves"),
        }
    }

    fn to_log(&self) -> String {
        self.to_string()
    }
}

// C# runtime class.
#[derive(Debug, Clone, Copy, Display)]
pub(crate) enum RuntimeClass {
    #[strum(to_string = "Dictionary<string, object>")]
    Dictinoary,

    // TODO: #[strum(to_string = "?")]
    GenericEqualityComparer,

    // TODO: #[strum(to_string = "?")]
    KeyValuePairArray,

    // TODO: #[strum(to_string = "?")]
    KeyValuePair,
}

impl RuntimeClass {
    const ALL: [Self; 4] = [
        Self::Dictinoary,
        Self::GenericEqualityComparer,
        Self::KeyValuePairArray,
        Self::KeyValuePair,
    ];

    fn full_name(&self) -> Leb128Str<'static> {
        let name = match self {
            Self::Dictinoary => {
                "System.Collections.Generic.Dictionary`2[[System.String, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089],[System.Object, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]"
            }
            Self::GenericEqualityComparer => {
                "System.Collections.Generic.GenericEqualityComparer`1[[System.String, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]"
            }
            Self::KeyValuePairArray => {
                "System.Collections.Generic.KeyValuePair`2[[System.String, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089],[System.Object, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]][]"
            }
            Self::KeyValuePair => {
                "System.Collections.Generic.KeyValuePair`2[[System.String, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089],[System.Object, mscorlib, Version=2.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]"
            }
        };
        Leb128Str::from(name)
    }

    fn try_from_full_name(full_name: Leb128Str) -> Result<Self, Error> {
        for class in Self::ALL {
            if class.full_name() == full_name {
                return Ok(class);
            }
        }
        Err(Error::InvalidRuntimeClass(full_name.to_string()))
    }
}

#[cfg(feature = "log")]
impl Log for RuntimeClass {
    fn to_bytes(&self) -> Vec<u8> {
        self.full_name().to_bytes()
    }

    fn to_log(&self) -> String {
        self.to_string()
    }
}
