use itertools::Itertools;

use crate::serde::de::log::Log;

impl Log for &[u8] {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }

    fn to_log(&self) -> String {
        if self.len() == 1 {
            format!("0x{:02X}", self[0])
        } else {
            String::from("[") + &self.iter().map(|i| format!("0x{i:02X}")).join(", ") + "]"
        }
    }
}

impl Log for i32 {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn to_log(&self) -> String {
        self.to_string()
    }
}
