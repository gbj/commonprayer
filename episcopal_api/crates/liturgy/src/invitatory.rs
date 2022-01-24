use serde::{Deserialize, Serialize};

use crate::{Antiphon, GloriaPatri};

/// An Invitatory Psalm, which is handled and displayed differently from a psalm
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Invitatory {
    /// Name for the canticle in its own language
    pub local_name: String,
    /// Latin name for the canticle
    pub latin_name: Option<String>,
    /// Biblical or other citation for the source of the canticle's text
    pub citation: Option<String>,
    /// Whether to insert a seasonal antiphon before/after the invitatory, and between sections
    pub antiphon: SeasonalAntiphon,
    /// The content of the psalm, by section
    pub sections: Vec<InvitatorySection>,
    /// The text of the Gloria Patri, to be included (or not) at the end of the invitatory
    pub gloria_patri: Option<GloriaPatri>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum SeasonalAntiphon {
    Omit,
    Insert,
    Antiphon(Antiphon),
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct InvitatorySection {
    pub verses: Vec<InvitatoryVerse>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct InvitatoryVerse {
    pub a: String,
    pub b: String,
}
