mod util;

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
    let mut input = parser::prologue(input, "element");

    match element_kind_parser.parse_next(&mut input)? {
        ElementKind::Header => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::RefTypeObject => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::UntypedRuntimeObject => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::UntypedExternalObject => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::RuntimeObject => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::ExternalObject => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::String => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::GenericArray => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::BoxedPrimitiveTypeValue => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::ObjectReference => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::NullValue => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::End => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::Assembly => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::ArrayFiller8b => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::ArrayFiller32b => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::ArrayOfPrimitiveType => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::ArrayOfObject => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::ArrayOfString => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::Method => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::Unknown4 => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::Unknown5 => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::MethodCall => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
        ElementKind::MethodResponse => {
            unimplemented!("this element kind was considered to be non-existent in saves")
        }
    };
}

fn element_kind_parser(input: &mut Stream) -> winnow::Result<ElementKind> {
    let mut input = parser::prologue(input, "kind");

    let kind = parser::explain(
        &mut input,
        move |input| {
            u8.parse_next(input)?
                .try_into()
                .map_err(|error| ContextError::from_external_error(input, error))
        },
        "Element kind. Every element is leaded with this tag.",
    )?;

    Ok(kind)
}
