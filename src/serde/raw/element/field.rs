use crate::serde::raw::{field::FieldKind, leb128_str::Leb128String, ty::class::Class};

#[allow(unused)] // TODO: It will be used someday.
#[derive(Debug, Clone)]
pub(crate) struct FieldMeta {
    count: usize,
    names: Vec<Leb128String>,
    kinds: Vec<FieldKind>,
    classes: Vec<Class>,
}

impl FieldMeta {
    pub(crate) fn new(
        count: usize,
        names: Vec<Leb128String>,
        kinds: Vec<FieldKind>,
        classes: Vec<Class>,
    ) -> Self {
        Self {
            count,
            names,
            kinds,
            classes,
        }
    }
}
