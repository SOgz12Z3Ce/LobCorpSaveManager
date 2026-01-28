use std::fmt::{Display, Formatter};

use crate::i18n::I18nText;

pub(crate) struct Abnormality {
    id: u32,
    code: &'static I18nText,
}

#[derive(Debug)]
pub(crate) enum Error {
    BadID(i64),
}

pub(crate) trait TryLookup<T> {
    fn try_lookup(id: T) -> Result<&'static Self, Error>;
}

macro_rules! abnormality {
    ($id:literal) => {{
        static ABNORMALITY: Abnormality = Abnormality {
            id: $id,
            code: I18nText::lookup_code($id),
        };
        &ABNORMALITY
    }};
}

macro_rules! try_lookup_abnormality {
    ($val:ident, [$($id:literal),+]) => {
        match $val {
            $($id => Ok(abnormality!($id)),)+
            _ => Err(Error::BadID($val.into()))
        }
    };
}

impl TryLookup<u32> for Abnormality {
    fn try_lookup(id: u32) -> Result<&'static Self, Error> {
        try_lookup_abnormality!(
            id,
            [
                100000, 100001, 100002, 100003, 100004, 100005, 100006, 100007, 100008, 100009,
                100010, 100011, 100012, 100013, 100014, 100015, 100016, 100017, 100018, 100019,
                100020, 100021, 100022, 100023, 100024, 100025, 100026, 100027, 100028, 100029,
                100030, 100031, 100032, 100033, 100034, 100035, 100036, 100037, 100039, 100040,
                100041, 100042, 100043, 100044, 100045, 100046, 100047, 100048, 100049, 100050,
                100051, 100052, 100053, 100054, 100055, 100056, 100057, 100058, 100059, 100060,
                100061, 100062, 100063, 100064, 100065, 100101, 100102, 100103, 100104, 100105,
                100106, 200001, 200002, 200003, 200004, 200005, 200006, 200007, 200009, 200010,
                200013, 200015, 200016, 200018, 200021, 200022, 200023, 200024, 200025, 300001,
                300002, 300003, 300004, 300005, 300006, 300007, 300101, 300102, 300103, 300104,
                300105, 300106, 300107, 300108, 300109, 300110, 1000350, 1000351, 1000352, 1000353,
                2000121, 2000122, 2000123, 2000124
            ]
        )
    }
}

impl TryLookup<i64> for Abnormality {
    fn try_lookup(id: i64) -> Result<&'static Self, Error> {
        let id_u32: u32 = id.try_into().map_err(|_| Error::BadID(id))?;
        Self::try_lookup(id_u32)
    }
}

impl Abnormality {
    pub(crate) fn id(&self) -> u32 {
        self.id
    }
    pub(crate) fn as_simple_str(&self) -> String {
        format!("{}(ID: {})",self.code.en(), self.id)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadID(id) => write!(f, "{} is not a vaild ID.", id),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
