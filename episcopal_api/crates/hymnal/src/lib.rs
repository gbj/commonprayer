use std::str::FromStr;

use serde::{
    de::{Error, Unexpected},
    Deserialize, Serialize,
};
use thiserror::Error;

mod el_himnario;
mod h82;
mod levas;
mod wlp;

pub use el_himnario::*;
pub use h82::*;
pub use levas::*;
pub use wlp::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Hymnals {
    Hymnal1982,
    LEVAS,
    WLP,
    ElHimnario
}

impl Hymnals {
    /// Returns an ID for the hymnal on Hymnary.org.
    pub fn hymnary_id(&self) -> &'static str {
        match self {
            Hymnals::Hymnal1982 => "EH1982",
            Hymnals::LEVAS => "LEVS1993",
            Hymnals::WLP => "WLP1997",
            Hymnals::ElHimnario => "EH1998",
        }
    }
}

impl std::fmt::Display for Hymnals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Hymnals::Hymnal1982 => "H82",
                Hymnals::LEVAS => "LEVAS",
                Hymnals::WLP => "WLP",
                Hymnals::ElHimnario => "El Himnario"
            }
        )
    }
}

#[derive(Copy, Clone, Error, Debug, PartialEq, Eq, Hash)]
pub enum HymnalFromStrError {
    #[error("could not find hymnal")]
    NotFound
}

impl FromStr for Hymnals {
    type Err = HymnalFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H82" => Ok(Hymnals::Hymnal1982),
            "Hymnal1982" => Ok(Hymnals::Hymnal1982),
            "LEVAS" => Ok(Hymnals::LEVAS),
            "WLP" => Ok(Hymnals::WLP),
            "El Himnario" => Ok(Hymnals::ElHimnario),
            "ElHimnario" => Ok(Hymnals::ElHimnario),
            _ => Err(HymnalFromStrError::NotFound)
        }
    }
}

impl From<Hymnals> for Hymnal {
    fn from(h: Hymnals) -> Self {
        match h {
            Hymnals::Hymnal1982 => HYMNAL_1982.clone(),
            Hymnals::LEVAS => LEVAS.clone(),
            Hymnals::WLP => WLP.clone(),
            Hymnals::ElHimnario => EL_HIMNARIO.clone()
        }
    }
}

impl Default for Hymnals {
    fn default() -> Self {
        Self::Hymnal1982
    }
}

impl Hymnals {
    pub fn is_default(&self) -> bool {
        self == &Self::default()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hymnal {
    pub id: Hymnals,
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub subtitle: String,
    pub copyright: String,
    pub year: u16,
    pub hymns: Vec<Hymn>,
}

impl Hymnal {
    pub fn to_metadata(&self) -> Self {
        Self {
            hymns: Vec::new(),
            ..self.clone()
        }
    }

    pub fn search(&self, search: &str) -> impl Iterator<Item = HymnMetadata> + '_ {
        if search.is_empty() {
            Box::new(self.hymns.iter().map(Hymn::to_metadata)) as Box<dyn Iterator<Item = HymnMetadata>>
        } else {
            let orig_search = search.to_string();
            let search = orig_search.clone();

            let tag = if search.starts_with("tag:") {
                Some(search.replace("tag:", ""))
            } else {
                None
            };

            Box::new(self
                .hymns
                .iter()
                .filter(move |hymn| {
                    if let Some(tag) = &tag {
                        hymn.tags.contains(tag)
                    } else {
                        let search = strip_non_word_characters(&search.to_lowercase());
                        let number = strip_non_word_characters(&hymn.number.to_string().to_lowercase());
                        let title = strip_non_word_characters(&hymn.title.to_lowercase());
                        let tune = strip_non_word_characters(&hymn.tune.to_lowercase());
                        let authors = strip_non_word_characters(&hymn.authors.to_lowercase());
                        let composers = strip_non_word_characters(&hymn.composers.to_lowercase());
                        let text = strip_non_word_characters(&hymn.text.to_lowercase());

                        number.contains(&search)
                            || title.contains(&search)
                            || tune.contains(&search)
                            || authors.contains(&search)
                            || composers.contains(&search)
                            || hymn.meter.contains(&orig_search)
                            || text.contains(&search)
                    }
                })
                .map(|hymn| hymn.to_metadata()))
            }
    }
}

fn strip_non_word_characters(original: &str) -> String {
    original.chars().filter(|ch| 
        // so that date ranges don't get read as numbers, i.e., "111" should not match "1711-1759"
        ch == &'-'
        // letters
        || ('a'..='z').contains(ch)
        // digits so can search by hymn number
        || ('0'..='9').contains(ch)).collect()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub struct Hymn {
    #[serde(skip_serializing_if = "Hymnals::is_default", default)]
    pub source: Hymnals,
    #[serde(skip_serializing_if = "is_zero", default)]
    pub page_number: u16,
    #[serde(skip_serializing_if = "is_false", default)]
    pub copyright_restriction: bool,
    pub number: HymnNumber,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub tune: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub authors: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub composers: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub meter: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub text_sources: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub tune_sources: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,
}

impl Hymn {
    pub fn to_metadata(&self) -> HymnMetadata {
        let text_empty = self.text.is_empty();
        let Hymn { source, number, title, tune, copyright_restriction, authors, composers, meter, tags, .. } = self.clone();
        HymnMetadata { source, number, title, tune, copyright_restriction, text_empty, authors, composers, meter, tags }
    }

    /// Returns the URL of an entry for this hymn on RiteSong.
    pub fn rite_song_link(&self) -> Option<String> {
        let hymnal = self.source;
        let number = self.number;
        let id = match (hymnal, number) {
            (Hymnals::Hymnal1982, HymnNumber::S(n)) => Some(1353 + (n - 1)),
            (Hymnals::Hymnal1982, HymnNumber::H(n)) => Some(193 + (n - 1)),
            (Hymnals::LEVAS, HymnNumber::H(n)) => Some(913 + (n - 1)),
            (Hymnals::LEVAS, HymnNumber::S(n)) => Some(913 + (n - 1)),
            (Hymnals::WLP, HymnNumber::H(n)) => Some(1968 + (n - 721)),
            (Hymnals::WLP, HymnNumber::S(n)) => Some(1968 + (n - 721)),
            (Hymnals::ElHimnario, _) => None,
        };

        let base = match (hymnal, number) {
            (Hymnals::Hymnal1982, HymnNumber::S(_n)) => {
                Some("https://www.riteseries.org/song/Hymnal1982ServiceMusic/")
            }
            (Hymnals::Hymnal1982, HymnNumber::H(_n)) => {
                Some("https://www.riteseries.org/song/Hymnal1982/")
            }
            (Hymnals::LEVAS, _) => Some("https://www.riteseries.org/song/levs/"),
            (Hymnals::WLP, _) => Some("https://www.riteseries.org/song/wlp/"),
            (Hymnals::ElHimnario, _) => None,
        };

        if let (Some(base), Some(id)) = (base, id) {
            Some(format!("{}{}/", base, id))
        } else {
            None
        }
    }

    /// Returns the URL of an entry for this hymn on Hymnary.org.
    pub fn hymnary_link(&self) -> String {
        let hymnal_id = self.source.hymnary_id();
        let page = self.page_number;
        format!(
            "https://hymnary.org/page/fetch/{hymnal_id}/{page}/high",
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HymnMetadata {
    #[serde(skip_serializing_if = "Hymnals::is_default", default)]
    pub source: Hymnals,
    pub number: HymnNumber,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub tune: String,
    #[serde(skip_serializing_if = "is_false", default)]
    pub copyright_restriction: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub text_empty: bool,
   // #[serde(skip_serializing_if = "String::is_empty", default)]
    pub authors: String,
   // #[serde(skip_serializing_if = "String::is_empty", default)]
    pub composers: String,
   // #[serde(skip_serializing_if = "String::is_empty", default)]
    pub meter: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,
}

fn is_zero(number: &u16) -> bool {
    *number == 0
}

fn is_false(b: &bool) -> bool {
    !(*b)
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

#[derive(Clone, Error, Debug, PartialEq, Eq, Hash)]
pub enum HymnNumberFromStrError {
    #[error("could not parse hymn number")]
    InvalidNumber(String)
}


impl FromStr for HymnNumber {
    type Err = HymnNumberFromStrError;

    fn from_str(st: &str) -> Result<Self, Self::Err> {
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
                HymnNumberFromStrError::InvalidNumber(st.to_string())
            })?;
        if s {
            Ok(HymnNumber::S(n))
        } else {
            Ok(HymnNumber::H(n))
        }
    }
}

impl Default for HymnNumber {
    fn default() -> Self {
        HymnNumber::H(1)
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
