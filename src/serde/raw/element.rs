use strum_macros::Display;

use crate::error::Error;
#[cfg(feature = "log")]
use crate::serde::de::log::Log;

// BinaryElement @0200045C
#[derive(Debug, Clone, Copy, Display)]
pub(crate) enum ElementKind {
    Header,
    RefTypeObject,
    UntypedRuntimeObject,
    UntypedExternalObject,
    RuntimeObject,
    ExternalObject,
    String,
    GenericArray,
    BoxedPrimitiveTypeValue,
    ObjectReference,
    NullValue,
    End,
    Assembly,
    ArrayFiller8b,
    ArrayFiller32b,
    ArrayOfPrimitiveType,
    ArrayOfObject,
    ArrayOfString,
    Method,
    _Unknown4,
    _Unknown5,
    MethodCall,
    MethodResponse,
}

impl TryFrom<u8> for ElementKind {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Header),
            1 => Ok(Self::RefTypeObject),
            2 => Ok(Self::UntypedRuntimeObject),
            3 => Ok(Self::UntypedExternalObject),
            4 => Ok(Self::RuntimeObject),
            5 => Ok(Self::ExternalObject),
            6 => Ok(Self::String),
            7 => Ok(Self::GenericArray),
            8 => Ok(Self::BoxedPrimitiveTypeValue),
            9 => Ok(Self::ObjectReference),
            10 => Ok(Self::NullValue),
            11 => Ok(Self::End),
            12 => Ok(Self::Assembly),
            13 => Ok(Self::ArrayFiller8b),
            14 => Ok(Self::ArrayFiller32b),
            15 => Ok(Self::ArrayOfPrimitiveType),
            16 => Ok(Self::ArrayOfObject),
            17 => Ok(Self::ArrayOfString),
            18 => Ok(Self::Method),
            19 => Ok(Self::_Unknown4),
            20 => Ok(Self::_Unknown5),
            21 => Ok(Self::MethodCall),
            22 => Ok(Self::MethodResponse),
            _ => Err(Error::InvalidElementKind(value)),
        }
    }
}

#[cfg(feature = "log")]
impl Log for ElementKind {
    fn as_bytes(&self) -> Vec<u8> {
        vec![*self as u8]
    }

    fn to_log(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug)]
pub(crate) enum Element {
    End,
}
