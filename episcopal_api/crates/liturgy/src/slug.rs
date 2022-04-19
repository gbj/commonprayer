use canticle_table::CanticleId;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, str::FromStr};
use strum_macros::{Display, EnumString};

use crate::Version;

#[derive(Clone, Debug, Hash)]
pub struct SlugPath(Vec<Slug>);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Display, EnumString)]
pub enum Slug {
    // General/Multipurpose
    Version(Version),
    ConcerningTheService,
    AdditionalDirections,
    AdditionalPrayers,
    Order,
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
    // Baptism
    Baptism,
    // Eucharist
    Eucharist,
    PrayersOfThePeople,
    OffertorySentences,
    ProperPrefaces,
    // P&Ts
    PrayersAndThanksgivings,
    // Marriage
    Marriage,
    CelebrationAndBlessing,
    WitnessingAndBlessing,
    // Burial
    Burial,
    // Shared
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
        let cased = part.chars().enumerate().map(move |(j, c)| {
            if j == len - 1 {
                c.to_ascii_lowercase()
            } else {
                c
            }
        });
        if is_first {
            for ch in cased {
                buffer.push(ch)
            }
        } else {
            let is_last = splits.peek().is_none();
            for (j, ch) in cased.enumerate() {
                if j == len - 1 && !is_last {
                    buffer.push('-');
                    buffer.push(ch);
                } else {
                    buffer.push(ch);
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
        assert_eq!(slugify("NiceneCreed"), "nicene-creed");
        assert_eq!(slugify("WitnessingAndBlessing"), "witnessing-and-blessing");
    }

    #[test]
    fn unslugify_test() {
        assert_eq!(
            unslugify("prayers-and-thanksgivings"),
            "PrayersAndThanksgivings"
        );
        assert_eq!(unslugify("marriage"), "Marriage");
    }
}
