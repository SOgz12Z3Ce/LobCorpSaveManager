mod util;

use winnow::{Parser, combinator::repeat};

use crate::serde::{
    de::{
        Stream,
        parser::{
            self,
            element::util::{
                class_parser_of, field_count_parser, field_kind_parser, field_name_parser,
                id_parser, leading_element_kind_parser, runtime_object_class_parser,
                value_parser_of,
            },
        },
    },
    raw::{
        element::{Element, kind::ElementKind},
        field::FieldKind,
        leb128_str::Leb128Str,
        ty::{class::Class, value::Value},
    },
};

pub(crate) fn element_parser(input: &mut Stream) -> winnow::Result<Element> {
    match leading_element_kind_parser.parse_next(input)? {
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
        ElementKind::RuntimeObject => runtime_object_parser.parse_next(input),
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
        ElementKind::End => Ok(Element::End),
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
    }
}

fn runtime_object_parser(input: &mut Stream) -> winnow::Result<Element> {
    let mut input = parser::prologue(input, "runtime_object");

    let id = id_parser.parse_next(&mut input)?;
    let class = runtime_object_class_parser.parse_next(&mut input)?;
    let field_count = usize::from(field_count_parser.parse_next(&mut input)?);
    let field_names: Vec<Leb128Str> =
        repeat(field_count, field_name_parser).parse_next(&mut input)?;
    let field_kinds: Vec<FieldKind> =
        repeat(field_count, field_kind_parser).parse_next(&mut input)?;
    let field_classes: Vec<Class> = field_kinds
        .iter()
        .map(|kind| class_parser_of(*kind).parse_next(&mut input))
        .collect::<Result<_, _>>()?;
    let field_value: Vec<Value> = field_classes
        .iter()
        .map(|class| value_parser_of(*class).parse_next(&mut input))
        .collect::<Result<_, _>>()?;

    todo!()
}
