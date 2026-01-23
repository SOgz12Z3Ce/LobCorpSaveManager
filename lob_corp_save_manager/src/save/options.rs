use core::str;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::save::{RawSave, Save};

#[derive(Debug)]
pub enum Language {
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
pub struct Toggle(bool);

#[derive(Debug)]
pub struct Factor(f32);

#[derive(Debug)]
pub struct IndexCycle(u8);

#[derive(Debug)]
pub struct OptionsSave {
    pub language: Language,
    pub tooltips_enabled: Toggle,
    pub backer_abnormalities_enabled: Toggle,
    pub master_volume: Factor,
    pub music_volume: Factor,
    pub log_index: IndexCycle,
}

#[derive(Debug)]
pub enum Error {
    InvalidLanguage(String),
    UnsupportedLanguage(String),
    InvalidFactorValue(f32),
    InvalidIndexCycleValue(i32),
}

impl TryFrom<OptionsRawSave> for OptionsSave {
    type Error = Error;
    fn try_from(value: OptionsRawSave) -> Result<Self, Self::Error> {
        Ok(Self {
            language: Language::from_str(&value.language)?,
            tooltips_enabled: value.tooltips_enabled.into(),
            backer_abnormalities_enabled: value.backer_abnormalities_enabled.into(),
            master_volume: value.master_volume.try_into()?,
            music_volume: value.music_volume.try_into()?,
            log_index: value.log_index.try_into()?,
        })
    }
}

impl Save<OptionsRawSave> for OptionsSave {}

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

impl Language {
    fn as_str(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Korean => "kr",
            Language::ChineseSimplified => "cn",
            Language::ChineseTraditional => "cn_tr",
            Language::Japanese => "jp",
            Language::Russian => "ru",
            Language::Bulgarian => "bg",
            Language::SpanishLatinAmerica => "es",
            Language::French => "fr",
            Language::PortugueseBrazil => "pt_br",
            Language::PortuguesePortugal => "pt_pt",
        }
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
        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionsRawSave {
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

impl From<OptionsSave> for OptionsRawSave {
    fn from(value: OptionsSave) -> Self {
        Self {
            master_volume: value.master_volume.0,
            music_volume: value.music_volume.0,
            tooltips_enabled: value.tooltips_enabled.0,
            backer_abnormalities_enabled: value.backer_abnormalities_enabled.0,
            language: value.language.as_str().to_owned(),
            log_index: value.log_index.0.into(),
        }
    }
}

impl RawSave<OptionsSave> for OptionsRawSave {}
