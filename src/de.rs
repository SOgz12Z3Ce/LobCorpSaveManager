use crate::{error::Error, raw::File};
use winnow::{
    Parser,
    error::{StrContext, StrContextValue},
    token::literal,
};

pub fn parse(input: Vec<u8>) -> Result<File, Error> {
    Ok(parser.parse(&input)?)
}

pub fn parser(input: &mut &[u8]) -> winnow::Result<File> {
    literal(0x00)
        .context(StrContext::Label("magic number"))
        .context(StrContext::Expected(StrContextValue::Description("magic")))
        .parse_next(input)?;
    Ok(File { objects: vec![] })
}
