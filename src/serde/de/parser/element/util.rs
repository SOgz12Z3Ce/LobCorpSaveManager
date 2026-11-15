use winnow::{
    Parser,
    binary::{le_i32, le_u32, u8},
    error::{ContextError, FromExternalError, ParserError},
    token::take,
};

use crate::serde::{
    de::{Stream, parser},
    raw::{class::Class, count::Count, field::FieldKind, id::Id, leb128_str::Leb128Str},
};

pub(crate) fn id_parser(input: &mut Stream) -> winnow::Result<Id> {
    let mut input = parser::prologue(input, "id");

    parser::explain(
        &mut input,
        move |input| {
            le_u32
                .parse_next(input)?
                .try_into()
                .map_err(|error| ContextError::from_external_error(input, error))
        },
        "Object ID. ID is starting from 1 and consecutive. In C#, ID is processed as i64, written as i32 and read as u32. So ID should be in [1, 2147483647].",
    )
}

pub(crate) fn field_kind_parser(input: &mut Stream) -> winnow::Result<FieldKind> {
    let mut input = parser::prologue(input, "field_kind");

    parser::explain(
        &mut input,
        move |input| {
            u8.parse_next(input)?
                .try_into()
                .map_err(|error| ContextError::from_external_error(input, error))
        },
        "Runtime object field kind.",
    )
}

pub(crate) fn class_parser_of<'b>(kind: FieldKind) -> impl Parser<Stream<'b>, Class, ContextError> {
    match kind {
        FieldKind::PrimitiveType => field_primitive_class_parser,
        FieldKind::String => {
            unimplemented!("this field kind was considered to be non-existent in saves")
        }
        FieldKind::ObjectType => {
            unimplemented!("this field kind was considered to be non-existent in saves")
        }
        FieldKind::RuntimeType => field_runtime_class_parser,
        FieldKind::GenericType => {
            unimplemented!("this field kind was considered to be non-existent in saves")
        }
        FieldKind::ArrayOfObject => {
            unimplemented!("this field kind was considered to be non-existent in saves")
        }
        FieldKind::ArrayOfString => {
            unimplemented!("this field kind was considered to be non-existent in saves")
        }
        FieldKind::ArrayOfPrimitiveType => {
            unimplemented!("this field kind was considered to be non-existent in saves")
        }
    }
}

macro_rules! wrapper {
    ($name:ident: $parser:ident -> $result:ty) => {
        macro_rules! $name {
            ($func:ident($stage:literal: $explaination:literal)) => {
                pub(crate) fn $func<'a, 'b>(input: &'a mut Stream<'b>) -> winnow::Result<$result> {
                    let mut input = parser::prologue(input, $stage);

                    parser::explain(
                        &mut input,
                        move |input| $parser.parse_next(input),
                        $explaination,
                    )
                }
            };
        }
    };
}

wrapper!(primitive_class_parser: primitive_class_parser -> Class);
wrapper!(runtime_class_wrapper: runtime_class_parser -> Class);
wrapper!(count_wrapper: count_parser -> Count);
wrapper!(leb128_str_wrapper: leb128_str_parser -> Leb128Str<'b>);

primitive_class_parser!(field_primitive_class_parser("field_class": "Runtime object field class. This is a primitive class."));
runtime_class_wrapper!(runtime_object_class_parser("class": "Runtime object class. LEB128 encoded C# type full name."));
runtime_class_wrapper!(field_runtime_class_parser("class": "Runtime object field class. This is a runtime class. LEB128 encoded C# type full name."));
count_wrapper!(field_count_parser("field_count": "Runtime object field count."));
leb128_str_wrapper!(field_name_parser("field_name": "Runtime object field name. LEB128 encoded string."));

fn primitive_class_parser(input: &mut Stream) -> winnow::Result<Class> {
    let code = u8.parse_next(input)?;
    Class::primitive(code).map_err(|error| ContextError::from_external_error(input, error))
}

fn runtime_class_parser(input: &mut Stream) -> winnow::Result<Class> {
    let name = leb128_str_parser.parse_next(input)?;
    Class::runtime(name).map_err(|error| ContextError::from_external_error(input, error))
}

fn count_parser(input: &mut Stream) -> winnow::Result<Count> {
    let count = le_i32.parse_next(input)?;
    if count < 0 {
        Err(ContextError::from_input(input))
    } else {
        Count::try_from(count).map_err(|error| ContextError::from_external_error(input, error))
    }
}

fn leb128_str_parser<'b>(input: &mut Stream<'b>) -> winnow::Result<Leb128Str<'b>> {
    let length = {
        let mut buffer = 0u64;
        let mut shift = 0usize;

        let mut counter = 0usize;
        loop {
            let byte = u8.parse_next(input)?;
            let end = byte & 0b10000000 == 0;
            let section = byte & 0b01111111;
            buffer |= (section as u64) << shift;
            if end {
                break;
            }
            counter += 1;

            if counter >= 5 {
                // sic (@06001598). Somehow, C# allows reading 5 bytes but
                // store them in a i32 buffer. The buffer should be larger than
                // 5 * 7 = 35 bits and unsigned.
                //
                // This is a little worrying. We choose not to process this.
                // ([...], not because they are easy, but because they are
                // hard; [...])
                return Err(ContextError::from_input(input));
            }
            shift += 7;
        }
        buffer
    };
    if length <= 0 {
        return Err(ContextError::from_input(input));
    }

    let content = take(length as usize).parse_next(input)?;
    let content = str::from_utf8(content);
    match content {
        Ok(content) => Ok(content.into()),
        Err(error) => Err(ContextError::from_external_error(input, error)),
    }
}
