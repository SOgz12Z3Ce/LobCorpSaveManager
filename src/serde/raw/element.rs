use crate::serde::raw::{
    field::FieldKind,
    id::Id,
    leb128_str::Leb128String,
    ty::{class::Class, value::Value},
};

pub(crate) mod field;
pub(crate) mod kind;

#[allow(unused)] // TODO: It will be used someday.
#[derive(Debug)]
pub(crate) enum Element {
    // Header,
    // RefTypeObject,
    // UntypedRuntimeObject,
    // UntypedExternalObject,
    RuntimeObject {
        id: Id,
        class: Class,
        fields: Vec<Field>,
    },
    // ExternalObject,
    // String,
    // GenericArray,
    // BoxedPrimitiveTypeValue,
    // ObjectReference,
    // NullValue,
    End,
    // Assembly,
    // ArrayFiller8b,
    // ArrayFiller32b,
    // ArrayOfPrimitiveType,
    // ArrayOfObject,
    // ArrayOfString,
    // Method,
    // Unknown4,
    // Unknown5,
    // MethodCall,
    // MethodResponse,
}

#[derive(Debug)]
pub(crate) struct FieldMeta {
    name: Leb128String,
    kind: FieldKind,
    class: Class,
}

impl FieldMeta {
    pub(crate) fn new(name: Leb128String, kind: FieldKind, class: Class) -> Self {
        Self { name, kind, class }
    }
}

#[derive(Debug)]
pub(crate) struct Field {
    meta: FieldMeta,
    value: Value,
}

impl Field {
    pub(crate) fn new(meta: FieldMeta, value: Value) -> Self {
        Self { meta, value }
    }
}
