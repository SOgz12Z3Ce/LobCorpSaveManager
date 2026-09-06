use crate::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub struct File {
    pub elements: Vec<Element>,
}

#[derive(Debug)]
pub enum Element {
    RefTypeObject {
        id: Id,
        ref_id: Id,
    },
    RuntimeObject {
        id: Id,
        class: Class,
        field_count: usize,
        field_names: Vec<String>,
        field_kinds: Vec<FieldKind>,
        field_classes: Vec<Class>,
        field_values: Vec<FieldValue>,
    },
    String {
        id: Id,
        value: String,
    },
    GenericArray {
        id: Id,
        rank: usize,
        length: usize,
        kind: FieldKind,
        class: Class,
        values: Vec<FieldValue>,
    },
    BoxedPrimitiveTypeValue {
        class: Class,
        value: Box<FieldValue>,
    },
    ObjectReference {
        id: Id,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum ElementKind {
    RefTypeObject,
    RuntimeObject,
    String,
    GenericArray,
    BoxedPrimitiveTypeValue,
    ObjectReference,
}

impl TryFrom<u8> for ElementKind {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::RefTypeObject),
            0x04 => Ok(Self::RuntimeObject),
            0x06 => Ok(Self::String),
            0x07 => Ok(Self::GenericArray),
            0x08 => Ok(Self::BoxedPrimitiveTypeValue),
            0x09 => Ok(Self::ObjectReference),
            _ => Err(Self::Error::InvalidElementKind(value)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Id(u64);

impl TryFrom<u32> for Id {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Err(Self::Error::InvalidID),
            _ => Ok(Self(value.into())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FieldKind {
    PrimitiveType,
    String,
    ObjectType,
    RuntimeType,
}

impl TryFrom<u8> for FieldKind {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::PrimitiveType),
            0x01 => Ok(Self::String),
            0x02 => Ok(Self::ObjectType),
            0x03 => Ok(Self::RuntimeType),
            _ => Err(Self::Error::InvalidFieldKind(value)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Class {
    Int,
    Float,
    String,
    Object,
    Dictinoary,
    GenericEqualityComparer,
    KeyValuePairArray,
    KeyValuePair,
}

impl TryFrom<u8> for Class {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x08 => Ok(Self::Int),
            0x0B => Ok(Self::Float),
            _ => Err(Self::Error::InvalidPrimitiveClass(value)),
        }
    }
}

impl FromStr for Class {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
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
            _ => Err(Self::Err::InvalidRuntimeClass(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum FieldValue {
    Int(i32),
    Float(f32),
    String(String),
    Object(Element),
    Reference(Id),
}
