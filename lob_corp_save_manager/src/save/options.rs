use core::str;
use std::{
    fmt::{Display, Formatter},
    fs::File,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::save::RawSave;

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

#[derive(Debug)]
struct Toggle(bool);

#[derive(Debug)]
struct Factor(f32);

#[derive(Debug)]
struct IndexCycle(u8);

#[derive(Debug)]
pub struct OptionsSave {
    language: Language,
    tooltips_enabled: Toggle,
    backer_abnormalities_enabled: Toggle,
    master_volume: Factor,
    music_volume: Factor,
    log_index: IndexCycle,
}

#[derive(Debug)]
pub enum Error {
    BadJsonFile(super::Error),
    BadJsonString(serde_json::Error),
    InvalidFactorValue(f32),
    InvalidIndexCycleValue(i32),
    InvalidLanguage(String),
    UnsupportedLanguage(String),
}

impl OptionsSave {
    pub fn from_file(file: File) -> Result<Self, Error> {
        let raw = OptionsRawSave::from_file(file).map_err(Error::BadJsonFile)?;
        Self::from_raw(raw)
    }
    fn set_master_volume(&mut self, value: f32) -> Result<(), Error> {
        self.master_volume = value.try_into()?;
        Ok(())
    }
    fn set_music_volume(&mut self, value: f32) -> Result<(), Error> {
        self.music_volume = value.try_into()?;
        Ok(())
    }
    fn set_log_index(&mut self, value: i32) -> Result<(), Error> {
        self.log_index = value.try_into()?;
        Ok(())
    }
    fn from_raw(raw: OptionsRawSave) -> Result<Self, Error> {
        Ok(Self {
            language: Language::from_str(&raw.language)?,
            tooltips_enabled: raw.tooltips_enabled.into(),
            backer_abnormalities_enabled: raw.backer_abnormalities_enabled.into(),
            master_volume: raw.master_volume.try_into()?,
            music_volume: raw.music_volume.try_into()?,
            log_index: raw.log_index.try_into()?,
        })
    }
}

impl Display for OptionsSave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "==== Options ====")?;
        writeln!(f, "Game:")?;
        writeln!(f, "  Language: {}", self.language)?;
        writeln!(f, "  Display Tooltips: {}", self.tooltips_enabled.as_word())?;
        writeln!(
            f,
            "  Enable Backer Abnormalities: {}",
            self.backer_abnormalities_enabled.as_word()
        )?;

        writeln!(f, "Audio:")?;
        writeln!(f, "  Master Volume: {}%", self.master_volume.0 * 100.0)?;
        writeln!(f, "  Music Volume: {}%", self.music_volume.0 * 100.0)?;

        writeln!(f, "Debug:")?;
        write!(f, "  Log Index: {}", self.log_index.0)?;
        Ok(())
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::English => "English",
                Language::Korean => "한국어",
                Language::ChineseSimplified => "中文(简体)",
                Language::ChineseTraditional => "中文(繁體)",
                Language::Japanese => "日本語",
                Language::Russian => "русский",
                Language::Bulgarian => "български",
                Language::SpanishLatinAmerica => "Español Latinoamérica",
                Language::French => "français",
                Language::PortugueseBrazil => "Português do Brasil",
                Language::PortuguesePortugal => "Português",
            }
        )
    }
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
            "vn" => Err(Error::UnsupportedLanguage("vn".to_owned())),
            "bg" => Ok(Language::Bulgarian),
            "es" => Ok(Language::SpanishLatinAmerica),
            "fr" => Ok(Language::French),
            "pt_br" => Ok(Language::PortugueseBrazil),
            "pt_pt" => Ok(Language::PortuguesePortugal),
            _ => Err(Error::InvalidLanguage(s.to_owned())),
        }
    }
}

impl Toggle {
    fn as_word(&self) -> &'static str {
        match self {
            Self(true) => "on",
            Self(false) => "off",
        }
    }
}

impl From<bool> for Toggle {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl TryFrom<f32> for Factor {
    type Error = Error;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value >= 0.0 && value <= 1.0 {
            Ok(Self(value))
        } else {
            Err(Error::InvalidFactorValue(value))
        }
    }
}

impl TryFrom<u8> for IndexCycle {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 10 {
            Ok(Self(value))
        } else {
            Err(Error::InvalidIndexCycleValue(value.into()))
        }
    }
}

impl TryFrom<i32> for IndexCycle {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let value_u8: u8 = value
            .try_into()
            .map_err(|_| Error::InvalidIndexCycleValue(value))?;
        Self::try_from(value_u8)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadJsonFile(e) => write!(f, "Cannot deserialize JSON file: {:?}", e)?,
            Error::BadJsonString(e) => write!(f, "Cannot deserialize JSON string: {:?}", e)?,
            Error::InvalidFactorValue(v) => write!(f, "{} is not in [0, 1].", v)?,
            Error::InvalidIndexCycleValue(v) => write!(f, "{} is not in [0, 10).", v)?,
            Error::InvalidLanguage(s) => write!(f, "{} is not a vaild language.", s)?,
            Error::UnsupportedLanguage(s) => write!(f, "{} is not suppored in regular game.", s)?,
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

#[derive(Serialize, Deserialize, Debug)]
struct OptionsRawSave {
    #[serde(rename = "masterVolume")]
    master_volume: f32,

    #[serde(rename = "bgmVolume")]
    music_volume: f32,

    #[serde(rename = "tooltip")]
    tooltips_enabled: bool,

    #[serde(rename = "dlcCreatureOn")]
    backer_abnormalities_enabled: bool,

    #[serde(rename = "language")]
    language: String,

    #[serde(rename = "logIndex")]
    log_index: i32,
}

impl FromStr for OptionsRawSave {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|e| Error::BadJsonString(e))
    }
}

impl RawSave for OptionsRawSave {}
