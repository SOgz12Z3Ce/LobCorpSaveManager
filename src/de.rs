use crate::{error::Error, raw::File};
use winnow::{Parser, binary, token};

pub fn parse(input: Vec<u8>) -> Result<File, Error> {
    let (file, _) = (parser, token::rest).parse(&input)?;
    Ok(file)
}

pub fn parser(input: &mut &[u8]) -> winnow::Result<File> {
    magic_parser.parse_next(input)?;
    Ok(File { objects: vec![] })
}

fn magic_parser(input: &mut &[u8]) -> winnow::Result<()> {
    token::literal(0x00).parse_next(input)?;
    binary::le_i32.verify(|i| *i == 1).parse_next(input)?;
    binary::le_i32.verify(|i| *i == -1).parse_next(input)?;
    binary::le_i32.verify(|i| *i == 1).parse_next(input)?;
    binary::le_i32.verify(|i| *i == 0).parse_next(input)?;

    Ok(())
}
