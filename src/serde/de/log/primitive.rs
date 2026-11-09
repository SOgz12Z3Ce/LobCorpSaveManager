use crate::serde::de::log::Log;

impl Log for &[u8] {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }

    fn to_log(&self) -> String {
        if self.len() == 1 {
            format!("{:02X}", self[0])
        } else {
            let s = String::from("[");
            let s = self
                .iter()
                .map(|i| format!("{i:02X}"))
                .fold(s, |acc, i| acc + &i + ", ");
            s + "]"
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
