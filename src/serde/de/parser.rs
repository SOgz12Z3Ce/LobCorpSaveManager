mod element;
mod top;

use std::{mem::DropGuard, ops::DerefMut};

use winnow::Parser;

#[cfg(feature = "log")]
use crate::serde::de::log::Log;
use crate::serde::{
    de::{
        Stream,
        parser::top::{elements_parser, magic_parser},
    },
    raw::File,
};

fn prologue<'a, 'b>(
    input: &'a mut Stream<'b>,
    stage: &'static str,
) -> impl DerefMut<Target = &'a mut Stream<'b>> {
    #[cfg(feature = "log")]
    input.state.log_engine.begin(stage);
    DropGuard::new(input, move |input| {
        #[cfg(feature = "log")]
        input.state.log_engine.end();
    })
}

#[cfg(not(feature = "log"))]
fn explain<'b, F, O>(
    input: &mut Stream<'b>,
    process: F,
    _explaination: &'static str,
) -> winnow::Result<O>
where
    F: FnOnce(&mut Stream<'b>) -> winnow::Result<O>,
{
    process(input)
}

#[cfg(feature = "log")]
fn explain<'b, F, O>(
    input: &mut Stream<'b>,
    process: F,
    explaination: &'static str,
) -> winnow::Result<O>
where
    F: FnOnce(&mut Stream<'b>) -> winnow::Result<O>,
    O: Log,
{
    let value = process(input)?;
    input.state.log_engine.log(&value, explaination);
    Ok(value)
}

pub(crate) fn parser(input: &mut Stream) -> winnow::Result<File> {
    let mut input = prologue(input, "main");

    magic_parser.parse_next(&mut input)?;
    let elements = elements_parser.parse_next(&mut input)?;

    Ok(File::new(elements))
}
