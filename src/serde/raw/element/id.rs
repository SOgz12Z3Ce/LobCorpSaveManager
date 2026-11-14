use crate::error::Error;

// In C#, ID is processed as i64, written as i32 and read as u32.
// ID is generated with 1i64 beginning and consecutively increases by 1.
// C# will throw an exception when the casting overflow. So ID should be
// >= 1 (initial value of generator) and <= i32::MAX (or throw exception when
// writing).
#[derive(Debug)]
pub(crate) struct Id(u32);

impl TryFrom<u32> for Id {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (1..=i32::MAX as u32).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Self::Error::InvalidID(value))
        }
    }
}
