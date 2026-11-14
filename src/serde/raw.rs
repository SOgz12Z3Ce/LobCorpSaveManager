pub(crate) mod element;

use std::fmt::Display;

use crate::serde::raw::element::Element;

#[derive(Debug)]
pub(crate) struct File {
    elements: Vec<Element>,
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = &self.elements;
        write!(f, "{self:?}") // TODO: Use a better way to display.
    }
}

impl File {
    pub(crate) fn new(elements: Vec<Element>) -> Self {
        Self { elements }
    }
}
