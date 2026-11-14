use winnow::{
    Parser,
    binary::{le_u32, u8},
    error::{ContextError, FromExternalError, ParserError},
    token::take,
};

use crate::serde::{
    de::{Stream, parser},
    raw::element::{class::Class, id::Id},
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

pub(crate) fn runtime_class_parser(input: &mut Stream) -> winnow::Result<Class> {
    let mut input = parser::prologue(input, "class");

    parser::explain(
        &mut input,
        move |input| {
            let name = leb128_string_parser.parse_next(input)?;
            Class::runtime(name).map_err(|error| ContextError::from_external_error(input, error))
        },
        "Runtime object class. LEB128 encoded C# type full name.",
    )
}

fn leb128_string_parser<'b>(input: &mut Stream<'b>) -> winnow::Result<&'b str> {
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

    let capacity = usize::try_from(length).expect("length has been verified to be > 0 above");
    let content = take(capacity).parse_next(input)?;
    str::from_utf8(content).map_err(|e| ContextError::from_external_error(input, e))
}
