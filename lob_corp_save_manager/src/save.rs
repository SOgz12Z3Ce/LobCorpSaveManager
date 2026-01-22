use std::{fmt::Display, fs::File, io::Read, str::FromStr};

pub mod settings;
trait Save {}

#[derive(Debug)]
pub enum Error {
    ParseJsonFail(String, Box<dyn std::error::Error>),
    ReadFileFail(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseJsonFail(json, err) => write!(f, "{:?} from this JSON: {}", err, json)?,
            Error::ReadFileFail(err) => write!(f, "Cannot read file: {:?}", err)?,
        }
        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ParseJsonFail(_, err) => Some(err.as_ref()),
            Error::ReadFileFail(err) => Some(err),
        }
    }
}

pub trait RawSave: FromStr
where
    <Self as FromStr>::Err: std::error::Error + 'static,
{
    fn from_file(mut file: File) -> Result<Self, Error> {
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(Error::ReadFileFail)?;
        Self::from_str(&content).map_err(|e| Error::ParseJsonFail(content, Box::new(e)))
    }
}
