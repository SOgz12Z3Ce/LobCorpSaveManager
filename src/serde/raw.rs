use crate::serde::raw::element::Element;

mod element;

#[derive(Debug)]
pub(crate) struct File {
    elements: Vec<Element>,
}

impl File {
    pub(crate) fn new(elements: Vec<Element>) -> Self {
        Self { elements }
    }
}
