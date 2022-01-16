#![feature(test)]

use std::convert::TryInto;

mod bcp1979;
mod calendar;
mod date;
pub mod feasts;
mod holy_day;
pub mod lff2018;
mod liturgical_color;
mod liturgical_day;
mod liturgical_week;
pub mod propers;
mod rank;
mod sanctoral;
mod season;
mod transferred_feast;
mod weekday;
mod year;
pub use self::calendar::Calendar;
pub use bcp1979::BCP1979_CALENDAR;
pub use date::Date;
pub use feasts::{Feast, Time};
pub use holy_day::{HolyDay, HolyDayId};
pub use lff2018::LFF2018_CALENDAR;
pub use liturgical_color::Color;
pub use liturgical_day::{LiturgicalDay, LiturgicalDayId};
pub use liturgical_week::{Cycle, LiturgicalWeek};
pub use propers::Proper;
pub use rank::Rank;
pub use sanctoral::Sanctoral;
pub use season::Season;
pub use weekday::Weekday;
pub use year::{DailyOfficeYear, RCLYear, Year, YearType};

/// Calculates the date of Easter as a [Date](Date) in any given year.
/// ```
/// # use calendar::easter_in_year;
/// use chrono::Datelike;
/// let year1 = 2020;
/// let year2 = 1983; // April 3
/// let year3 = 2027; // March 28
/// // Easter 2020: April 12
/// assert_eq!(easter_in_year(2020).month(), 4);
/// assert_eq!(easter_in_year(2020).day(), 12);
/// // Easter 1983: April 3
/// assert_eq!(easter_in_year(1983).month(), 4);
/// assert_eq!(easter_in_year(1983).day(), 3);
/// // Easter 2027: March 28
/// assert_eq!(easter_in_year(2027).month(), 3);
/// assert_eq!(easter_in_year(2027).day(), 28);
/// ```
#[allow(clippy::many_single_char_names)]
pub fn easter_in_year(year: u32) -> Date {
    // Computus: Meeus/Jones/Butcher algorithm
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = ((h + l - 7 * m + 114) % 31) + 1;
    Date::from_ymd(
        year.try_into().unwrap(),
        month.try_into().unwrap(),
        day.try_into().unwrap(),
    )
}

// Crate-wide benchmark
extern crate test;
#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    use crate::{Date, BCP1979_CALENDAR};

    #[bench]
    fn bench_liturgical_day(b: &mut Bencher) {
        b.iter(|| {
            for m in 1..=12 {
                for d in 1..=28 {
                    black_box(BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2021, m, d), false));
                }
            }
        });
    }
}
