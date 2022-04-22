use serde::{
    de::{Error, Unexpected},
    Deserialize, Serialize,
};

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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub fn search(&self, search: &str) -> impl Iterator<Item = HymnNumber> + '_ {
        let orig_search = search.to_string();
        let search = orig_search.clone();

        let tag = if search.starts_with("tag:") {
            Some(search.replace("tag:", ""))
        } else {
            None
        };

        self
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
            .map(|hymn| hymn.number)
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
