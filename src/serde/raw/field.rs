use strum_macros::Display;

use crate::error::Error;
#[cfg(feature = "log")]
use crate::serde::de::log::Log;

// TypeTag @0200045D
#[derive(Debug, Clone, Copy, Display)]
pub(crate) enum FieldKind {
    PrimitiveType,
    String,
    ObjectType,
    RuntimeType,
    GenericType,
    ArrayOfObject,
    ArrayOfString,
    ArrayOfPrimitiveType,
}

impl TryFrom<u8> for FieldKind {
    type Error = Error;

    // From the constructor of BinaryCommon (@06002AE8).
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PrimitiveType),
            1 => Ok(Self::String),
            2 => Ok(Self::ObjectType),
            3 => Ok(Self::RuntimeType),
            4 => Ok(Self::GenericType),
            5 => Ok(Self::ArrayOfObject),
            6 => Ok(Self::ArrayOfString),
            7 => Ok(Self::ArrayOfPrimitiveType),
            _ => Err(Self::Error::InvalidFieldKind(value)),
        }
    }
}

#[cfg(feature = "log")]
impl Log for FieldKind {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            FieldKind::PrimitiveType => vec![0x00],
            FieldKind::String => vec![0x01],
            FieldKind::ObjectType => vec![0x02],
            FieldKind::RuntimeType => vec![0x03],
            FieldKind::GenericType => vec![0x04],
            FieldKind::ArrayOfObject => vec![0x05],
            FieldKind::ArrayOfString => vec![0x06],
            FieldKind::ArrayOfPrimitiveType => vec![0x07],
        }
    }

    fn to_log(&self) -> String {
        self.to_string()
    }
}
