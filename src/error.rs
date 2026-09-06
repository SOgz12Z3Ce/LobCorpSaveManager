use thiserror::Error;
use winnow::error::{ContextError, ParseError};

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("method has not been implemented")]
    #[allow(dead_code)]
    NotImplemented,

    #[error("parse failed at '{offset}'")]
    ParseError { offset: usize },

    #[error("element kind '{0}' is invalid")]
    InvalidElementKind(u8),

    #[error("id '0' is invalid")]
    InvalidID,

    #[error("primitive class '{0}' is invalid")]
    InvalidPrimitiveClass(u8),

    #[error("runtime class '{0}' is invalid")]
    InvalidRuntimeClass(String),

    #[error("field kind '{0}' is invalid")]
    InvalidFieldKind(u8),
}

impl From<ParseError<&[u8], ContextError>> for Error {
    fn from(value: ParseError<&[u8], ContextError>) -> Self {
        println!("{}", value.inner());
        Self::ParseError {
            offset: value.offset(),
        }
    }
}
