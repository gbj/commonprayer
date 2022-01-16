use std::{convert::TryFrom, fmt::Display};

use serde::{Deserialize, Serialize};

/// Different versions that a liturgical [Document](crate::Document) could be (e.g., Rite I, Rite II, EOW)
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Version {
    /// 1979 Book of Common Prayer
    BCP1979,
    /// Rite I (traditional language in 1979 BCP)
    RiteI,
    /// Rite II (contemporary language in 1979 BCP)
    RiteII,
    /// Enriching Our Worship series
    EOW,
    /// Expansive-language Eucharistic texts authorized by General Convention
    Expansive,
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
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Version::BCP1979 => "1979",
            Version::RiteI => "Rite I",
            Version::RiteII => "Rite II",
            Version::EOW => "EOW",
            Version::Expansive => "Expansive",
            Version::NRSV => "NRSV",
            Version::NRSVAE => "NRSVAE",
            Version::ESV => "ESV",
            Version::KJV => "KJV",
            Version::CEB => "CEB",
        };
        write!(f, "{}", label)
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::BCP1979
    }
}

impl TryFrom<&str> for Version {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1979" => Ok(Self::BCP1979),
            "RiteI" => Ok(Self::RiteI),
            "Rite I" => Ok(Self::RiteI),
            "RiteII" => Ok(Self::RiteII),
            "Rite II" => Ok(Self::RiteII),
            "EOW" => Ok(Self::EOW),
            "Expansive" => Ok(Self::Expansive),
            "NRSV" => Ok(Self::NRSV),
            "ESV" => Ok(Self::ESV),
            "KJV" => Ok(Self::KJV),
            "CEB" => Ok(Self::CEB),
            _ => Err(()),
        }
    }
}
