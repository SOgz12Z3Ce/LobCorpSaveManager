#[derive(Debug)]
pub enum Value {
    Int(i32),
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}
