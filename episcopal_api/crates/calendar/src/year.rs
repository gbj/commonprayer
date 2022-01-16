use serde::{Deserialize, Serialize};

use crate::{Date, LiturgicalWeek};

/// Specifies which type of year another item (such as a lectionary or a liturgy) takes, without
/// providing a concrete year of any kind.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum YearType {
    Rcl,
    DailyOffice,
    None,
}

/// Wraps any of the possible year types, while including a concrete year. Used especially for lectionaries.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Year {
    DailyOffice(DailyOfficeYear),
    Rcl(RCLYear),
    Any,
}

/// The year in the 1979 Book of Common Prayer Daily Office lectionary, following the calculation
/// of [BCP p. 934](https://www.episcopalchurch.org/wp-content/uploads/sites/2/2019/11/bcp_compressed.pdf#page=934)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub enum DailyOfficeYear {
    One,
    Two,
}

impl DailyOfficeYear {
    /// Calculates the year in the Daily Office Lectionary.
    /// ```
    /// # use crate::calendar::{Date, LiturgicalWeek, DailyOfficeYear};
    /// // see BCP p. 934
    /// let christmas_1976 = Date::from_ymd(1976, 12, 25);
    /// assert_eq!(DailyOfficeYear::new(christmas_1976, LiturgicalWeek::Christmas), DailyOfficeYear::One);
    /// let october_2021 = Date::from_ymd(2021, 10, 29);
    /// assert_eq!(DailyOfficeYear::new(october_2021, LiturgicalWeek::Pentecost23), DailyOfficeYear::One);
    /// ```
    pub fn new(date: Date, week: LiturgicalWeek) -> Self {
        let base_year = if is_advent(week) || date.month() == 12 {
            date.year()
        } else {
            date.year() - 1
        };

        if base_year % 2 == 0 {
            DailyOfficeYear::One
        } else {
            DailyOfficeYear::Two
        }
    }
}

/// The year in the Revised Common Lectionary, following the calculation
/// of [BCP p. 888](https://www.episcopalchurch.org/wp-content/uploads/sites/2/2019/11/bcp_compressed.pdf#page=888)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub enum RCLYear {
    A,
    B,
    C,
}

impl RCLYear {
    /// Calculates the year in the Revised Common Lectionary.
    /// ```
    /// # use crate::calendar::{Date, LiturgicalWeek, RCLYear};
    /// let year_a_example = Date::from_ymd(2019, 12, 8);
    /// let year_b_example = Date::from_ymd(2021, 10, 24);
    /// assert_eq!(RCLYear::new(year_a_example, LiturgicalWeek::Advent2), RCLYear::A);
    /// assert_eq!(RCLYear::new(year_b_example, LiturgicalWeek::Pentecost22), RCLYear::B);
    /// ```
    pub fn new(date: Date, week: LiturgicalWeek) -> Self {
        let base_year = if is_advent(week) || date.month() == 12 {
            date.year()
        } else {
            date.year() - 1
        };

        let offset = base_year % 3;

        if offset == 0 {
            RCLYear::A
        } else if offset == 1 {
            RCLYear::B
        } else {
            RCLYear::C
        }
    }
}

fn is_advent(week: LiturgicalWeek) -> bool {
    matches!(
        week,
        LiturgicalWeek::Advent1
            | LiturgicalWeek::Advent2
            | LiturgicalWeek::Advent3
            | LiturgicalWeek::Advent4
    )
}
