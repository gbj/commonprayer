use serde::{
    de::{Error, Unexpected},
    Deserialize, Serialize,
};

mod h82;
mod levas;
mod wlp;

pub use h82::*;
pub use levas::*;
pub use wlp::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Hymnals {
    Hymnal1982,
    LEVAS,
    WLP,
}

impl From<Hymnals> for Hymnal {
    fn from(h: Hymnals) -> Self {
        match h {
            Hymnals::Hymnal1982 => HYMNAL_1982.clone(),
            Hymnals::LEVAS => LEVAS.clone(),
            Hymnals::WLP => WLP.clone(),
        }
    }
}

impl Default for Hymnals {
    fn default() -> Self {
        Self::Hymnal1982
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hymnal {
    pub id: Hymnals,
    pub title: String,
    pub subtitle: String,
    pub copyright: String,
    pub year: u16,
    pub hymns: Vec<Hymn>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hymn {
    pub source: Hymnals,
    pub page_number: u16,
    pub copyright_restriction: bool,
    pub number: HymnNumber,
    pub title: String,
    pub tune: String,
    pub authors: String,
    pub composers: String,
    pub meter: String,
    pub text_sources: String,
    pub tune_sources: String,
    pub text: String,
    pub tags: Vec<String>,
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum HymnNumber {
    S(u16),
    H(u16),
}

impl std::fmt::Display for HymnNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HymnNumber::S(n) => write!(f, "S{}", n),
            HymnNumber::H(n) => write!(f, "{}", n),
        }
    }
}

impl Serialize for HymnNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for HymnNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let st = String::deserialize(deserializer)?;
        let s = st.starts_with('S') || st.starts_with("#S");
        let n = st
            .replace('#', "")
            // strip out S-whatever to get plain number
            .replace("S-", "")
            .replace("S ", "")
            .replace('S', "")
            // strip out alternates/parts
            .replace('a', "")
            .replace('b', "")
            .replace('c', "")
            .replace('d', "")
            .replace('e', "")
            .parse::<u16>()
            .map_err(|_| {
                D::Error::invalid_value(Unexpected::Str(&st), &"an integer hymn number")
            })?;
        if s {
            Ok(HymnNumber::S(n))
        } else {
            Ok(HymnNumber::H(n))
        }
    }
}
