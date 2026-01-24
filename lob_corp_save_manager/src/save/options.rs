use core::str;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::save::{RawSave, Save};

pub struct OptionsSave {
    pub language: Language,
    pub tooltips_enabled: Toggle,
    pub backer_abnormalities_enabled: Toggle,
    pub master_volume: Factor,
    pub music_volume: Percentage,
    pub last_log_index: i32,
}

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

pub struct Toggle(bool);

pub struct Factor(f32);

pub struct Percentage(f32);

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
    last_log_index: i32,
}

#[derive(Debug)]
pub enum Error {
    InvalidLanguage(String),
    UnsupportedLanguage(String),
    InvalidFactorValue(f32),
    InvalidPercentageValue(f32),
}

impl Display for OptionsSave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "==== Options ====")?;
        writeln!(f, "Game:")?;
        writeln!(f, "  Language: {}", self.language)?;
        writeln!(f, "  Display Tooltips: {}", self.tooltips_enabled)?;
        writeln!(
            f,
            "  Enable Backer Abnormalities: {}",
            self.backer_abnormalities_enabled
        )?;

        writeln!(f, "Audio:")?;
        writeln!(f, "  Master Volume: {}", self.master_volume)?;
        writeln!(f, "  Music Volume: {}", self.music_volume)?;

        writeln!(f, "Debug:")?;
        write!(f, "  Last Log Index: {}", self.last_log_index)?;
        Ok(())
    }
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
            last_log_index: value.last_log_index,
        })
    }
}

impl Save<OptionsRawSave> for OptionsSave {}

impl Language {
    fn as_raw_str(&self) -> &'static str {
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

impl Display for Toggle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self(true) => write!(f, "on"),
            Self(false) => write!(f, "off"),
        }
    }
}

impl From<bool> for Toggle {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Display for Factor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x", self.0)
    }
}

impl TryFrom<f32> for Factor {
    type Error = Error;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value > 0.0 {
            Ok(Self(value))
        } else {
            Err(Error::InvalidFactorValue(value))
        }
    }
}

impl Display for Percentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0 * 100.0)
    }
}

impl TryFrom<f32> for Percentage {
    type Error = Error;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value >= 0.0 && value <= 1.0 {
            Ok(Self(value))
        } else {
            Err(Error::InvalidPercentageValue(value))
        }
    }
}

impl From<OptionsSave> for OptionsRawSave {
    fn from(value: OptionsSave) -> Self {
        Self {
            master_volume: value.master_volume.0,
            music_volume: value.music_volume.0,
            tooltips_enabled: value.tooltips_enabled.0,
            backer_abnormalities_enabled: value.backer_abnormalities_enabled.0,
            language: value.language.as_raw_str().to_owned(),
            last_log_index: value.last_log_index,
        }
    }
}

impl RawSave<OptionsSave> for OptionsRawSave {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidLanguage(s) => write!(f, "{} is not a vaild language.", s)?,
            Error::UnsupportedLanguage(s) => write!(f, "{} is not suppored in regular game.", s)?,
            Error::InvalidFactorValue(v) => write!(f, "{} is negative.", v)?,
            Error::InvalidPercentageValue(v) => write!(f, "{} is not in [0, 1].", v)?,
        }
        Ok(())
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
