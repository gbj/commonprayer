use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{CanticleTables, PreferenceKey};
use canticle_table::{CanticleId, CanticleNumber};

/// An entry that can be looked up from a [CanticleTable](canticle_table::CanticleTable).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CanticleTableEntry {
    pub nth: CanticleNumber,
    pub table: CanticleTableChoice,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CanticleTableChoice {
    /// Dynamically loads the canticle table selected in the specified preference
    Preference(PreferenceKey),
    /// Statically uses the chosen canticle table
    Selected(CanticleTables),
}

/// A Canticle (i.e., a psalm-like text not found in the Book of Psalms, and used liturgically)
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Canticle {
    /// Unique identifier for the canticle; may be shared between different versions or translations
    pub number: CanticleId,
    /// Biblical or other citation for the source of the canticle's text
    pub citation: Option<String>,
    /// Name for the canticle in its own language
    pub local_name: String,
    /// Latin name for the canticle
    pub latin_name: Option<String>,
    /// The content of the psalm, by section
    pub sections: Vec<CanticleSection>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CanticleSection {
    /// Title of section, if any
    pub title: Option<String>,
    /// The set of verses included in this section
    pub verses: Vec<CanticleVerse>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CanticleVerse {
    /// Text of the first half of the verse, up to the asterisk
    pub a: String,
    /// Text of the second half of the verse, after the asterisk
    pub b: String,
}

impl<A, B> From<(A, B)> for CanticleVerse
where
    A: Display,
    B: Display,
{
    fn from((a, b): (A, B)) -> Self {
        Self {
            a: a.to_string(),
            b: b.to_string(),
        }
    }
}
