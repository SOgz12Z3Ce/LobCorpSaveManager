use crate::serde::{de::log::Log, raw::id::Id};

#[derive(Debug)]
pub enum Value {
    Int(i32),
    ObjectReference(Id),
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<Id> for Value {
    fn from(value: Id) -> Self {
        Self::ObjectReference(value)
    }
}

impl Log for Value {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Value::Int(value) => value.to_le_bytes().to_vec(),
            Value::ObjectReference(id) => id.to_bytes(),
        }
    }

    fn to_log(&self) -> String {
        match self {
            Value::Int(value) => value.to_string(),
            Value::ObjectReference(id) => id.to_log(),
        }
    }
}
