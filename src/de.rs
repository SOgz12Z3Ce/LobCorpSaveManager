use crate::{
    error::Error,
    raw::{Class, Element, ElementKind, FieldKind, FieldValue, File, Id},
};
use std::str::FromStr;
use winnow::{
    Parser,
    binary::{le_f32, le_i32, le_u32, u8},
    combinator::repeat,
    error::{ContextError, FromExternalError, ParserError},
    token::{literal, rest, take},
};

pub fn parse(input: Vec<u8>) -> Result<File, Error> {
    let (file, _) = (parser, rest).parse(&input)?;

    Ok(file)
}

pub fn parser(input: &mut &[u8]) -> winnow::Result<File> {
    magic_parser.parse_next(input)?;
    let root = element_parser.parse_next(input)?;
    let obj2 = element_parser.parse_next(input)?;
    let obj3 = element_parser.parse_next(input)?;

    Ok(File {
        elements: vec![root, obj2, obj3],
    })
}

fn magic_parser(input: &mut &[u8]) -> winnow::Result<()> {
    literal(0x00).parse_next(input)?;
    le_i32.verify(|i| *i == 1).parse_next(input)?;
    le_i32.verify(|i| *i == -1).parse_next(input)?;
    le_i32.verify(|i| *i == 1).parse_next(input)?;
    le_i32.verify(|i| *i == 0).parse_next(input)?;

    Ok(())
}

fn element_parser(input: &mut &[u8]) -> winnow::Result<Element> {
    let kind = element_kind_parser.parse_next(input)?;
    match kind {
        ElementKind::RefTypeObject => {
            let id = id_parser.parse_next(input)?;
            let ref_id = id_parser.parse_next(input)?;

            Ok(Element::RefTypeObject { id, ref_id })
        }
        ElementKind::RuntimeObject => {
            let id = id_parser.parse_next(input)?;
            let class = runtime_class_parser.parse_next(input)?;
            let field_count = count_parser.parse_next(input)?;
            let field_names: Vec<String> =
                repeat(field_count, leb128_string_parser).parse_next(input)?;
            let field_kinds: Vec<FieldKind> =
                repeat(field_count, field_kind_parser).parse_next(input)?;
            let field_classes: Vec<Class> = field_kinds
                .iter()
                .map(|k| class_for_field_kind(*k).parse_next(input))
                .collect::<Result<_, _>>()?;
            let field_values: Vec<FieldValue> = field_classes
                .iter()
                .map(|c| field_value_for_class(*c).parse_next(input))
                .collect::<Result<_, _>>()?;

            Ok(Element::RuntimeObject {
                id,
                class,
                field_count,
                field_names,
                field_kinds,
                field_classes,
                field_values,
            })
        }
        ElementKind::String => {
            let id = id_parser.parse_next(input)?;
            let value = leb128_string_parser.parse_next(input)?;

            Ok(Element::String { id, value })
        }
        ElementKind::GenericArray => {
            let id = id_parser.parse_next(input)?;
            literal(0x00).parse_next(input)?;
            // If type.IsArry, then 0x01, else:
            // If array.Rank == 1, then 0x00 else:
            // 0x02.
            // As far as we can see, we should have 0x00 here.
            let rank = count_parser.verify(|i| *i == 1).parse_next(input)?; // We don't need complex arraies.
            let length = count_parser.parse_next(input)?;
            let kind = field_kind_parser.parse_next(input)?;
            let class = class_for_field_kind(kind).parse_next(input)?;
            let values: Vec<FieldValue> =
                repeat(length, field_value_for_class(class)).parse_next(input)?;

            Ok(Element::GenericArray {
                id,
                rank,
                length,
                kind,
                class,
                values,
            })
        }
        ElementKind::BoxedPrimitiveTypeValue => {
            let class = primitive_class_parser.parse_next(input)?;
            let value = field_value_for_class(class).parse_next(input)?;

            Ok(Element::BoxedPrimitiveTypeValue {
                class,
                value: Box::new(value),
            })
        }
        ElementKind::ObjectReference => {
            let id = id_parser.parse_next(input)?;

            Ok(Element::ObjectReference { id })
        }
    }
}

fn element_kind_parser(input: &mut &[u8]) -> winnow::Result<ElementKind> {
    u8.parse_next(input)?
        .try_into()
        .map_err(|e| ContextError::from_external_error(input, e))
}

fn id_parser(input: &mut &[u8]) -> winnow::Result<Id> {
    // Store in file as u32. Process in C# as i64.
    le_u32
        .parse_next(input)?
        .try_into()
        .map_err(|e| ContextError::from_external_error(input, e))
}

fn runtime_class_parser(input: &mut &[u8]) -> winnow::Result<Class> {
    let class = leb128_string_parser.parse_next(input)?;
    let class = Class::from_str(&class).map_err(|e| ContextError::from_external_error(input, e))?;

    Ok(class)
}

fn leb128_string_parser(input: &mut &[u8]) -> winnow::Result<String> {
    let length = {
        let mut buffer = 0i32; // sic
        let mut shift = 0usize;

        let mut counter = 0usize;
        loop {
            let byte = u8.parse_next(input)?;
            let end = byte & 0b10000000 == 0;
            let section = byte & 0b01111111;
            buffer |= i32::from(section) << shift;
            if end {
                break;
            }
            counter += 1;
            if counter > 5 {
                // Only read 5 or less bytes. The 5th byte will be truncated.
                return Err(ContextError::from_input(input));
            }
            shift += 7;
        }
        buffer
    };
    if length <= 0 {
        return Err(ContextError::from_input(input));
    }
    let capacity = usize::try_from(length).expect("length has been verified to be > 0 above");
    let content = take(capacity).parse_next(input)?;
    let string =
        str::from_utf8(content).map_err(|e| ContextError::from_external_error(input, e))?;

    Ok(string.to_owned())
}

fn count_parser(input: &mut &[u8]) -> winnow::Result<usize> {
    // Store in file as i32. Process in C# as i32.
    let count = le_i32.parse_next(input)?;
    if count < 0 {
        Err(ContextError::from_input(input))
    } else {
        Ok(count
            .try_into()
            .expect("count has been verified to be >= 0 and is less than i32 max"))
    }
}

fn field_kind_parser(input: &mut &[u8]) -> winnow::Result<FieldKind> {
    u8.parse_next(input)?
        .try_into()
        .map_err(|e| ContextError::from_external_error(input, e))
}

fn class_for_field_kind<'a>(kind: FieldKind) -> impl Parser<&'a [u8], Class, ContextError> {
    match kind {
        FieldKind::PrimitiveType => primitive_class_parser,
        FieldKind::String => string_class_parser,
        FieldKind::ObjectType => object_class_parser,
        FieldKind::RuntimeType => runtime_class_parser,
    }
}

fn primitive_class_parser(input: &mut &[u8]) -> winnow::Result<Class> {
    u8.parse_next(input)?
        .try_into()
        .map_err(|e| ContextError::from_external_error(input, e))
}

fn string_class_parser(_input: &mut &[u8]) -> winnow::Result<Class> {
    Ok(Class::String)
}

fn object_class_parser(_input: &mut &[u8]) -> winnow::Result<Class> {
    Ok(Class::Object)
}

fn field_value_for_class<'a>(class: Class) -> impl Parser<&'a [u8], FieldValue, ContextError> {
    match class {
        Class::Int => int_field_value_parser,
        Class::Float => float_field_value_parser,
        Class::String => element_field_value_parser,
        Class::Object => element_field_value_parser,
        Class::Dictinoary => element_field_value_parser,
        Class::GenericEqualityComparer => element_field_value_parser,
        Class::KeyValuePairArray => element_field_value_parser,
        Class::KeyValuePair => element_field_value_parser,
    }
}

fn int_field_value_parser(input: &mut &[u8]) -> winnow::Result<FieldValue> {
    let value = le_i32.parse_next(input)?;

    Ok(FieldValue::Int(value))
}

fn float_field_value_parser(input: &mut &[u8]) -> winnow::Result<FieldValue> {
    let value = le_f32.parse_next(input)?;

    Ok(FieldValue::Float(value))
}

fn element_field_value_parser(input: &mut &[u8]) -> winnow::Result<FieldValue> {
    let value = element_parser.parse_next(input)?;

    Ok(FieldValue::Object(value))
}
