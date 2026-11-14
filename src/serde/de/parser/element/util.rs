use winnow::{
    Parser,
    binary::le_u32,
    error::{ContextError, FromExternalError},
};

use crate::serde::{
    de::{Stream, parser},
    raw::element::id::Id,
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
