use winnow::{
    Parser,
    binary::le_u32,
    error::{ContextError, FromExternalError},
};

use crate::serde::{de::Stream, raw::Id};

pub fn id_parser(input: &mut Stream) -> winnow::Result<Id> {
    le_u32
        .parse_next(input)?
        .try_into()
        .map_err(|error| ContextError::from_external_error(input, error))
}
