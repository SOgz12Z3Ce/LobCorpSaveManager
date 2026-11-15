use crate::error::Error;
#[cfg(feature = "log")]
use crate::serde::de::log::Log;

// C# i32 count.
#[derive(Debug)]
pub(crate) struct Count(usize);

impl TryFrom<i32> for Count {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(Self(value as usize))
        } else {
            Err(Self::Error::InvalidCount(value))
        }
    }
}

impl From<Count> for usize {
    fn from(value: Count) -> Self {
        value.0
    }
}

#[cfg(feature = "log")]
impl Log for Count {
    fn to_bytes(&self) -> Vec<u8> {
        (self.0 as i32).to_le_bytes().to_vec()
    }

    fn to_log(&self) -> String {
        self.0.to_string()
    }
}
