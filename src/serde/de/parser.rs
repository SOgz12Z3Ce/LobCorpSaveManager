use winnow::{Parser, binary::le_i32, token::literal};

use crate::serde::{de::Stream, raw::File};

pub(crate) fn parser(input: &mut Stream) -> winnow::Result<File> {
    #[cfg(feature = "log")]
    input.state.log.begin("main");

    magic_parser.parse_next(input)?;
    let elements = Vec::new();

    #[cfg(feature = "log")]
    input.state.log.end();
    Ok(File::new(elements))
}

fn magic_parser(input: &mut Stream) -> winnow::Result<()> {
    #[cfg(feature = "log")]
    input.state.log.begin("magic");

    literal(0x00).parse_next(input)?;
    #[cfg(feature = "log")]
    input.state.log.log(&[0x00, 0x01, 0x02], "test parser");
    #[cfg(feature = "log")]
    input.state.log.log(&[0x00, 0x01, 0x02], "test parser");
    #[cfg(feature = "log")]
    input.state.log.log(
        &[
            0x00, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01,
            0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01,
            0x02,
        ],
        "test parser",
    );

    le_i32.verify(|i| *i == 1).parse_next(input)?;
    le_i32.verify(|i| *i == -1).parse_next(input)?;
    le_i32.verify(|i| *i == 1).parse_next(input)?;
    le_i32.verify(|i| *i == 0).parse_next(input)?;

    #[cfg(feature = "log")]
    input.state.log.end();
    Ok(())
}
