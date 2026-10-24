use crate::serde::{de::Stream, raw::File};

pub(crate) fn parser(input: &mut Stream) -> winnow::Result<File> {
    let elements = Vec::new();
    Ok(File::new(elements))
}
