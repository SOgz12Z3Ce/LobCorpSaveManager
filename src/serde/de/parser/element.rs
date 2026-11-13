use crate::serde::{
    de::{Stream, parser},
    raw::element::Element,
};

pub(crate) fn element_parser(input: &mut Stream) -> winnow::Result<Element> {
    parser::prologue(input, "element");
    todo!()
}
