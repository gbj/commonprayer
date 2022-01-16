use std::{convert::TryInto, fmt::Display, ops::Sub};

use chrono::{Datelike, NaiveDate};
use language::Language;
use serde::{Deserialize, Deserializer, Serialize};
use thiserror::Error;

use crate::Weekday;

#[derive(Error, Debug)]
pub enum Error {
    #[error("could not parse Date from string")]
    Parse,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub(crate) naive_date: chrono::NaiveDate,
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.naive_date.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let naive_date = chrono::NaiveDate::deserialize(deserializer)?;
        Ok(Self { naive_date })
    }
}

impl Date {
    /// Creates Date from a year, month, and day.
    pub fn from_ymd(year: u16, month: u8, day: u8) -> Date {
        let naive_date = chrono::NaiveDate::from_ymd(year.into(), month.into(), day.into());
        Self { naive_date }
    }

    /// Creates Date from a year, month, and day.
    pub fn parse_from_str(s: &str, fmt: &str) -> Result<Date, Error> {
        let naive_date = chrono::NaiveDate::parse_from_str(s, fmt).map_err(|_| Error::Parse)?;
        Ok(Self { naive_date })
    }

    pub fn year(&self) -> u16 {
        // relatively because we only construct dates with a u16 year
        self.naive_date.year().try_into().unwrap()
    }

    pub fn month(&self) -> u8 {
        // relatively because we only construct dates with a u8 month
        self.naive_date.month().try_into().unwrap()
    }

    pub fn day(&self) -> u8 {
        // relatively because we only construct dates with a u8 day
        self.naive_date.day().try_into().unwrap()
    }

    pub fn weekday(&self) -> Weekday {
        self.naive_date.weekday().into()
    }

    pub fn nth_instance_in_month(&self) -> u8 {
        (self.day() + 7 - 1) / 7
    }

    pub fn add_weeks(&self, weeks: impl Into<i64>) -> Self {
        let naive_date = self.naive_date + chrono::Duration::weeks(weeks.into());
        Self { naive_date }
    }

    pub fn subtract_weeks(&self, weeks: impl Into<i64>) -> Self {
        let naive_date = self.naive_date - chrono::Duration::weeks(weeks.into());
        Self { naive_date }
    }

    pub fn add_days(&self, days: impl Into<i64>) -> Self {
        let naive_date = self.naive_date + chrono::Duration::days(days.into());
        Self { naive_date }
    }

    pub fn subtract_days(&self, days: impl Into<i64>) -> Self {
        let naive_date = self.naive_date - chrono::Duration::days(days.into());
        Self { naive_date }
    }

    pub fn day_in_year(&self) -> u16 {
        let first_day_of_year = NaiveDate::from_ymd(self.year().into(), 1, 1);
        let difference = self.naive_date - first_day_of_year;
        difference.num_days().try_into().unwrap_or(0)
    }

    /// Calculates the [Date](Date) of the Sunday before the given date.
    /// ```
    /// # use calendar::Date;
    /// let test_1 = Date::from_ymd(2020, 5, 21).sunday_before();
    /// assert_eq!(test_1.month(), 5);
    /// assert_eq!(test_1.day(), 17);
    /// // Wraps to previous month
    /// let test_2 = Date::from_ymd(2020, 4, 1).sunday_before();
    /// assert_eq!(test_2.month(), 3);
    /// assert_eq!(test_2.day(), 29);
    /// // Wraps to previous year
    /// let test_3 = Date::from_ymd(2020, 1, 4).sunday_before();
    /// assert_eq!(test_3.month(), 12);
    /// assert_eq!(test_3.day(), 29);
    /// // On Sundays, returns the same day
    /// let test_4 = Date::from_ymd(2021, 10, 3).sunday_before();
    /// assert_eq!(test_4.month(), 10);
    /// assert_eq!(test_4.day(), 3);
    /// ```
    pub fn sunday_before(&self) -> Date {
        let date = self.naive_date;
        let nth_weekday_from_sunday = date.weekday().num_days_from_sunday();
        let naive_date = date - chrono::Duration::days(nth_weekday_from_sunday.into());
        naive_date.into()
    }

    pub fn to_localized_name(&self, _language: Language) -> String {
        // TODO i18n
        let month = match self.month() {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "",
        };
        format!("{} {}, {}", month, self.day(), self.year())
    }

    pub fn to_padded_string(&self) -> String {
        format!("{}-{:02}-{:02}", self.year(), self.month(), self.day())
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.year(), self.month(), self.day())
    }
}

pub type Duration = chrono::Duration;

impl From<chrono::NaiveDate> for Date {
    fn from(naive_date: chrono::NaiveDate) -> Self {
        Self { naive_date }
    }
}

impl From<Date> for chrono::NaiveDate {
    fn from(date: Date) -> Self {
        date.naive_date
    }
}

impl Sub for Date {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.naive_date - rhs.naive_date
    }
}
