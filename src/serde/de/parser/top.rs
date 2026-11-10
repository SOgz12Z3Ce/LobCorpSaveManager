use winnow::{Parser, binary::le_i32, token::literal};

use crate::serde::{
    de::{
        Stream,
        parser::{self, element::element_parser},
    },
    raw::element::Element,
};

pub(crate) fn magic_parser(input: &mut Stream) -> winnow::Result<()> {
    parser::begin(input, "magic");

    parser::parse_next(
        input,
        literal(0x00),
        "Magic number, byte '0x00' at offset 0h.",
    )?;

    parser::parse_next(
        input,
        le_i32.verify(|i| *i == 1),
        "Magic number, little endian 32-bit signed '1' at offset 1h.",
    )?;
    parser::parse_next(
        input,
        le_i32.verify(|i| *i == -1),
        "Has header flag, little endian 32-bit signed '2' for has one, '-1' for doesn't have one. Saves always don't have headers.",
    )?;
    parser::parse_next(
        input,
        le_i32.verify(|i| *i == 1),
        "Magic number, little endian 32-bit signed '1' at offset 9h.",
    )?;
    parser::parse_next(
        input,
        le_i32.verify(|i| *i == 0),
        "Magic number, little endian 32-bit signed '0' at offset Dh.",
    )?;

    parser::end(input);
    Ok(())
}

pub(crate) fn elements_parser(input: &mut Stream) -> winnow::Result<Vec<Element>> {
    parser::begin(input, "elements");

    let mut elements = Vec::new();
    loop {
        let element = element_parser.parse_next(input)?;
        elements.push(element);
        if matches!(element, Element::End) {
            parser::end(input);
            return Ok(elements);
        }
    }
}
