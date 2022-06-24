use std::{cmp::Reverse, convert::TryInto};

use itertools::Itertools;
use language::Language;
use status::Status;

use crate::{
    easter_in_year, feasts::KalendarEntry, holy_day::HolyDayId, liturgical_day::LiturgicalDayId,
    liturgical_week::Cycle, propers::calculate_proper, DailyOfficeYear, Date, Feast, LiturgicalDay,
    LiturgicalWeek, Proper, RCLYear, Rank, Season, Time, Weekday,
};

/// The settings for a particular calendar. Different calendars vary slightly
/// in the way their liturgical cycles are set up relative to Christmas and Easter.
/// Based on this structure, we can generate a [LiturgicalWeek](LiturgicalWeek)
/// and [LiturgicalDay](LiturgicalDay) from any date.
pub struct Calendar {
    /// How many weeks before Easter the Easter cycle begins in the calendar
    pub easter_cycle_begins: u8,
    /// How many weeks before Christmas the Christmas cycle begins in the calendar
    pub christmas_cycle_begins: u8,
    /// Whether to use `Proper 1`, `Proper 2`, etc. for weeks after Pentecost
    pub has_propers: bool,
    /// Maps nth week of cycle onto the liturgical week identifier
    /// i.e., in the Episcopal Church calendar the 1st week of the Christmas cycle is Christ the King
    // TODO: benchmark against a HashMap — I'm making the assumption that O(n) for small n is better than O(1) given the cost of hashing
    pub weeks: &'static [(Cycle, u8, LiturgicalWeek)],
    /// All holy days in the calendar
    pub holy_days: &'static [KalendarEntry],
    /// A calendar that can be used to look up information for feasts that are not found in this calendar
    /// (used for e.g., days that appear in the BCP calendar but don't have separate LFF entries)
    pub holy_days_fallback: Option<&'static Calendar>,
    /// Ranks of holy days in this calendar
    pub holy_day_ranks: &'static [(Feast, Rank)],
    /// Associations between [Feast]s and [Season]s
    pub feast_seasons: &'static [(Feast, Season)],
    /// Associations between [LiturgicalWeek]s and [Season]s
    pub week_seasons: &'static [(LiturgicalWeek, Season)],
    /// Name for each [Feast](crate::Feast), by language
    pub feast_names: &'static [(Feast, Language, &'static str, Status)],
    /// Name for each [LiturgicalWeek](crate::LiturgicalWeek), by language
    pub week_names: &'static [(LiturgicalWeek, Language, &'static str)],
    /// Name for each [Proper](crate::Proper), by language
    pub proper_names: &'static [(Proper, Language, &'static str)],
    /// Alternative services for certain major days
    pub major_day_alternatives: &'static [(Feast, &'static [Feast])],
}

impl Calendar {
    /// The [LiturgicalDay](LiturgicalDay) that is observed on a given date,
    /// including any feasts or special observances and any transferred feasts.
    /// ```
    /// # use crate::calendar::{BCP1979_CALENDAR, Date, Weekday, LiturgicalWeek, DailyOfficeYear, RCLYear, Feast, LiturgicalDayId};
    /// let date = Date::from_ymd(2020, 5, 21);
    /// let thursday_easter_6 = BCP1979_CALENDAR.liturgical_day(date, false);
    /// assert_eq!(thursday_easter_6.week, LiturgicalWeek::Easter6);
    /// assert_eq!(thursday_easter_6.weekday, Weekday::Thu);
    /// assert_eq!(thursday_easter_6.daily_office_year, DailyOfficeYear::Two);
    /// assert_eq!(thursday_easter_6.rcl_year, RCLYear::A);
    /// assert_eq!(thursday_easter_6.holy_days, vec![]);
    /// assert_eq!(thursday_easter_6.proper, None);
    /// assert_eq!(thursday_easter_6.observed, LiturgicalDayId::Feast(Feast::AscensionDay));
    /// ```
    pub fn liturgical_day(&self, date: Date, evening: bool) -> LiturgicalDay {
        let mut original = self.liturgical_day_without_transferred_feasts(date, evening);

        // modify if feasts should be transferred
        let transferred = self.transferred_feast(&original);
        if let Some(transferred) = transferred {
            let alternate = std::mem::replace(
                &mut original.observed,
                LiturgicalDayId::TransferredFeast(transferred),
            );
            original.alternate = Some(alternate);
        }

        // remove certain feasts from from holy_days
        // - the actual feast being observed
        // - the alternate
        // - black-letter days if it's a Sunday or a holy day
        let observed = original.observed;
        let weekday = original.weekday;
        original.holy_days.retain(|feast| {
            let feast_rank = self.feast_day_rank(feast);
            match observed {
                LiturgicalDayId::Feast(o_feast) => {
                    o_feast != *feast
                        && (feast_rank >= Rank::PrecedenceOverWeekday
                            || feast_rank == Rank::EmberDay)
                }
                LiturgicalDayId::TransferredFeast(t_feast) => {
                    // to be honest it's a little unclear whether a transferred red-letter day should
                    // cause the original black-letter day to be ignored, or also commemorated
                    // but we'll drop it, which I think is the most reasonable option
                    t_feast != *feast
                        && (feast_rank >= Rank::PrecedenceOverWeekday
                            || feast_rank == Rank::EmberDay)
                }
                _ => weekday != Weekday::Sun,
            }
        });

        original
    }

    /// The [LiturgicalDay](LiturgicalDay) that is observed on a given date,
    /// without transferring any feasts.
    /// ```
    /// # use crate::calendar::{BCP1979_CALENDAR, Date, Weekday, LiturgicalWeek, DailyOfficeYear, RCLYear, Feast, LiturgicalDayId};
    /// let date = Date::from_ymd(2020, 5, 21);
    /// let thursday_easter_6 = BCP1979_CALENDAR.liturgical_day(date, false);
    /// assert_eq!(thursday_easter_6.week, LiturgicalWeek::Easter6);
    /// assert_eq!(thursday_easter_6.weekday, Weekday::Thu);
    /// assert_eq!(thursday_easter_6.daily_office_year, DailyOfficeYear::Two);
    /// assert_eq!(thursday_easter_6.rcl_year, RCLYear::A);
    /// assert_eq!(thursday_easter_6.holy_days, vec![]);
    /// assert_eq!(thursday_easter_6.proper, None);
    /// assert_eq!(thursday_easter_6.observed, LiturgicalDayId::Feast(Feast::AscensionDay));
    /// ```
    pub fn liturgical_day_without_transferred_feasts(
        &self,
        date: Date,
        evening: bool,
    ) -> LiturgicalDay {
        let weekday = date.weekday();
        let week = self.liturgical_week(date);
        let proper = self.proper(date, week);
        let holy_days = self.holy_days(date, week, evening, false);
        let fallback_holy_days = self.holy_days_fallback.map(|fallback| {
            Calendar::filter_holy_days(date, week, evening, false, fallback.holy_days)
        });
        let holy_days = if let Some(fallback) = fallback_holy_days {
            holy_days.chain(fallback).unique().collect::<Vec<_>>()
        } else {
            holy_days.collect::<Vec<_>>()
        };
        let (observed, alternate) = self.observed_day(week, proper, weekday, &holy_days);
        let alternative_services = self
            .major_day_alternatives
            .iter()
            .find(|(s_feast, _)| observed == LiturgicalDayId::Feast(*s_feast))
            .map(|(_, alternatives)| alternatives.to_vec())
            .unwrap_or_default();

        LiturgicalDay {
            date,
            evening,
            week,
            weekday,
            daily_office_year: DailyOfficeYear::new(date, week),
            rcl_year: RCLYear::new(date, week),
            holy_days,
            proper,
            observed,
            alternate,
            alternative_services,
        }
    }

    /// Gives the appropriate liturgical [Season] for the given day
    /// ```
    /// # use crate::calendar::{BCP1979_CALENDAR, Date, Season};
    /// let date = Date::from_ymd(2021, 11, 29);
    /// let monday_advent_1 = BCP1979_CALENDAR.liturgical_day(date, false);
    /// assert_eq!(BCP1979_CALENDAR.season(&monday_advent_1), Season::Advent);
    /// let date = Date::from_ymd(2020, 5, 21);
    /// let thursday_easter_6 = BCP1979_CALENDAR.liturgical_day(date, false);
    /// assert_eq!(BCP1979_CALENDAR.season(&thursday_easter_6), Season::Ascension);
    /// ```
    pub fn season(&self, day: &LiturgicalDay) -> Season {
        match day.observed {
            LiturgicalDayId::Feast(feast) => self
                .feast_seasons
                .iter()
                .find(|(search, _)| *search == feast)
                .map(|(_, season)| *season)
                .unwrap_or(Season::Saints),
            LiturgicalDayId::TransferredFeast(feast) => self
                .feast_seasons
                .iter()
                .find(|(search, _)| *search == feast)
                .map(|(_, season)| *season)
                .unwrap_or(Season::Saints),
            _ => self
                .week_seasons
                .iter()
                .find(|(search, _)| *search == day.week)
                .map(|(_, season)| *season)
                .unwrap_or(Season::OrdinaryTime),
        }
    }

    /// Gives the [Season] of the week during which a day falls
    /// ```
    /// # use crate::calendar::{BCP1979_CALENDAR, Date, Season};
    /// let date = Date::from_ymd(2020, 3, 25);
    /// let annunciation = BCP1979_CALENDAR.liturgical_day(date, false);
    /// assert_eq!(BCP1979_CALENDAR.season(&annunciation), Season::Saints);
    /// assert_eq!(BCP1979_CALENDAR.base_season(&annunciation), Season::Lent);
    /// let date = Date::from_ymd(2020, 12, 25);
    /// let christmas = BCP1979_CALENDAR.liturgical_day(date, false);
    /// assert_eq!(BCP1979_CALENDAR.season(&christmas), Season::Christmas);
    /// assert_eq!(BCP1979_CALENDAR.base_season(&christmas), Season::Christmas);
    /// ```
    pub fn base_season(&self, day: &LiturgicalDay) -> Season {
        self.week_seasons
            .iter()
            .find(|(search, _)| *search == day.week)
            .map(|(_, season)| *season)
            .unwrap_or(Season::OrdinaryTime)
    }

    // Gives the appropriate [Rank] for the given day
    /// ```
    /// # use crate::calendar::{BCP1979_CALENDAR, Date, Rank};
    /// let date = Date::from_ymd(2021, 11, 29);
    /// let monday_advent_1 = BCP1979_CALENDAR.liturgical_day(date, false);
    /// assert_eq!(BCP1979_CALENDAR.rank(&monday_advent_1), Rank::FerialWeekday);
    /// let date = Date::from_ymd(2020, 5, 21);
    /// let thursday_easter_6 = BCP1979_CALENDAR.liturgical_day(date, false);
    /// assert_eq!(BCP1979_CALENDAR.rank(&thursday_easter_6), Rank::PrincipalFeast);
    /// ```
    pub fn rank(&self, day: &LiturgicalDay) -> Rank {
        match day.observed {
            LiturgicalDayId::Feast(feast) => self.feast_day_rank(&feast),
            LiturgicalDayId::TransferredFeast(feast) => self.feast_day_rank(&feast),
            _ => {
                if day.weekday == Weekday::Sun {
                    Rank::Sunday
                } else if !day.holy_days.is_empty() {
                    Rank::OptionalObservance
                } else {
                    Rank::FerialWeekday
                }
            }
        }
    }

    /// The name of a [Feast](crate::Feast) in a given [Language](language::Language)
    pub fn feast_name(&self, feast: Feast, language: Language) -> Option<String> {
        if let Some(fallback) = self.holy_days_fallback {
            self.feast_names
                .iter()
                .chain(fallback.feast_names.iter())
                .find(|(s_feast, s_language, _, _)| *s_feast == feast && *s_language == language)
                .map(|(_, _, name, status)| {
                    if status == &Status::TrialUse {
                        format!("[{name}]")
                    } else {
                        name.to_string()
                    }
                })
        } else {
            self.feast_names
                .iter()
                .find(|(s_feast, s_language, _, _)| *s_feast == feast && *s_language == language)
                .map(|(_, _, name, status)| {
                    if status == &Status::TrialUse {
                        format!("[{name}]")
                    } else {
                        name.to_string()
                    }
                })
        }
    }

    /// The name of a [LiturgicalWeek](crate::LiturgicalWeek) in a given [Language](language::Language)
    pub fn week_name(&self, week: LiturgicalWeek, language: Language) -> Option<&str> {
        self.week_names
            .iter()
            .find(|(s_week, s_language, _)| *s_week == week && *s_language == language)
            .map(|(_, _, name)| *name)
    }

    /// The name of a [Proper](crate::Proper) in a given [Language](language::Language)
    pub fn proper_name(&self, proper: Proper, language: Language) -> Option<&str> {
        self.proper_names
            .iter()
            .find(|(s_proper, s_language, _)| *s_proper == proper && *s_language == language)
            .map(|(_, _, name)| *name)
    }

    /// The [LiturgicalWeek](LiturgicalWeek) within which a given date falls,
    /// ignoring any feasts or special observances.
    fn liturgical_week(&self, date: Date) -> LiturgicalWeek {
        let index = self.liturgical_week_index(date);
        self.weeks
            .iter()
            .find(|(cycle, offset, _)| index.cycle == *cycle && index.week == *offset)
            .map(|(_, _, week)| *week)
            .unwrap_or(LiturgicalWeek::None)
    }

    /// For calendars that use the Proper ____ system, gives the [Proper](Proper)
    /// within which the date falls, if any.
    fn proper(&self, date: Date, week: LiturgicalWeek) -> Option<Proper> {
        if self.has_propers && week >= LiturgicalWeek::Pentecost {
            calculate_proper(date)
        } else {
            None
        }
    }

    /// The rank of the given feast day in this calendar
    pub fn feast_day_rank(&self, feast: &Feast) -> Rank {
        if let Some(fallback) = self.holy_days_fallback {
            self.holy_day_ranks
                .iter()
                .chain(fallback.holy_day_ranks.iter())
                .find(|(search_feast, _)| search_feast == feast)
                .map(|(_, rank)| *rank)
                .unwrap_or(Rank::OptionalObservance)
        } else {
            self.holy_day_ranks
                .iter()
                .find(|(search_feast, _)| search_feast == feast)
                .map(|(_, rank)| *rank)
                .unwrap_or(Rank::OptionalObservance)
        }
    }

    /// Whether the given feast is the "Eve of ___"
    pub fn feast_is_eve(&self, feast: &Feast) -> bool {
        let in_own_calendar = self
            .holy_days
            .iter()
            .find(|(_, search_feast, _, _)| search_feast == feast)
            .map(|(_, _, time, _)| matches!(*time, Time::EveningOnly(_)));
        let in_fallback = self
            .holy_days_fallback
            .and_then(|fallback| {
                fallback
                    .holy_days
                    .iter()
                    .find(|(_, search_feast, _, _)| search_feast == feast)
            })
            .map(|(_, _, time, _)| matches!(*time, Time::EveningOnly(_)));
        in_own_calendar.unwrap_or_else(|| in_fallback.unwrap_or(false))
    }

    /// If the feast is the Eve of ___, returns Some(___); otherwise, None
    pub fn feast_eve_following_day(&self, feast: &Feast) -> Option<Feast> {
        let in_own_calendar = self
            .holy_days
            .iter()
            .find(|(_, search_feast, _, _)| search_feast == feast)
            .and_then(|(_, _, time, _)| match time {
                Time::EveningOnly(next_day) => next_day.as_ref(),
                _ => None,
            });
        let in_fallback = self.holy_days_fallback.and_then(|fallback| {
            fallback
                .holy_days
                .iter()
                .find(|(_, search_feast, _, _)| search_feast == feast)
                .and_then(|(_, _, time, _)| match time {
                    Time::EveningOnly(next_day) => next_day.as_ref(),
                    _ => None,
                })
        });
        match (in_own_calendar, in_fallback) {
            (Some(f), _) => Some(*f),
            (None, Some(f)) => Some(*f),
            (None, None) => None,
        }
    }

    pub(crate) fn holy_days(
        &self,
        date: Date,
        week: LiturgicalWeek,
        evening: bool,
        ignore_evening: bool,
    ) -> impl Iterator<Item = Feast> {
        Calendar::filter_holy_days(date, week, evening, ignore_evening, self.holy_days)
    }

    fn filter_holy_days(
        date: Date,
        week: LiturgicalWeek,
        evening: bool,
        ignore_evening: bool,
        holy_days: &[KalendarEntry],
    ) -> impl Iterator<Item = Feast> + '_ {
        let today_month = date.month();
        let today_day = date.day();
        let today_weekday = date.weekday();
        let today_year = date.year();
        holy_days
            .iter()
            .filter_map(move |(id, feast, f_time, f_stops_at_sunday)| {
                let has_stopped = if let Some(stopping_week) = f_stops_at_sunday {
                    week >= *stopping_week
                } else {
                    false
                };
                let time_ok = (!matches!(*f_time, Time::EveningOnly(_))
                    || (!ignore_evening && matches!(*f_time, Time::EveningOnly(_)) && evening))
                    && (*f_time != Time::MorningOnly || !evening);
                match id {
                    HolyDayId::Date(f_month, f_day) => {
                        if *f_month == today_month && *f_day == today_day && time_ok && !has_stopped
                        {
                            Some(*feast)
                        } else {
                            None
                        }
                    }
                    HolyDayId::SpecialDay(f_week, f_weekday) => {
                        if *f_week == week && *f_weekday == today_weekday && time_ok && !has_stopped
                        {
                            Some(*feast)
                        } else {
                            None
                        }
                    }
                    HolyDayId::DayOfMonth { month, week, day } => {
                        // divide date by 7 and round up => nth instance of a day of week
                        if *month == today_month
                            && *day == today_weekday
                            && date.nth_instance_in_month() == *week
                            && time_ok
                            && !has_stopped
                        {
                            Some(*feast)
                        } else {
                            None
                        }
                    }
                    HolyDayId::WeekdayAfterDate {
                        month,
                        day,
                        starting_weekday,
                        weekday,
                    } => {
                        let starting_date = Date::from_ymd(today_year, *month, *day);
                        let starting_date = if let Some(starting_weekday) = starting_weekday {
                            if starting_date.weekday() == *starting_weekday {
                                starting_date.add_days(7)
                            } else {
                                starting_date
                            }
                        } else {
                            starting_date
                        };

                        let distance_in_days = (date - starting_date).num_days();

                        if (0..7).contains(&distance_in_days) && weekday == &date.weekday() {
                            Some(*feast)
                        } else {
                            None
                        }
                    }
                }
            })
    }

    fn observed_day(
        &self,
        week: LiturgicalWeek,
        proper: Option<Proper>,
        weekday: Weekday,
        holy_days: &[Feast],
    ) -> (LiturgicalDayId, Option<LiturgicalDayId>) {
        if holy_days.is_empty() {
            (
                self.observed_day_from_week_or_proper(week, proper, weekday),
                None,
            )
        } else {
            // include all eligible feasts
            let mut observable_feasts = holy_days
                .iter()
                .filter(|feast| {
                    let rank = self.feast_day_rank(feast);
                    // only include if rank is higher than a black-letter day
                    rank >= Rank::PrecedenceOverWeekday
                    // if, if today is a Sunday, if rank is above a Sunday
                    // Sundays trump e.g., red-letter saints’ days
                        && (weekday != Weekday::Sun || rank >= Rank::Sunday)
                })
                .collect::<Vec<_>>();

            // sort in reverse order, i.e., from highest-ranking feast to lowest
            observable_feasts.sort_by_cached_key(|feast| Reverse(self.feast_day_rank(feast)));
            let highest_ranking_feast = observable_feasts.get(0);

            if let Some(highest_ranking_feast) = highest_ranking_feast {
                if weekday == Weekday::Sun
                    && self.feast_day_rank(highest_ranking_feast) > Rank::Sunday
                {
                    (
                        LiturgicalDayId::Feast(**highest_ranking_feast),
                        Some(self.observed_day_from_week_or_proper(week, proper, weekday)),
                    )
                } else {
                    (
                        LiturgicalDayId::Feast(**highest_ranking_feast),
                        observable_feasts
                            .iter()
                            .filter(|feast| self.feast_day_rank(feast) >= Rank::SpecialDevotion)
                            .nth(1)
                            .copied()
                            .copied()
                            .map(LiturgicalDayId::Feast),
                    )
                }
            }
            // Holy days that fall on a Sunday in ordinary time
            // can be used as alternatives to the Sunday propers,
            // but aren't by default
            else if weekday == Weekday::Sun
                && ((week > LiturgicalWeek::TrinitySunday && week < LiturgicalWeek::LastPentecost)
                    || (week > LiturgicalWeek::Epiphany1 && week < LiturgicalWeek::LastEpiphany))
            {
                let alternate_feast = holy_days
                    .iter()
                    .find(|feast| self.feast_day_rank(feast) > Rank::OptionalObservance)
                    .copied()
                    .map(LiturgicalDayId::Feast);

                (
                    self.observed_day_from_week_or_proper(week, proper, weekday),
                    alternate_feast,
                )
            } else {
                (
                    self.observed_day_from_week_or_proper(week, proper, weekday),
                    None,
                )
            }
        }
    }

    fn observed_day_from_week_or_proper(
        &self,
        week: LiturgicalWeek,
        proper: Option<Proper>,
        weekday: Weekday,
    ) -> LiturgicalDayId {
        if let Some(proper) = proper {
            LiturgicalDayId::ProperAndDay(proper, weekday)
        } else {
            LiturgicalDayId::WeekAndDay(week, weekday)
        }
    }

    fn liturgical_week_index(&self, date: Date) -> LiturgicalWeekIndex {
        let year = date.year();
        let easter = easter_in_year(year.into());
        let christmas_eve = Date::from_ymd(year, 12, 24);
        let last_epiphany = easter.subtract_weeks(self.easter_cycle_begins);
        let fourth_advent = christmas_eve.sunday_before();
        let last_pentecost = fourth_advent
            .sunday_before()
            .subtract_weeks(self.christmas_cycle_begins);
        if date >= last_pentecost || date < last_epiphany {
            self.christmas_cycle_week(date)
        } else {
            self.easter_cycle_week(date, easter)
        }
    }

    fn christmas_cycle_week(&self, date: Date) -> LiturgicalWeekIndex {
        // year of Christmas is either the current year or, if we're in January/February, the previous year
        let christmas_year = if date.month() >= 10 {
            date.year()
        } else {
            date.year() - 1
        };

        // date of Christmas in this liturgical year
        let christmas = Date::from_ymd(christmas_year, 12, 25);
        let christmas_eve = Date::from_ymd(christmas_year, 12, 24);
        let epiphany = Date::from_ymd(christmas_year + 1, 1, 6);

        // If in Advent...
        if date <= christmas_eve {
            let advent_4 = christmas_eve.sunday_before();
            let weeks_from_advent_4 = date.sunday_before() - advent_4;
            let week = weeks_from_advent_4.num_weeks() + 4;
            LiturgicalWeekIndex {
                cycle: Cycle::Advent,
                week: week.try_into().unwrap(),
            }
        }
        // Christmas
        else if date < epiphany {
            let week = date - christmas.sunday_before();
            LiturgicalWeekIndex {
                cycle: Cycle::Christmas,
                week: week.num_weeks().try_into().unwrap(),
            }
        }
        // Epiphany
        else {
            let week = date - epiphany.sunday_before();
            LiturgicalWeekIndex {
                cycle: Cycle::Epiphany,
                week: week.num_weeks().try_into().unwrap(),
            }
        }
    }

    fn easter_cycle_week(&self, date: Date, easter: Date) -> LiturgicalWeekIndex {
        let num_weeks = (date - easter).num_weeks();

        let weeks_from_easter: i64 = if date == easter {
            0
        } else if date < easter && date.weekday() != Weekday::Sun {
            num_weeks - 1
        } else {
            num_weeks
        };

        let week = weeks_from_easter + self.easter_cycle_begins as i64;

        LiturgicalWeekIndex {
            cycle: Cycle::Easter,
            week: week.try_into().unwrap(),
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
struct LiturgicalWeekIndex {
    cycle: Cycle,
    week: u8,
}

#[cfg(test)]
mod tests {
    use crate::BCP1979_CALENDAR;

    use super::*;

    #[test]
    fn easter_cycle_dates() {
        let easter = Date::from_ymd(2022, 4, 17);
        let monday_lent_1 = Date::from_ymd(2022, 3, 7);
        let monday_holy_week = Date::from_ymd(2022, 4, 11);
        let monday_easter_week = Date::from_ymd(2022, 4, 19);
        let monday_easter_2 = Date::from_ymd(2022, 4, 26);
        assert_eq!(
            BCP1979_CALENDAR.easter_cycle_week(monday_lent_1, easter),
            LiturgicalWeekIndex {
                cycle: Cycle::Easter,
                week: 1
            }
        );
        assert_eq!(
            BCP1979_CALENDAR.easter_cycle_week(monday_holy_week, easter),
            LiturgicalWeekIndex {
                cycle: Cycle::Easter,
                week: 6
            }
        );
        assert_eq!(
            BCP1979_CALENDAR.easter_cycle_week(easter, easter),
            LiturgicalWeekIndex {
                cycle: Cycle::Easter,
                week: 7
            }
        );
        assert_eq!(
            BCP1979_CALENDAR.easter_cycle_week(monday_easter_week, easter),
            LiturgicalWeekIndex {
                cycle: Cycle::Easter,
                week: 7
            }
        );
        assert_eq!(
            BCP1979_CALENDAR.easter_cycle_week(monday_easter_2, easter),
            LiturgicalWeekIndex {
                cycle: Cycle::Easter,
                week: 8
            }
        );
    }

    #[test]
    fn should_not_override_principal_feasts() {
        let date = Date::from_ymd(2020, 5, 31);
        let day = BCP1979_CALENDAR.liturgical_day(date, false);
        assert_eq!(day.observed, LiturgicalDayId::Feast(Feast::Pentecost));
    }

    #[test]
    fn should_not_override_sundays() {
        let date = Date::from_ymd(2020, 10, 18);
        let day = BCP1979_CALENDAR.liturgical_day(date, false);
        assert_eq!(
            day.observed,
            LiturgicalDayId::ProperAndDay(Proper::Proper24, Weekday::Sun)
        );
    }

    #[test]
    fn should_not_observe_eve_if_not_evening() {
        let date = Date::from_ymd(2018, 5, 19);
        let day = BCP1979_CALENDAR.liturgical_day(date, false);
        assert_ne!(day.observed, LiturgicalDayId::Feast(Feast::EveOfPentecost));

        let day = BCP1979_CALENDAR.liturgical_day(date, true);
        assert_eq!(day.observed, LiturgicalDayId::Feast(Feast::EveOfPentecost));
    }

    #[test]
    fn appropriately_handle_ember_days() {
        let date = Date::from_ymd(2021, 2, 24);
        let day = BCP1979_CALENDAR.liturgical_day(date, false);
        assert!(day.holy_days.contains(&Feast::EmberDay));

        let date = Date::from_ymd(2021, 9, 15);
        let day = BCP1979_CALENDAR.liturgical_day(date, false);
        assert!(day.holy_days.contains(&Feast::EmberDay));

        let date = Date::from_ymd(2021, 9, 14);
        let day = BCP1979_CALENDAR.liturgical_day(date, false);
        assert!(!day.holy_days.contains(&Feast::EmberDay));

        let date = Date::from_ymd(2022, 9, 21);
        let day = BCP1979_CALENDAR.liturgical_day(date, false);
        assert!(day.holy_days.contains(&Feast::EmberDay));
    }
}
