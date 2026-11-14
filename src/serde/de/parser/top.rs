use winnow::{Parser, binary::le_i32, token::literal};

use crate::serde::{
    de::{
        Stream,
        parser::{self, element::element_parser},
    },
    raw::element::Element,
};

pub(crate) fn magic_parser(input: &mut Stream) -> winnow::Result<()> {
    let mut input = parser::prologue(input, "magic");

    parser::explain(
        &mut input,
        move |input| literal(0x00).parse_next(input),
        "Magic number. Byte '0x00' at offset '0h'.",
    )?;

    parser::explain(
        &mut input,
        move |input| le_i32.verify(|i| *i == 1).parse_next(input),
        "Magic number. Little endian 32-bit signed '1' at offset '1h'.",
    )?;
    parser::explain(
        &mut input,
        move |input| le_i32.verify(|i| *i == -1).parse_next(input),
        "Flag for having a header. Little endian 32-bit signed '2' for having a header, '-1' for not. Saves never have any header.",
    )?;
    parser::explain(
        &mut input,
        move |input| le_i32.verify(|i| *i == 1).parse_next(input),
        "Magic number. Little endian 32-bit signed '1' at offset '9h'.",
    )?;
    parser::explain(
        &mut input,
        move |input| le_i32.verify(|i| *i == 0).parse_next(input),
        "Magic number. Little endian 32-bit signed '0' at offset 'Dh'.",
    )?;

    Ok(())
}

pub(crate) fn elements_parser(input: &mut Stream) -> winnow::Result<Vec<Element>> {
    let mut input = parser::prologue(input, "elements");

    let mut elements = Vec::new();
    loop {
        let element = element_parser.parse_next(&mut input)?;
        if matches!(element, Element::End) {
            elements.push(element);
            return Ok(elements);
        }
        elements.push(element);
    }
}
