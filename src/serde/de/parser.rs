mod magic;

use winnow::Parser;

#[cfg(feature = "log")]
use crate::serde::de::log::Log;
use crate::serde::{
    de::{Stream, parser::magic::magic_parser},
    raw::File,
};

fn begin(_input: &mut Stream, _parser: &'static str) {
    #[cfg(feature = "log")]
    _input.state.log_engine.begin(_parser);
}

fn end(_input: &mut Stream) {
    #[cfg(feature = "log")]
    _input.state.log_engine.end();
}

#[cfg(not(feature = "log"))]
fn parse_next<'a, 'b, P, O, E>(input: &'a mut Stream<'b>, mut parser: P, _: &str) -> Result<O, E>
where
    P: Parser<Stream<'b>, O, E>,
{
    parser.parse_next(input)
}

#[cfg(feature = "log")]
fn parse_next<'a, 'b, P, O, E>(
    input: &'a mut Stream<'b>,
    mut parser: P,
    explaination: &str,
) -> Result<O, E>
where
    P: Parser<Stream<'b>, O, E>,
    O: Log,
{
    let value = parser.parse_next(input)?;
    input.state.log_engine.log(&value, explaination);
    Ok(value)
}

pub(crate) fn parser(input: &mut Stream) -> winnow::Result<File> {
    begin(input, "main");

    magic_parser.parse_next(input)?;
    let elements = Vec::new();

    end(input);
    Ok(File::new(elements))
}
