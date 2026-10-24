mod combinator;

#[derive(Debug, Default)]
pub(crate) struct LogEngine {
    parsers: Vec<&'static str>,
    offset: usize,
    before: Vec<u8>,
}

impl LogEngine {
    pub(crate) fn begin(&mut self, parser: &'static str) {
        self.parsers.push(parser);
    }

    pub(crate) fn end(&mut self) {
        self.parsers.pop();
    }

    fn update(&mut self, byte: u8) {
        self.offset += 1;
        self.before.push(byte);
        if self.before.len() == 16 {
            self.before.clear();
        }
    }

    pub(crate) fn log(&mut self, bytes: &[u8], message: &str) {
        // Header
        println!("         00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");

        // Bytes
        print!("{:08X}", self.offset / 16 * 16);
        for byte in &self.before {
            print!(" \x1b[90m{byte:02X}\x1b[0m");
        }
        for byte in &bytes[..bytes.len() - 1] {
            print!(" \x1b[93m{:02X}\x1b[0m", byte);
            self.update(*byte);
            if self.offset % 16 == 0 {
                println!("");
                print!("{:08X}", self.offset / 16 * 16);
            }
        }
        let last_byte = bytes[bytes.len() - 1];
        print!(" \x1b[93m{:02X}\x1b[0m", last_byte);
        self.update(last_byte);
        for _ in 0..(16 - self.offset % 16) {
            print!(" \x1b[90m??\x1b[0m");
        }
        println!("");
        println!("");

        // Parsers
        print!("  ");
        println!("{}", self.parsers.join("/"));

        // Message
        println!("    {message}");
        println!("--------------------------------------------------------");
    }
}
