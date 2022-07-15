use calendar::{Calendar, LiturgicalDay, Rank, Season, Weekday};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

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
        let day_season = calendar.season(day);

        let primary_table_options = self
            .0
            .iter()
            .filter(|entry| {
                let day_rank = calendar.rank(day);
                // only use "Feast Day" canticles for actual feasts, not days like
                // Ash Wednesday/Thursday/Friday
                let is_feast = day_rank >= Rank::HolyDay && day_rank != Rank::PrecedenceOverHolyDay;
                let is_evening = day.evening;

                let feast_match = is_feast == entry.feast_day;

                let weekday_match = entry.is_weekday_match(&day.weekday);
                let season_match = entry.is_season_match(&day_season);
                let evening_match = is_evening == entry.evening;
                let number_match = nth == entry.nth;

                evening_match && number_match && season_match && weekday_match && feast_match
            })
            .collect::<Vec<_>>();

        // if there's one specific for a season, use that instead of the general one
        let primary_table_options = if primary_table_options.len() > 1
            && primary_table_options.iter().any(|entry| {
                entry.is_season_match(&day_season) && entry.is_weekday_match(&day.weekday)
            }) {
            primary_table_options
                .iter()
                .filter(|entry| !(entry.is_weekday_match(&day.weekday) && entry.season == None))
                .map(|entry| entry.canticle)
                .collect::<Vec<_>>()
        } else {
            primary_table_options
                .iter()
                .map(|entry| entry.canticle)
                .collect::<Vec<_>>()
        };

        // fallback table
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

impl CanticleTableEntry {
    pub fn is_weekday_match(&self, weekday: &Weekday) -> bool {
        self.weekday
            .map(|w| &w == weekday)
            // entries with no weekday work for all weekdays
            .unwrap_or(true)
    }

    pub fn is_season_match(&self, season: &Season) -> bool {
        self.season
            .map(|s| &s == season)
            // entries with no season work for all seasons
            .unwrap_or(true)
    }
}

/// Whether this the first or second canticle in the liturgy.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize, EnumString, Display)]
pub enum CanticleNumber {
    One,
    Two,
}

#[cfg(test)]
mod tests {
    use calendar::{Date, BCP1979_CALENDAR};

    use crate::{bcp1979::BCP1979_CANTICLE_TABLE_RITE_II, CanticleId, CanticleNumber};

    #[test]
    fn ash_wednesday_bcp1979() {
        let day = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 3, 2), false);
        let one = BCP1979_CANTICLE_TABLE_RITE_II.find(
            &BCP1979_CALENDAR,
            &day,
            CanticleNumber::One,
            None,
            false,
        );
        assert_eq!(one, vec![CanticleId::Canticle14]);
        let two = BCP1979_CANTICLE_TABLE_RITE_II.find(
            &BCP1979_CALENDAR,
            &day,
            CanticleNumber::Two,
            None,
            false,
        );
        assert_eq!(two, vec![CanticleId::Canticle16]);
    }

    #[test]
    fn thursday_after_ash_wednesday_bcp1979() {
        let day = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2022, 3, 3), false);
        let one = BCP1979_CANTICLE_TABLE_RITE_II.find(
            &BCP1979_CALENDAR,
            &day,
            CanticleNumber::One,
            None,
            false,
        );
        assert_eq!(one, vec![CanticleId::Canticle8]);
        let two = BCP1979_CANTICLE_TABLE_RITE_II.find(
            &BCP1979_CALENDAR,
            &day,
            CanticleNumber::Two,
            None,
            false,
        );
        assert_eq!(two, vec![CanticleId::Canticle19]);
    }
}
