use crate::raw::{FieldMeta, Id};
use std::collections::HashMap;
use winnow::Stateful;

#[derive(Debug, Clone)]
pub struct State {
    pub metadata: HashMap<Id, FieldMeta>,
}

pub type Stream<'a> = Stateful<&'a [u8], State>;
