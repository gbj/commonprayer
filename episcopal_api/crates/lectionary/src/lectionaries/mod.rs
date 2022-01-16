mod bcp1979_30_day_psalter;
mod bcp1979_daily_office_psalter;
mod bcp1979_office;
mod lff2018;
mod rcl;
mod rcl1;
mod rcl2;

pub use bcp1979_30_day_psalter::BCP1979_30_DAY_PSALTER;
pub use bcp1979_daily_office_psalter::BCP1979_DAILY_OFFICE_PSALTER;
pub use bcp1979_office::BCP1979_DAILY_OFFICE_LECTIONARY;
pub use lff2018::LFF2018_LECTIONARY;
pub use rcl::{rcl_readings, RCLTrack, RCL};
pub use rcl1::RCL_TRACK_1;
pub use rcl2::RCL_TRACK_2;

#[cfg(test)]
mod tests {
    use calendar::{Date, LiturgicalDayId, LiturgicalWeek, Rank, Weekday, BCP1979_CALENDAR};

    use crate::{rcl_readings, RCLTrack};

    #[test]
    fn rcl_readings_for_every_sunday() {
        for year in 2018..=2021 {
            for month in 1..=12 {
                for day in 1..=28 {
                    let date = Date::from_ymd(year, month, day);
                    let liturgical_day = BCP1979_CALENDAR.liturgical_day(date, false);

                    if date.weekday() == Weekday::Sun
                        && (matches!(liturgical_day.observed, LiturgicalDayId::WeekAndDay(_, _))
                            || matches!(
                                liturgical_day.observed,
                                LiturgicalDayId::ProperAndDay(_, _)
                            ))
                    {
                        assert!(
                            rcl_readings(&liturgical_day.observed, &liturgical_day, RCLTrack::One)
                                .count()
                                >= 4
                        );
                        assert!(
                            rcl_readings(&liturgical_day.observed, &liturgical_day, RCLTrack::Two)
                                .count()
                                >= 4
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn rcl_readings_for_major_feasts() {
        for year in 2018..=2021 {
            for month in 1..=12 {
                for day in 1..=28 {
                    let date = Date::from_ymd(year, month, day);
                    let liturgical_day = BCP1979_CALENDAR.liturgical_day(date, false);

                    let feast = match liturgical_day.observed {
                        LiturgicalDayId::Feast(f) => Some(f),
                        LiturgicalDayId::TransferredFeast(f) => Some(f),
                        _ => None,
                    };
                    if let Some(feast) = feast {
                        if !(BCP1979_CALENDAR.feast_is_eve(&feast)
                            || BCP1979_CALENDAR.feast_day_rank(&feast) < Rank::HolyDay
                            || liturgical_day.week == LiturgicalWeek::Easter
                                && date.weekday() != Weekday::Sun
                            || liturgical_day.week == LiturgicalWeek::Easter6
                                && date.weekday() != Weekday::Thu)
                        {
                            assert!(
                                rcl_readings(
                                    &liturgical_day.observed,
                                    &liturgical_day,
                                    RCLTrack::One,
                                )
                                .count()
                                    >= 4
                            );
                        }
                    }
                    if date.weekday() == Weekday::Sun
                        && (matches!(liturgical_day.observed, LiturgicalDayId::WeekAndDay(_, _))
                            || matches!(
                                liturgical_day.observed,
                                LiturgicalDayId::ProperAndDay(_, _)
                            ))
                    {}
                }
            }
        }
    }
}
