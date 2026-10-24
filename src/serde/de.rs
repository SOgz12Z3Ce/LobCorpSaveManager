mod parser;

use winnow::{Parser, Stateful, token::rest};

use crate::{
    error::Error,
    serde::{de::parser::parser, raw::File},
};

#[derive(Debug, Default)]
struct State {}

type Stream<'a> = Stateful<&'a [u8], State>;

pub(crate) fn parse(content: &[u8]) -> Result<File, Error> {
    let input_len = content.len();
    let input = Stream {
        input: content,
        state: State::default(),
    };
    let (file, rest) = (parser, rest).parse(input)?;
    if !rest.is_empty() {
        let rest_len = rest.len();
        eprintln!(
            "[WARNING] Content didn't be fully consumed: {} byte{} remained, from offset 0x{:08X}.",
            rest_len,
            if rest_len == 1 { "" } else { "s" },
            input_len - rest_len,
        );
    }
    Ok(file)
}
