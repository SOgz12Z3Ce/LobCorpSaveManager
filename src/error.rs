use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("method has not been implemented")]
    #[allow(dead_code)]
    NotImplemented,
    #[error("parsing failed ('{0}')")]
    ParseError(String),
}

impl From<winnow::error::ParseError<&[u8], winnow::error::ContextError>> for Error {
    fn from(value: winnow::error::ParseError<&[u8], winnow::error::ContextError>) -> Self {
        let message = value.inner().to_string();
        Self::ParseError(message)
    }
}
