#[cfg(feature = "log")]
use crate::serde::de::log::Log;

#[derive(Debug)]
pub(crate) struct Leb128Str<'a>(&'a str);

impl<'a> From<&'a str> for Leb128Str<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl<'a> From<Leb128Str<'a>> for &'a str {
    fn from(value: Leb128Str<'a>) -> Self {
        value.0
    }
}

#[cfg(feature = "log")]
impl<'a> Log for Leb128Str<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_owned().into_bytes()
    }

    fn to_log(&self) -> String {
        self.0.to_owned()
    }
}
