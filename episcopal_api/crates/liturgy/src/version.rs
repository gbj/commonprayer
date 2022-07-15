use language::Language;
use std::{convert::TryFrom, fmt::Display, str::FromStr};
use strum_macros::{EnumIter, IntoStaticStr};
use thiserror::Error;

use serde::{Deserialize, Serialize};

/// Different versions that a liturgical [Document](crate::Document) could be (e.g., Rite I, Rite II, EOW)
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
    IntoStaticStr,
    PartialOrd,
    Ord,
)]
pub enum Version {
    /// 1979 Book of Common Prayer
    BCP1979,
    /// Rite I (traditional language in 1979 BCP)
    RiteI,
    /// Rite II (contemporary language in 1979 BCP)
    RiteII,
    /// Book of Occasional Services
    BOS,
    /// Spanish-language translation of 1979 BCP
    LibroDeOracionComun,
    /// Enriching Our Worship series
    EOW,
    /// Expansive-language Eucharistic and marriage texts authorized by General Convention
    Expansive,
    // A document that's intended to show parallels between multiple other versions
    Parallel,
    /// The New Revised Standard Version of the Bible
    NRSV,
    /// The New Revised Standard Version (Anglicized Edition)
    NRSVAE,
    /// The English Standard Version of the Bible
    ESV,
    /// The Authorized Version (“King James Version”) of the Bible
    KJV,
    /// The Common English Bible
    CEB,
    /// The Reina-Valera (1909)
    RV09,
}

impl Version {
    pub fn is_bible_translation(&self) -> bool {
        matches!(
            self,
            Version::NRSV | Version::NRSVAE | Version::ESV | Version::KJV | Version::CEB
        )
    }

    pub fn is_subset_of(&self, other: &Version) -> bool {
        matches!(
            (self, other),
            (Version::RiteI, Version::BCP1979) | (Version::RiteII, Version::BCP1979)
        )
    }

    pub fn is_default(&self) -> bool {
        self == &Self::default()
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Version::BCP1979 => "1979",
            Version::RiteI => "Rite I",
            Version::RiteII => "Rite II",
            Version::EOW => "EOW",
            Version::BOS => "BOS",
            Version::LibroDeOracionComun => "LOC",
            Version::Expansive => "Expansive",
            Version::NRSV => "NRSV",
            Version::NRSVAE => "NRSVAE",
            Version::ESV => "ESV",
            Version::KJV => "KJV",
            Version::CEB => "CEB",
            Version::RV09 => "Reina-Valera",
            Version::Parallel => "Parallels",
        };
        write!(f, "{}", label)
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::BCP1979
    }
}

#[derive(Error, Debug)]
pub enum VersionConversionError {
    #[error("invalid version given")]
    Invalid(String),
}

impl TryFrom<&str> for Version {
    type Error = VersionConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "BCP1979" => Ok(Self::BCP1979),
            "1979" => Ok(Self::BCP1979),
            "RiteI" => Ok(Self::RiteI),
            "Rite I" => Ok(Self::RiteI),
            "RiteII" => Ok(Self::RiteII),
            "Rite II" => Ok(Self::RiteII),
            "EOW" => Ok(Self::EOW),
            "BOS" => Ok(Self::BOS),
            "LibroDeOracionComun" => Ok(Self::LibroDeOracionComun),
            "LOC" => Ok(Self::LibroDeOracionComun),
            "Expansive" => Ok(Self::Expansive),
            "NRSVAE" => Ok(Self::NRSVAE),
            "NRSV" => Ok(Self::NRSV),
            "ESV" => Ok(Self::ESV),
            "KJV" => Ok(Self::KJV),
            "CEB" => Ok(Self::CEB),
            "Parallels" => Ok(Self::Parallel),
            "Parallel" => Ok(Self::Parallel),
            "Reina-Valera" => Ok(Self::RV09),
            "RV" => Ok(Self::RV09),
            "RV09" => Ok(Self::RV09),
            _ => Err(VersionConversionError::Invalid(value.to_string())),
        }
    }
}

impl FromStr for Version {
    type Err = VersionConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<Version> for Language {
    fn from(version: Version) -> Self {
        match version {
            Version::LibroDeOracionComun | Version::RV09 => Language::Es,
            _ => Language::En,
        }
    }
}
