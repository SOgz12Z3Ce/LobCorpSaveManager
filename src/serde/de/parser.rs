use winnow::{Parser, binary::le_i32, token::literal};

#[cfg(feature = "log")]
use crate::serde::de::log::Log;
use crate::serde::{de::Stream, raw::File};

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

// fn parse_next_with_log<'a, P, O, E>(
//     input: &mut &'a Stream,
//     parser: P,
//     explaination: &str,
// ) -> Result<O, E>
// where
//     P: Parser<&'a Stream<'a>, O, E>,
//     O: Log,
// {
//     let value = parser.parse_next(input)?;
//     #[cfg(feature = "log")]
//     input.state.log_engine.log(value, explaination);
//     Ok(value)
// }

pub(crate) fn parser(input: &mut Stream) -> winnow::Result<File> {
    begin(input, "main");

    magic_parser.parse_next(input)?;
    let elements = Vec::new();

    end(input);
    Ok(File::new(elements))
}

fn magic_parser(input: &mut Stream) -> winnow::Result<()> {
    begin(input, "magic");

    parse_next(
        input,
        literal(0x00),
        "Magic number, byte '0x00' at offset 0h.",
    )?;

    parse_next(
        input,
        le_i32.verify(|i| *i == 1),
        "Magic number, little endian 32-bit signed '1' at offset 1h.",
    )?;
    parse_next(
        input,
        le_i32.verify(|i| *i == -1),
        "Magic number, little endian 32-bit signed '-1' at offset 5h.",
    )?;
    parse_next(
        input,
        le_i32.verify(|i| *i == 1),
        "Magic number, little endian 32-bit signed '-1' at offset 9h.",
    )?;
    parse_next(
        input,
        le_i32.verify(|i| *i == 0),
        "Magic number, little endian 32-bit signed '0' at offset Dh.",
    )?;

    end(input);
    Ok(())
}
