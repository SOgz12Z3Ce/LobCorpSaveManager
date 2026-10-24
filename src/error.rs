use thiserror::Error;
use winnow::error::{ContextError, ParseError};

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("method has not been implemented")]
    #[allow(dead_code)]
    NotImplemented,

    #[error("parse failed at '{offset}': '{message}'")]
    Parse {
        message: ContextError,
        offset: usize,
    },

    #[error("I/O failed: '{0}'")]
    IO(#[from] std::io::Error),
}

impl<I> From<ParseError<I, ContextError>> for Error {
    fn from(value: ParseError<I, ContextError>) -> Self {
        Self::Parse {
            message: value.inner().clone(),
            offset: value.offset(),
        }
    }
}
