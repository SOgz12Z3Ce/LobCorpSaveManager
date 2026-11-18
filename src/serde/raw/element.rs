use crate::serde::raw::{
    element::field::FieldMeta,
    id::Id,
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
        field_meta: FieldMeta,
        field_values: Vec<Value>,
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
