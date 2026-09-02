#[derive(Debug)]
pub struct File {
    pub objects: Vec<Element>,
}

#[derive(Debug)]
pub struct Element {
    pub kind: ElementKind,
}

#[derive(Debug)]
pub enum ElementKind {
    RuntimeObject,
}
