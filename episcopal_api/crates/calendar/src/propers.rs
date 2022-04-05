use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

use crate::Date;

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Debug,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    AsRefStr,
    Display,
    EnumIter,
    EnumString,
    IntoStaticStr,
)]
pub enum Proper {
    Proper1,
    Proper2,
    Proper3,
    Proper4,
    Proper5,
    Proper6,
    Proper7,
    Proper8,
    Proper9,
    Proper10,
    Proper11,
    Proper12,
    Proper13,
    Proper14,
    Proper15,
    Proper16,
    Proper17,
    Proper18,
    Proper19,
    Proper20,
    Proper21,
    Proper22,
    Proper23,
    Proper24,
    Proper25,
    Proper26,
    Proper27,
    Proper28,
    Proper29,
}

const PROPERS: [(u8, u8, Option<Proper>); 31] = [
    (5, 4, None),
    (5, 11, Some(Proper::Proper1)),
    (5, 18, Some(Proper::Proper2)),
    (5, 25, Some(Proper::Proper3)),
    (6, 1, Some(Proper::Proper4)),
    (6, 8, Some(Proper::Proper5)),
    (6, 15, Some(Proper::Proper6)),
    (6, 22, Some(Proper::Proper7)),
    (6, 29, Some(Proper::Proper8)),
    (7, 6, Some(Proper::Proper9)),
    (7, 13, Some(Proper::Proper10)),
    (7, 20, Some(Proper::Proper11)),
    (7, 27, Some(Proper::Proper12)),
    (8, 3, Some(Proper::Proper13)),
    (8, 10, Some(Proper::Proper14)),
    (8, 17, Some(Proper::Proper15)),
    (8, 24, Some(Proper::Proper16)),
    (8, 31, Some(Proper::Proper17)),
    (9, 7, Some(Proper::Proper18)),
    (9, 14, Some(Proper::Proper19)),
    (9, 21, Some(Proper::Proper20)),
    (9, 28, Some(Proper::Proper21)),
    (10, 5, Some(Proper::Proper22)),
    (10, 12, Some(Proper::Proper23)),
    (10, 19, Some(Proper::Proper24)),
    (10, 26, Some(Proper::Proper25)),
    (11, 2, Some(Proper::Proper26)),
    (11, 9, Some(Proper::Proper27)),
    (11, 16, Some(Proper::Proper28)),
    (11, 23, Some(Proper::Proper29)),
    (11, 30, None),
];

/// Calculates propers for season after Pentecost.
/// ```
/// # use crate::calendar::{Date, Proper, propers::calculate_proper};
/// // Dates within Ordinary Time are assigned propers
/// let date = Date::from_ymd(2020, 7, 25);
/// assert_eq!(calculate_proper(date), Some(Proper::Proper11));
/// let date = Date::from_ymd(2021, 11, 16);
/// assert_eq!(calculate_proper(date), Some(Proper::Proper28));
/// // Dates that are always before/after Ordinary Time get no proper
/// let date = Date::from_ymd(2020, 12, 23);
/// assert_eq!(calculate_proper(date), None);
/// let date = Date::from_ymd(2020, 4, 10);
/// assert_eq!(calculate_proper(date), None);
/// // Dates that *sometimes* fall within Ordinary Time are given propers,
/// // even if these will be overridden (e.g., if the date falls before Pentecost this year)
/// let date = Date::from_ymd(2020, 5, 23);
/// assert_eq!(calculate_proper(date), Some(Proper::Proper2));
/// ```
pub fn calculate_proper(date: Date) -> Option<Proper> {
    let year = date.year();
    let last_sunday = date.sunday_before();
    PROPERS
        .iter()
        // find the first proper that's 3 or fewer days from the Sunday
        // the Sunday will always be closer to another Proper if it's 4 or more days from this one
        // if it's fewer than 3 days, this will always be the closest one
        .find(|(proper_m, proper_d, _)| {
            let proper_date = Date::from_ymd(year, *proper_m, *proper_d);
            let distance = (proper_date - last_sunday).num_days().abs();
            distance <= 3
        })
        .and_then(|(_, _, proper)| *proper)
}
