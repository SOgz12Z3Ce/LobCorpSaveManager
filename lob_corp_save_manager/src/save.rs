use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, BufWriter},
};

use serde::{Serialize, de::DeserializeOwned};

pub(crate) mod etc;
pub(crate) mod options;

pub(crate) trait Save<R>: TryFrom<R>
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
    fn into_file(self, path: &str) -> Result<(), Error> {
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|e| Error::OpenFileFail(path.to_owned(), e))?;
        let writer = BufWriter::new(file);
        let raw: R = self.into();
        serde_json::to_writer(writer, &raw).map_err(|e| Error::WriteFileFail(path.to_owned(), e))
    }
}

pub(crate) trait RawSave<S>: From<S> + Serialize + DeserializeOwned
where
    S: TryFrom<Self>,
{
}

#[derive(Debug)]
pub(crate) enum Error {
    BadJsonFile(serde_json::Error),
    BadRawSave(Box<dyn std::error::Error>),
    OpenFileFail(String, io::Error),
    WriteFileFail(String, serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadJsonFile(e) => write!(f, "Cannot deserialize JSON file: {:?}", e)?,
            Self::BadRawSave(e) => write!(f, "Invaild Save: {:?}", e)?,
            Self::OpenFileFail(path, e) => write!(f, "Cannot open file \"{}\": {:?}", path, e)?,
            Self::WriteFileFail(path, e) => {
                write!(f, "Cannot write into file \"{}\": {:?}", path, e)?
            }
        }
        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BadJsonFile(e) => Some(e),
            Self::BadRawSave(e) => Some(e.as_ref()),
            Self::OpenFileFail(_, e) => Some(e),
            Self::WriteFileFail(_, e) => Some(e),
        }
    }
}
