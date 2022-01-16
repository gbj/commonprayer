use serde::{Deserialize, Serialize};

use crate::{Feast, LiturgicalWeek, Rank, Sanctoral, Season, Weekday};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HolyDay {
    identifier: Feast,
    sanctoral: Sanctoral,
    rank: Rank,
    season: Option<Season>,
    octave: Option<HolyDayId>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HolyDayId {
    /// Holy days defined by a month/day pair (e.g., Christmas is 12/25, St. Luke's Day is 10/18)
    Date(u8, u8),
    /// e.g., Ascension Day is the Thursday after the Sixth Sunday of Easter
    /// ```no_compile
    /// let ascension = HolyDayId::SpecialDay(LiturgicalWeek::Easter6, Weekday::Thu)
    /// ```
    SpecialDay(LiturgicalWeek, Weekday),
    /// e.g., (American) Thanksgiving Day is the fourth Thursday in November
    /// ```no_compile
    /// let thanksgiving = HolyDayId::DayOfMonth { month: 11, week: 4, day: Weekday::Thu }
    /// ```
    DayOfMonth { month: u8, week: u8, day: Weekday },
}
