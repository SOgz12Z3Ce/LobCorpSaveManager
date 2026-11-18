use crate::serde::raw::element::FieldMeta;

#[allow(unused)] // TODO: It will be used someday.
#[derive(Debug)]
pub(crate) struct FieldInfo(Vec<FieldMeta>);

impl FieldInfo {
    pub(crate) fn new(field_metas: Vec<FieldMeta>) -> Self {
        Self(field_metas)
    }
}
