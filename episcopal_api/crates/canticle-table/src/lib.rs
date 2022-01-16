use calendar::{Calendar, LiturgicalDay, Rank, Season, Weekday};
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate lazy_static;

pub mod bcp1979;
pub mod eow;
mod id;
pub use id::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CanticleTable(Vec<CanticleTableEntry>);

impl<T> From<T> for CanticleTable
where
    T: IntoIterator<Item = CanticleTableEntry>,
{
    fn from(entries: T) -> Self {
        Self(entries.into_iter().collect())
    }
}

impl CanticleTable {
    pub fn find(
        &self,
        calendar: &Calendar,
        day: &LiturgicalDay,
        nth: CanticleNumber,
        fallback: Option<&CanticleTable>,
        traditional_language: bool,
    ) -> Vec<CanticleId> {
        let primary_table_options = self
            .0
            .iter()
            .filter(|entry| {
                let is_feast = calendar.rank(day) > Rank::HolyDay;
                let is_evening = day.evening;

                let feast_match = is_feast == entry.feast_day;

                // whether this canticle is used on this weekday
                let weekday_match = entry
                    .weekday
                    .map(|weekday| weekday == day.weekday)
                    // entries with no weekday work for all weekdays
                    .unwrap_or(true);

                // whether this canticle is used in this week
                let season_match = entry
                    .season
                    .map(|season| season == calendar.season(day))
                    // entries with no season work for all seasons
                    .unwrap_or(true);

                let evening_match = is_evening == entry.evening;
                let number_match = nth == entry.nth;

                evening_match && number_match && season_match && weekday_match && feast_match
            })
            .map(|entry| entry.canticle)
            .collect::<Vec<_>>();

        let options = if primary_table_options.is_empty() {
            match fallback {
                Some(fallback) => fallback.find(calendar, day, nth, None, traditional_language),
                None => primary_table_options,
            }
        } else {
            primary_table_options
        };

        if options.is_empty() {
            let default = match (traditional_language, day.evening, nth) {
                (true, true, CanticleNumber::One) => CanticleId::Canticle4, // Te Deum
                (true, true, CanticleNumber::Two) => CanticleId::Canticle7, // Benedictus
                (true, false, CanticleNumber::One) => CanticleId::Canticle3, // Magnificat
                (true, false, CanticleNumber::Two) => CanticleId::Canticle5, // Nunc dimittis
                (false, true, CanticleNumber::One) => CanticleId::Canticle16, // Te Deum
                (false, true, CanticleNumber::Two) => CanticleId::Canticle21, // Benedictus
                (false, false, CanticleNumber::One) => CanticleId::Canticle15, // Magnificat
                (false, false, CanticleNumber::Two) => CanticleId::Canticle17, // Nunc dimittis
            };
            vec![default]
        } else {
            options
        }
    }
}

/// A single entry in a table of suggested canticles, listing which canticle should
/// be used for the first or second canticle in the morning or evening in a given season.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CanticleTableEntry {
    canticle: CanticleId,
    evening: bool,
    nth: CanticleNumber,
    feast_day: bool,
    /// If `None`, can be used on any weekday.
    weekday: Option<Weekday>,
    /// If `None`, can be used in any season.
    season: Option<Season>,
}

/// Whether this the first or second canticle in the liturgy.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CanticleNumber {
    One,
    Two,
}
