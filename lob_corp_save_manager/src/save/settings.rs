use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    BadJsonString(serde_json::Error),
    InvalidFactorValue(f32),
    InvalidIndexCycleValue(i32),
    InvalidLanguage(String),
    NotSupportedLanguage(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadJsonString(e) => write!(f, "Cannot deserialize JSON string: {:?}", e)?,
            Error::InvalidFactorValue(v) => write!(f, "{} is not in [0, 1].", v)?,
            Error::InvalidIndexCycleValue(v) => write!(f, "{} is not in [0, 10).", v)?,
            Error::InvalidLanguage(s) => write!(f, "{} is not a vaild language.", s)?,
            Error::NotSupportedLanguage(s) => {
                write!(f, "{} dosen't be suppored in regular game.", s)?
            }
        }
        Ok(())
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::BadJsonString(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct SettingsSave {
    master_volume: Factor,
    bgm_volume: Factor,
    tooltip_enabled: bool,
    patron_abnormailties_enabled: bool,
    language: Language,
    log_index: IndexCycle,
}

impl SettingsSave {
    pub fn from_raw(raw: SettingsRawSave) -> Result<Self, Error> {
        Ok(Self {
            master_volume: Factor::new(raw.master_volume)?,
            bgm_volume: Factor::new(raw.bgm_volume)?,
            tooltip_enabled: raw.tooltip_enabled,
            patron_abnormailties_enabled: raw.patron_abnormailties_enabled,
            language: Language::from_str(&raw.language)?,
            log_index: IndexCycle::new_i32(raw.log_index)?,
        })
    }
}

#[derive(Debug)]
pub struct Factor(f32);

impl Factor {
    fn new(f: f32) -> Result<Self, Error> {
        if f >= 0f32 && f <= 1f32 {
            Ok(Factor(f))
        } else {
            Err(Error::InvalidFactorValue(f))
        }
    }
}

#[derive(Debug)]
pub struct IndexCycle(u8);

impl IndexCycle {
    fn new(i: u8) -> Result<Self, Error> {
        if i < 10 {
            Ok(IndexCycle(i))
        } else {
            Err(Error::InvalidIndexCycleValue(i as i32))
        }
    }

    fn new_i32(i: i32) -> Result<Self, Error> {
        match i.try_into().ok() {
            Some(val) => IndexCycle::new(val),
            None => Err(Error::InvalidIndexCycleValue(i as i32)),
        }
    }
}

#[derive(Debug)]
enum Language {
    English,
    Korean,
    ChineseSimplified,
    ChineseTraditional,
    Japanese,
    Russian,
    Bulgarian,
    SpanishLatinAmerica,
    French,
    PortugueseBrazil,
    PortuguesePortugal,
}

impl FromStr for Language {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(Language::English),
            "kr" => Ok(Language::Korean),
            "cn" => Ok(Language::ChineseSimplified),
            "cn_tr" => Ok(Language::ChineseTraditional),
            "jp" => Ok(Language::Japanese),
            "ru" => Ok(Language::Russian),
            "vn" => Err(Error::NotSupportedLanguage("vn".to_owned())),
            "bg" => Ok(Language::Bulgarian),
            "es" => Ok(Language::SpanishLatinAmerica),
            "fr" => Ok(Language::French),
            "pt_br" => Ok(Language::PortugueseBrazil),
            "pt_pt" => Ok(Language::PortuguesePortugal),
            _ => Err(Error::InvalidLanguage(s.to_owned())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsRawSave {
    #[serde(rename = "masterVolume")]
    master_volume: f32,

    #[serde(rename = "bgmVolume")]
    bgm_volume: f32,

    #[serde(rename = "tooltip")]
    tooltip_enabled: bool,

    #[serde(rename = "dlcCreatureOn")]
    patron_abnormailties_enabled: bool,

    #[serde(rename = "language")]
    language: String,

    #[serde(rename = "logIndex")]
    log_index: i32,
}

impl FromStr for SettingsRawSave {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str(s) {
            Ok(save) => Ok(save),
            Err(e) => Err(Error::BadJsonString(e)),
        }
    }
}
