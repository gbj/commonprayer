use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, AsRefStr, Display, EnumIter, EnumString, IntoStaticStr)]
pub enum Season {
    Advent,
    Christmas,
    Epiphany,
    Lent,
    HolyWeek,
    Easter,
    Ascension,
    Pentecost,
    Trinity,
    OrdinaryTime,
    Saints,
    Ember,
    National,
    Thanksgiving,
    Rogation,
    Mary,
    Incarnation,
}

impl Season {
    /// Whether this is truly a liturgical season (Advent, Christmas, Epiphany,
    /// Lent, Easter, Pentecost, Ordinary Time) or simply a marker for the particular day
    /// (a saint’s day, a feast of the Incarnation, etc.)
    pub fn is_true_season(&self) -> bool {
        matches!(
            self,
            Season::Advent
                | Season::Christmas
                | Season::Epiphany
                | Season::Lent
                | Season::Easter
                | Season::Ascension
                | Season::Pentecost
                | Season::Trinity
                | Season::OrdinaryTime
        )
    }
}
