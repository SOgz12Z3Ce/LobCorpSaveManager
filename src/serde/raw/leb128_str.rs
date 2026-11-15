#[cfg(feature = "log")]
use crate::serde::de::log::Log;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl<'a> ToString for Leb128Str<'a> {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

#[cfg(feature = "log")]
impl<'a> Log for Leb128Str<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        encode_leb128(self.0.to_owned())
    }

    fn to_log(&self) -> String {
        self.0.to_owned()
    }
}

pub(crate) struct Leb128String(String);

impl From<String> for Leb128String {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Leb128String> for String {
    fn from(value: Leb128String) -> Self {
        value.0
    }
}

#[cfg(feature = "log")]
impl Log for Leb128String {
    fn to_bytes(&self) -> Vec<u8> {
        self.0.clone().into_bytes()
    }

    fn to_log(&self) -> String {
        self.0.clone()
    }
}

#[cfg(feature = "log")]
fn encode_leb128(value: String) -> Vec<u8> {
    let string = value.into_bytes();
    let mut length = string.len();
    let mut length = {
        let mut bytes = Vec::new();
        loop {
            let section = (length & 0b01111111) as u8;
            length >>= 7;
            let end = length == 0;
            if end {
                bytes.push(section);
                break bytes;
            } else {
                bytes.push(section | 0b10000000);
            }
        }
    };
    length.extend(string);
    length
}
