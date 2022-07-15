use language::Language;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

use crate::{Version, SlugPath};
use lectionary::ReadingType;

/// An explanatory sentence or direction for the liturgy
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum PreferenceKey {
    Global(GlobalPref),
    Local(String),
}

#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    AsRefStr,
    EnumIter,
    EnumString,
    IntoStaticStr,
)]
pub enum GlobalPref {
    Language,
    Calendar,
    Version,
    BibleVersion,
    PsalterVersion,
    Lectionary,
    PsalmCycle,
    CanticleTable,
    OmitForeOffice,
    ReadingA,
    ReadingB,
    ReadingC,
    CanticleOne,
    CanticleTwo,
    UseBlackLetterCollects,
    /// Whether to insert the Gloria Patri after each psalm in the Daily Office, or only at the end of the psalms
    InsertGloria,
    GloriaPatriTraditional,
}

impl<T> From<T> for PreferenceKey
where
    T: Display,
{
    fn from(key: T) -> Self {
        Self::Local(key.to_string())
    }
}

impl From<GlobalPref> for PreferenceKey {
    fn from(key: GlobalPref) -> Self {
        Self::Global(key)
    }
}

#[derive(
    Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize, EnumIter, EnumString, IntoStaticStr,
)]
pub enum PreferenceValue {
    Language(Language),
    Version(Version),
    Lectionary(Lectionaries),
    CanticleTable(CanticleTables),
    Canticle(SlugPath),
    ReadingType(ReadingType),
    Local(String),
    Bool(bool),
}

impl From<Language> for PreferenceValue {
    fn from(language: Language) -> Self {
        Self::Language(language)
    }
}

impl From<Version> for PreferenceValue {
    fn from(version: Version) -> Self {
        Self::Version(version)
    }
}

impl From<Lectionaries> for PreferenceValue {
    fn from(lectionary: Lectionaries) -> Self {
        Self::Lectionary(lectionary)
    }
}

impl From<CanticleTables> for PreferenceValue {
    fn from(table: CanticleTables) -> Self {
        Self::CanticleTable(table)
    }
}

impl From<ReadingType> for PreferenceValue {
    fn from(reading_type: ReadingType) -> Self {
        Self::ReadingType(reading_type)
    }
}

impl From<bool> for PreferenceValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl Default for PreferenceValue {
    fn default() -> Self {
        Self::Local(Default::default())
    }
}

pub trait ClientPreferences {
    fn value(&self, key: &PreferenceKey) -> Option<&PreferenceValue>;
}

impl ClientPreferences for [(PreferenceKey, PreferenceValue); 0] {
    fn value(&self, _key: &PreferenceKey) -> Option<&PreferenceValue> {
        None
    }
}

impl ClientPreferences for HashMap<PreferenceKey, PreferenceValue> {
    fn value(&self, key: &PreferenceKey) -> Option<&PreferenceValue> {
        self.get(key)
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    EnumIter,
    EnumString,
    IntoStaticStr,
    Display,
)]
pub enum Lectionaries {
    BCP1979DailyOffice,
    BCP1979DailyOfficePsalms,
    BCP1979ThirtyDayPsalms,
    RCLTrack1,
    RCLTrack2,
}

impl Default for Lectionaries {
    fn default() -> Self {
        Self::BCP1979DailyOffice
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    EnumIter,
    EnumString,
    IntoStaticStr,
    Display,
)]
pub enum CanticleTables {
    BCP1979RiteI,
    BCP1979RiteII,
    EOW,
    Classical,
}

impl Default for CanticleTables {
    fn default() -> Self {
        Self::BCP1979RiteII
    }
}
