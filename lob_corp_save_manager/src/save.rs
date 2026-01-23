use std::{fmt::Display, fs::File};

use serde::{Serialize, de::DeserializeOwned};

pub mod options;

#[derive(Debug)]
pub enum Error {
    BadJsonFile(serde_json::Error),
    BadRawSave(Box<dyn std::error::Error>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadJsonFile(e) => write!(f, "Cannot deserialize JSON file: {:?}", e)?,
            Self::BadRawSave(e) => write!(f, "Invaild Save: {:?}", e)?,
        }
        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BadJsonFile(e) => Some(e),
            Self::BadRawSave(e) => Some(e.as_ref()),
        }
    }
}

pub trait Save<R>: TryFrom<R>
where
    R: RawSave<Self>,
    <Self as TryFrom<R>>::Error: 'static + std::error::Error,
{
    fn from_file(file: File) -> Result<Self, Error> {
        let raw: R = serde_json::from_reader(file).map_err(Error::BadJsonFile)?;
        raw.try_into().map_err(|e| Error::BadRawSave(Box::new(e)))
    }
    fn into_json(self) -> Result<String, serde_json::Error> {
        let raw: R = self.into();
        serde_json::to_string(&raw)
    }
}

pub trait RawSave<S>: From<S> + Serialize + DeserializeOwned
where
    S: TryFrom<Self>,
{
}
