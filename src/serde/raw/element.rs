pub(crate) mod id;
pub(crate) mod kind;

#[derive(Debug)]
pub(crate) enum Element {
    // Header,
    // RefTypeObject,
    // UntypedRuntimeObject,
    // UntypedExternalObject,
    // RuntimeObject,
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
