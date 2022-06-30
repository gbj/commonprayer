use canticle_table::CanticleId;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Serialize,
};
use std::{convert::TryFrom, str::FromStr};
use strum_macros::{Display, EnumString};
use thiserror::Error;

use crate::Version;

#[derive(Clone, Default, Debug, Hash, PartialEq, Eq)]
pub struct SlugPath(Vec<Slug>);

impl SlugPath {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_slice(&self) -> &[Slug] {
        self.0.as_slice()
    }

    pub fn push(&mut self, next: Slug) {
        self.0.push(next)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> From<T> for SlugPath
where
    T: IntoIterator<Item = Slug>,
{
    fn from(path: T) -> Self {
        Self(path.into_iter().collect())
    }
}

impl<'a> IntoIterator for &'a SlugPath {
    type Item = &'a Slug;
    type IntoIter = std::slice::Iter<'a, Slug>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).iter()
    }
}

#[derive(Error, Debug, Clone)]
pub enum SlugPathError {
    #[error("could not parse a slug from this path part")]
    SlugNotFound(String),
}

impl FromStr for SlugPath {
    type Err = SlugPathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut path = Vec::new();
        for part in s.split('/') {
            if let Some(slug) = Slug::unslugify(part) {
                path.push(slug);
            } else {
                return Err(SlugPathError::SlugNotFound(part.to_string()));
            }
        }
        Ok(SlugPath(path))
    }
}

impl std::fmt::Display for SlugPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(Slug::slugify)
                .intersperse_with(|| String::from('/'))
                .collect::<String>()
        )
    }
}

impl Serialize for SlugPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SlugPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        SlugPath::from_str(&s)
            .map_err(|_| D::Error::invalid_value(Unexpected::Str(&s), &"a valid Slug in the path"))
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Display, EnumString)]
pub enum Slug {
    // General/Multipurpose
    Version(Version),
    ConcerningTheService,
    AdditionalDirections,
    AdditionalPrayers,
    Order,
    Readings,
    Parallels,
    // Daily Office
    Office,
    MorningPrayer,
    NoondayPrayer,
    ServiceOfLight,
    EveningPrayer,
    Compline,
    Canticles,
    SuggestedCanticles,
    OpeningSentences,
    InvitatoryAntiphons,
    ClosingSentences,
    Canticle(CanticleId),
    // Litany
    GreatLitany,
    // Collects
    Collects,
    // Baptism
    Baptism,
    // Eucharist
    Eucharist,
    PrayersOfThePeople,
    OffertorySentences,
    GreatThanksgiving,
    ProperPrefaces,
    PenitentialOrder,
    PenitentialSentences,
    ConsecratingAdditional,
    PrayerA,
    PrayerB,
    PrayerC,
    PrayerD,
    PrayerI,
    PrayerII,
    Prayer1,
    Prayer2,
    Prayer3,
    // P&Ts
    PrayersAndThanksgivings,
    // Pastoral Offices
    PastoralOffices,
    // Marriage
    Marriage,
    CelebrationAndBlessing,
    CivilMarriage,
    WitnessingAndBlessing,
    WitnessingAndBlessingLifelongCovenant,
    // Burial
    Burial,
    BurialOfAChild,
    BurialOfANonChristian,
    // Occasional Services
    OccasionalServices,
    Guadalupe,
    Renaming,
    // Creeds
    Creeds,
    ApostlesCreed,
    NiceneCreed,
}

impl Slug {
    pub fn slugify(&self) -> String {
        match self {
            Self::Version(version) => {
                let s: &'static str = version.into();
                s.to_string()
            }
            Self::Canticle(id) => id.to_string(),
            _ => slugify(&self.to_string()),
        }
    }

    pub fn unslugify(slug: &str) -> Option<Self> {
        if let Ok(version) = Version::from_str(slug) {
            Some(Self::Version(version))
        } else if let Ok(id) = CanticleId::try_from(slug) {
            Some(Self::Canticle(id))
        } else {
            Self::from_str(&unslugify(slug)).ok()
        }
    }
}

const UPPERCASE: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn slugify(unslugged: &str) -> String {
    let mut splits = unslugged.split_inclusive(UPPERCASE).enumerate().peekable();

    let mut buffer = String::new();

    while let Some((i, part)) = splits.next() {
        let is_first = i == 0;
        let len = part.len();

        if is_first {
            buffer.push_str(&part.to_ascii_lowercase())
        } else {
            let is_last = splits.peek().is_none();
            for (j, ch) in part.chars().enumerate() {
                if j == len - 1 && (!is_last || ch.is_ascii_uppercase()) {
                    buffer.push('-');
                    buffer.push(ch.to_ascii_lowercase());
                } else {
                    buffer.push(ch.to_ascii_lowercase());
                }
            }
        }
    }
    buffer
}

fn unslugify(slug: &str) -> String {
    slug.split('-')
        .map(|word| {
            let mut chars = word.chars();
            let first = chars
                .next()
                .map(|c| c.to_string().to_uppercase())
                .unwrap_or_default();
            format!("{}{}", first, chars.collect::<String>())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::slug::{slugify, unslugify};

    #[test]
    fn slugify_test() {
        assert_eq!(slugify("PrayerA"), "prayer-a");
        assert_eq!(slugify("NiceneCreed"), "nicene-creed");
        assert_eq!(slugify("WitnessingAndBlessing"), "witnessing-and-blessing");
    }

    #[test]
    fn unslugify_test() {
        assert_eq!(unslugify("prayer-a"), "PrayerA");
        assert_eq!(
            unslugify("prayers-and-thanksgivings"),
            "PrayersAndThanksgivings"
        );
        assert_eq!(unslugify("marriage"), "Marriage");
    }
}
