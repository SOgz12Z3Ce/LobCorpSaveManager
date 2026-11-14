use winnow::{
    Parser,
    binary::u8,
    error::{ContextError, FromExternalError},
};

use crate::serde::{
    de::{Stream, parser},
    raw::element::{Element, ElementKind},
};

pub(crate) fn element_parser(input: &mut Stream) -> winnow::Result<Element> {
    parser::prologue(input, "element");

    element_kind_parser.parse_next(input)?;

    todo!()
}

fn element_kind_parser(input: &mut Stream) -> winnow::Result<ElementKind> {
    parser::prologue(input, "kind");

    let kind = parser::explain(
        input,
        move |input| {
            let kind = u8.parse_next(input)?;
            let kind = ElementKind::try_from(kind);
            kind.map_err(|error| ContextError::from_external_error(input, error))
        },
        "Element kind. Every element is leaded with this tag.",
    )?;

    Ok(kind)
}
