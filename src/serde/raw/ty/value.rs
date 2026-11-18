use crate::serde::de::log::Log;

#[derive(Debug)]
pub enum Value {
    Int(i32),
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl Log for Value {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Value::Int(value) => value.to_le_bytes().to_vec(),
        }
    }

    fn to_log(&self) -> String {
        match self {
            Value::Int(value) => value.to_string(),
        }
    }
}
