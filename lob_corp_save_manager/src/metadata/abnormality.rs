use std::fmt::{Display, Formatter};

pub struct Abnormality {
    id: u32,
    // more fields are on the way...
}

#[derive(Debug)]
pub enum Error {
    BadID(i64),
}

pub trait TryLookup<T> {
    fn try_lookup(id: T) -> Result<&'static Self, Error>;
}

impl TryLookup<u32> for Abnormality {
    fn try_lookup(id: u32) -> Result<&'static Self, Error> {
        match id {
            100000 => Ok(&Abnormality { id: 100000 }),
            100001 => Ok(&Abnormality { id: 100001 }),
            100002 => Ok(&Abnormality { id: 100002 }),
            100003 => Ok(&Abnormality { id: 100003 }),
            100004 => Ok(&Abnormality { id: 100004 }),
            100005 => Ok(&Abnormality { id: 100005 }),
            100006 => Ok(&Abnormality { id: 100006 }),
            100007 => Ok(&Abnormality { id: 100007 }),
            100008 => Ok(&Abnormality { id: 100008 }),
            100009 => Ok(&Abnormality { id: 100009 }),
            100010 => Ok(&Abnormality { id: 100010 }),
            100011 => Ok(&Abnormality { id: 100011 }),
            100012 => Ok(&Abnormality { id: 100012 }),
            100013 => Ok(&Abnormality { id: 100013 }),
            100014 => Ok(&Abnormality { id: 100014 }),
            100015 => Ok(&Abnormality { id: 100015 }),
            100016 => Ok(&Abnormality { id: 100016 }),
            100017 => Ok(&Abnormality { id: 100017 }),
            100018 => Ok(&Abnormality { id: 100018 }),
            100019 => Ok(&Abnormality { id: 100019 }),
            100020 => Ok(&Abnormality { id: 100020 }),
            100021 => Ok(&Abnormality { id: 100021 }),
            100022 => Ok(&Abnormality { id: 100022 }),
            100023 => Ok(&Abnormality { id: 100023 }),
            100024 => Ok(&Abnormality { id: 100024 }),
            100025 => Ok(&Abnormality { id: 100025 }),
            100026 => Ok(&Abnormality { id: 100026 }),
            100027 => Ok(&Abnormality { id: 100027 }),
            100028 => Ok(&Abnormality { id: 100028 }),
            100029 => Ok(&Abnormality { id: 100029 }),
            100030 => Ok(&Abnormality { id: 100030 }),
            100031 => Ok(&Abnormality { id: 100031 }),
            100032 => Ok(&Abnormality { id: 100032 }),
            100033 => Ok(&Abnormality { id: 100033 }),
            100034 => Ok(&Abnormality { id: 100034 }),
            100035 => Ok(&Abnormality { id: 100035 }),
            100036 => Ok(&Abnormality { id: 100036 }),
            100037 => Ok(&Abnormality { id: 100037 }),
            100038 => Ok(&Abnormality { id: 100038 }),
            100039 => Ok(&Abnormality { id: 100039 }),
            100040 => Ok(&Abnormality { id: 100040 }),
            100041 => Ok(&Abnormality { id: 100041 }),
            100042 => Ok(&Abnormality { id: 100042 }),
            100043 => Ok(&Abnormality { id: 100043 }),
            100044 => Ok(&Abnormality { id: 100044 }),
            100045 => Ok(&Abnormality { id: 100045 }),
            100046 => Ok(&Abnormality { id: 100046 }),
            100047 => Ok(&Abnormality { id: 100047 }),
            100048 => Ok(&Abnormality { id: 100048 }),
            100049 => Ok(&Abnormality { id: 100049 }),
            100050 => Ok(&Abnormality { id: 100050 }),
            100051 => Ok(&Abnormality { id: 100051 }),
            100052 => Ok(&Abnormality { id: 100052 }),
            100053 => Ok(&Abnormality { id: 100053 }),
            100054 => Ok(&Abnormality { id: 100054 }),
            100055 => Ok(&Abnormality { id: 100055 }),
            100056 => Ok(&Abnormality { id: 100056 }),
            100057 => Ok(&Abnormality { id: 100057 }),
            100058 => Ok(&Abnormality { id: 100058 }),
            100059 => Ok(&Abnormality { id: 100059 }),
            100060 => Ok(&Abnormality { id: 100060 }),
            100061 => Ok(&Abnormality { id: 100061 }),
            100062 => Ok(&Abnormality { id: 100062 }),
            100063 => Ok(&Abnormality { id: 100063 }),
            100064 => Ok(&Abnormality { id: 100064 }),
            100065 => Ok(&Abnormality { id: 100065 }),
            100101 => Ok(&Abnormality { id: 100101 }),
            100102 => Ok(&Abnormality { id: 100102 }),
            100103 => Ok(&Abnormality { id: 100103 }),
            100104 => Ok(&Abnormality { id: 100104 }),
            100105 => Ok(&Abnormality { id: 100105 }),
            100106 => Ok(&Abnormality { id: 100106 }),
            200001 => Ok(&Abnormality { id: 200001 }),
            200002 => Ok(&Abnormality { id: 200002 }),
            200003 => Ok(&Abnormality { id: 200003 }),
            200004 => Ok(&Abnormality { id: 200004 }),
            200005 => Ok(&Abnormality { id: 200005 }),
            200006 => Ok(&Abnormality { id: 200006 }),
            200007 => Ok(&Abnormality { id: 200007 }),
            200009 => Ok(&Abnormality { id: 200009 }),
            200010 => Ok(&Abnormality { id: 200010 }),
            200013 => Ok(&Abnormality { id: 200013 }),
            200015 => Ok(&Abnormality { id: 200015 }),
            200016 => Ok(&Abnormality { id: 200016 }),
            300001 => Ok(&Abnormality { id: 300001 }),
            300002 => Ok(&Abnormality { id: 300002 }),
            300003 => Ok(&Abnormality { id: 300003 }),
            300004 => Ok(&Abnormality { id: 300004 }),
            300005 => Ok(&Abnormality { id: 300005 }),
            300006 => Ok(&Abnormality { id: 300006 }),
            300007 => Ok(&Abnormality { id: 300007 }),
            300101 => Ok(&Abnormality { id: 300101 }),
            300102 => Ok(&Abnormality { id: 300102 }),
            300103 => Ok(&Abnormality { id: 300103 }),
            300104 => Ok(&Abnormality { id: 300104 }),
            300105 => Ok(&Abnormality { id: 300105 }),
            300106 => Ok(&Abnormality { id: 300106 }),
            300107 => Ok(&Abnormality { id: 300107 }),
            300108 => Ok(&Abnormality { id: 300108 }),
            300109 => Ok(&Abnormality { id: 300109 }),
            300110 => Ok(&Abnormality { id: 300110 }),
            _ => Err(Error::BadID(id.into())),
        }
    }
}

impl TryLookup<i64> for Abnormality {
    fn try_lookup(id: i64) -> Result<&'static Self, Error> {
        let id_u32: u32 = id.try_into().map_err(|_| Error::BadID(id))?;
        Self::try_lookup(id_u32)
    }
}

impl Abnormality {
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn as_simple_str(&self) -> String {
        format!("ID: {}", self.id)
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
