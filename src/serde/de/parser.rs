use winnow::{Parser, binary::le_i32, token::literal};

use crate::serde::{de::Stream, raw::File};

pub(crate) fn parser(input: &mut Stream) -> winnow::Result<File> {
    #[cfg(feature = "log")]
    input.state.log_engine.begin("main");

    magic_parser.parse_next(input)?;
    let elements = Vec::new();

    #[cfg(feature = "log")]
    input.state.log_engine.end();
    Ok(File::new(elements))
}

fn magic_parser(input: &mut Stream) -> winnow::Result<()> {
    #[cfg(feature = "log")]
    input.state.log_engine.begin("magic");

    literal(0x00).parse_next(input)?;
    #[cfg(feature = "log")]
    input
        .state
        .log_engine
        .log(&[0x00], "Magic number, byte '0x00' at offset 0h.");

    let magic = le_i32.verify(|i| *i == 1).parse_next(input)?;
    #[cfg(feature = "log")]
    input.state.log_engine.log(
        &magic.to_le_bytes(),
        "Magic number, little endian 32-bit signed '1' at offset 1h.",
    );

    let magic = le_i32.verify(|i| *i == -1).parse_next(input)?;
    #[cfg(feature = "log")]
    input.state.log_engine.log(
        &magic.to_le_bytes(),
        "Magic number, little endian 32-bit signed '-1' at offset 5h.",
    );

    let magic = le_i32.verify(|i| *i == 1).parse_next(input)?;
    #[cfg(feature = "log")]
    input.state.log_engine.log(
        &magic.to_le_bytes(),
        "Magic number, little endian 32-bit signed '1' at offset 9h",
    );

    let magic = le_i32.verify(|i| *i == 0).parse_next(input)?;
    #[cfg(feature = "log")]
    input.state.log_engine.log(
        &magic.to_le_bytes(),
        "Magic number, little endian 32-bit signed '0' at offset Dh",
    );

    #[cfg(feature = "log")]
    input.state.log_engine.end();
    Ok(())
}
