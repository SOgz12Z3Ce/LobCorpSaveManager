use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{
    metadata::abnormality::{Abnormality, TryLookup},
    save::{RawSave, Save},
};

pub(crate) struct EtcSave {
    pub(crate) is_core_suppression_tutorial_played: Flag,
    pub(crate) is_kether_core_suppression_1_completed: Flag,
    pub(crate) is_kether_core_suppression_2_completed: Flag,
    pub(crate) is_kether_core_suppression_3_completed: Flag,
    pub(crate) is_kether_core_suppression_4_completed: Flag,
    pub(crate) is_kether_core_suppression_5_completed: Flag,
    pub(crate) pending_abnormalities: Vec<&'static Abnormality>,
}

pub(crate) struct Flag(bool);

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct EtcRawSave {
    #[serde(rename = "sefirabossTutorialPlayed")]
    is_core_suppression_tutorial_played: bool,

    #[serde(rename = "e0")]
    is_kether_core_suppression_1_completed: bool,

    #[serde(rename = "e1")]
    is_kether_core_suppression_2_completed: bool,

    #[serde(rename = "e2")]
    is_kether_core_suppression_3_completed: bool,

    #[serde(rename = "e3")]
    is_kether_core_suppression_4_completed: bool,

    #[serde(rename = "e4")]
    is_kether_core_suppression_5_completed: bool,

    #[serde(rename = "waitingCreature")]
    pending_abnormalities: Vec<i64>,
}

#[derive(Debug)]
pub(crate) enum Error {
    BadID(i64),
}

impl Display for EtcSave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "==== ETC ====")?;
        writeln!(f, "Core Supreesion Trackers:")?;
        writeln!(
            f,
            "  Core Suppression Tutorial Played: {}",
            self.is_core_suppression_tutorial_played
        )?;
        writeln!(
            f,
            "  Kether Core Suppression 1 Completed: {}",
            self.is_kether_core_suppression_1_completed
        )?;
        writeln!(
            f,
            "  Kether Core Suppression 2 Completed: {}",
            self.is_kether_core_suppression_2_completed
        )?;
        writeln!(
            f,
            "  Kether Core Suppression 3 Completed: {}",
            self.is_kether_core_suppression_3_completed
        )?;
        writeln!(
            f,
            "  Kether Core Suppression 4 Completed: {}",
            self.is_kether_core_suppression_4_completed
        )?;
        writeln!(
            f,
            "  Kether Core Suppression 5 Completed: {}",
            self.is_kether_core_suppression_5_completed
        )?;
        writeln!(f, "Pending Abnormalities:")?;
        write!(
            f,
            "{}",
            self.pending_abnormalities
                .iter()
                .map(|item| item.as_simple_str())
                .collect::<Vec<String>>()
                .join("\n")
        )
        // for abnormality in self.pending_abnormalities.iter() {
        //     writeln!(f, "  {}", abnormality.as_simple_str())?;
        // }
    }
}

impl TryFrom<EtcRawSave> for EtcSave {
    type Error = Error;
    fn try_from(value: EtcRawSave) -> Result<Self, Self::Error> {
        let pending_abnormalities = value
            .pending_abnormalities
            .into_iter()
            .map(|id| Abnormality::try_lookup(id).map_err(|_| Error::BadID(id)))
            .collect::<Result<Vec<&'static Abnormality>, Error>>()?;
        Ok(Self {
            is_core_suppression_tutorial_played: value.is_core_suppression_tutorial_played.into(),
            is_kether_core_suppression_1_completed: value
                .is_kether_core_suppression_1_completed
                .into(),
            is_kether_core_suppression_2_completed: value
                .is_kether_core_suppression_2_completed
                .into(),
            is_kether_core_suppression_3_completed: value
                .is_kether_core_suppression_3_completed
                .into(),
            is_kether_core_suppression_4_completed: value
                .is_kether_core_suppression_4_completed
                .into(),
            is_kether_core_suppression_5_completed: value
                .is_kether_core_suppression_5_completed
                .into(),
            pending_abnormalities: pending_abnormalities,
        })
    }
}

impl Save<EtcRawSave> for EtcSave {}

impl Display for Flag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self(true) => write!(f, "yes"),
            Self(false) => write!(f, "no"),
        }
    }
}

impl From<bool> for Flag {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<EtcSave> for EtcRawSave {
    fn from(value: EtcSave) -> Self {
        Self {
            is_core_suppression_tutorial_played: value.is_core_suppression_tutorial_played.0,
            is_kether_core_suppression_1_completed: value.is_kether_core_suppression_1_completed.0,
            is_kether_core_suppression_2_completed: value.is_kether_core_suppression_2_completed.0,
            is_kether_core_suppression_3_completed: value.is_kether_core_suppression_3_completed.0,
            is_kether_core_suppression_4_completed: value.is_kether_core_suppression_4_completed.0,
            is_kether_core_suppression_5_completed: value.is_kether_core_suppression_5_completed.0,
            pending_abnormalities: value
                .pending_abnormalities
                .into_iter()
                .map(|item| item.id().into())
                .collect::<Vec<i64>>(),
        }
    }
}

impl RawSave<EtcSave> for EtcRawSave {}

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
