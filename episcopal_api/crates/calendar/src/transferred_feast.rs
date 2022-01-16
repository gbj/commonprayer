use std::cmp::Reverse;

use crate::{
    holy_day::HolyDayId, Calendar, Date, Feast, LiturgicalDay, LiturgicalDayId, LiturgicalWeek,
    Rank, Time, Weekday,
};

impl Calendar {
    /// Checks whether any feasts that would have occurred on a Sunday, during Holy Week,
    /// or during the week of Easter should be transferred forward to the next open day,
    /// and therefore override the propers a particular liturgical day:
    /// "Feasts of our Lord, and all other Major Feasts appointed on fixed days
    /// in the Calendar, when they occur on a Sunday, are normally transferred
    /// to the first convenient open day within the week." (BCP p. 16)
    /// ```
    /// # use crate::calendar::{BCP1979_CALENDAR, Date, Feast, LiturgicalDayId, Proper, Weekday};
    /// // e.g., transfer Visitation to Monday when it overlaps with Pentecost the day before
    /// let date = Date::from_ymd(2020, 6, 1);
    /// let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
    /// assert_eq!(original_day.observed, LiturgicalDayId::ProperAndDay(Proper::Proper4, Weekday::Mon));
    /// let transfer = BCP1979_CALENDAR.transferred_feast(&original_day);
    /// assert_eq!(transfer, Some(Feast::TheVisitation));
    ///
    /// ```
    pub fn transferred_feast(&self, day: &LiturgicalDay) -> Option<Feast> {
        let date = day.date;
        let yesterday =
            self.liturgical_day_without_transferred_feasts(date.subtract_days(1), false);

        // All Saints’ Sunday should be marked as a transferred All Saints’, not simply an observed All Saints’
        if date.month() == 11
            && date.weekday() == Weekday::Sun
            && date.day() != 1
            && date.nth_instance_in_month() == 1
        {
            Some(Feast::AllSaintsDay)
        }
        // Christmastide — transfer St. Stephen, St. John, Holy Innocents as necessary
        else if date.month() == 12 && date.day() > 25 && date.day() < 30 {
            let christmas = Date::from_ymd(date.year(), 12, 25);
            // if Christmas is (weekday), and today's date is ___
            match (christmas.weekday(), date.day()) {
                // if Christmas is a Thursday, St. Stephen's Day is Friday,
                // St. John's is Saturday, but Holy Innocents is bumped by the First Sunday
                // after Christmas, so it falls on Monday 12/29
                (Weekday::Thu, 29) => Some(Feast::HolyInnocents),
                // if Christmas is a Friday, St. Stephen's Day is on the Saturday
                // but St. John's is bumped from 12/27 (a Sunday) to Monday
                // and Holy Innocents from 12/28 (the Monday) to Tuesday
                (Weekday::Fri, 28) => Some(Feast::John),
                (Weekday::Fri, 29) => Some(Feast::HolyInnocents),
                // if Christmas is a Saturday, each feast is displaced by a day
                // 12/26 is Sunday after Christmas, so St. Stephen's => 12/27
                // etc.
                (Weekday::Sat, 27) => Some(Feast::Stephen),
                (Weekday::Sat, 28) => Some(Feast::John),
                (Weekday::Sat, 29) => Some(Feast::HolyInnocents),
                // otherwise, there are no transferred feasts this week
                _ => None,
            }
        }
        // no holy days are observed during Holy Week or Easter Week
        else if day.week == LiturgicalWeek::HolyWeek || day.week == LiturgicalWeek::Easter {
            None
        }
        // during the week after the Second Sunday of Easter, we should check for any holy dates
        // that fell during the previous two weeks, and transfer them
        // there can be between 0 and 2 of these, so only apply on Monday and Tuesday
        else if day.week == LiturgicalWeek::Easter2
            && (date.weekday() == Weekday::Mon || date.weekday() == Weekday::Tue)
        {
            let week = day.week;
            let mut dates = (1..=14)
                .flat_map(|delta| {
                    let subtracted_date = date.subtract_days(delta);
                    let month = subtracted_date.month();
                    let day = subtracted_date.day();
                    self.holy_days
                        .iter()
                        .filter_map(move |(id, feast, time, stops_at_sunday)| {
                            if let HolyDayId::Date(s_month, s_day) = id {
                                if *time == Time::AllDay
                                    && month == *s_month
                                    && day == *s_day
                                    && self.feast_day_rank(feast) == Rank::HolyDay
                                    && (stops_at_sunday.is_none()
                                        || stops_at_sunday.unwrap() < week)
                                {
                                    Some((Date::from_ymd(date.year(), month, day), *feast))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<_>>();
            dates.sort_by_key(|d| {
                let date: Date = d.0;
                date
            });

            if date.weekday() == Weekday::Mon {
                dates.get(0).map(|(_, feast)| *feast)
            } else if date.weekday() == Weekday::Tue {
                dates.get(1).map(|(_, feast)| *feast)
            } else {
                None
            }
        }
        // transfer feasts to a Monday, if they fall on the day before (i.e., Sunday)
        else if date.weekday() == Weekday::Mon {
            let mut yesterday_feasts = self
                .holy_days(yesterday.date, yesterday.week, false, true)
                .filter(|feast| {
                    let rank = self.feast_day_rank(feast);
                    rank < Rank::Sunday && rank >= Rank::HolyDay
                })
                .collect::<Vec<_>>();
            yesterday_feasts.sort_by_cached_key(|feast| Reverse(self.feast_day_rank(feast)));
            yesterday_feasts.get(0).copied()
        }
        // transfer feasts to the next day, if the day before was a major feast and today is open
        else if let LiturgicalDayId::Feast(higher_feast) = yesterday.observed {
            if self.feast_day_rank(&higher_feast) > Rank::HolyDay && date.weekday() != Weekday::Sun
            {
                self.holy_days(yesterday.date, yesterday.week, false, true)
                    .find(|feast| self.feast_day_rank(feast) == Rank::HolyDay)
            } else {
                None
            }
        }
        // otherwise, no scenario in which it would be transferred
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::LiturgicalWeek;

    use super::super::{Date, Feast, LiturgicalDayId, Proper, Weekday, BCP1979_CALENDAR};

    #[test]
    fn empty_if_no_feasts() {
        let date = Date::from_ymd(2020, 6, 3);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert_eq!(
            original_day.observed,
            LiturgicalDayId::ProperAndDay(Proper::Proper4, Weekday::Wed)
        );
        let transfer = BCP1979_CALENDAR.transferred_feast(&original_day);
        assert_eq!(transfer, None);
    }

    #[test]
    fn does_not_transfer_observed_feasts_forward() {
        // Transfiguration observed 8/6
        let date = Date::from_ymd(2020, 8, 6);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert_eq!(
            original_day.observed,
            LiturgicalDayId::Feast(Feast::TheTransfiguration)
        );

        // So Transfiguration not transferred to 8/7
        let date = Date::from_ymd(2020, 8, 7);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert_eq!(
            original_day.observed,
            LiturgicalDayId::ProperAndDay(Proper::Proper13, Weekday::Fri)
        );
        let transfer = BCP1979_CALENDAR.transferred_feast(&original_day);
        assert_eq!(transfer, None);
    }

    #[test]
    fn transfers_forward_from_a_sunday() {
        // 4/25 is a Sunday
        let date = Date::from_ymd(2021, 4, 25);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert_eq!(
            original_day.observed,
            LiturgicalDayId::WeekAndDay(LiturgicalWeek::Easter4, Weekday::Sun)
        );

        // So St. Mark transfers to 4/26
        let date = Date::from_ymd(2021, 4, 26);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        let transfer = BCP1979_CALENDAR.transferred_feast(&original_day);
        assert_eq!(transfer, Some(Feast::Mark));
    }

    #[test]
    fn transfers_visitation_forward_from_pentecost() {
        // Pentecost is observed 5/31 (ordinary date for Visitation)
        let date = Date::from_ymd(2020, 5, 31);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert_eq!(
            original_day.observed,
            LiturgicalDayId::Feast(Feast::Pentecost)
        );

        // So Visitation transfers to 6/1
        let date = Date::from_ymd(2020, 6, 1);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        let transfer = BCP1979_CALENDAR.transferred_feast(&original_day);
        assert_eq!(transfer, Some(Feast::TheVisitation));
    }

    #[test]
    fn transfers_holy_day_forward_from_principal_feast() {
        // Ascension Day was on 5/1/2008 (ordinarily the date for Ss Philip and James)
        let date = Date::from_ymd(2008, 5, 1);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert_eq!(
            original_day.observed,
            LiturgicalDayId::Feast(Feast::AscensionDay)
        );

        // So Ss Philip & James transfers to 5/2
        let date = Date::from_ymd(2008, 5, 2);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        let transfer = BCP1979_CALENDAR.transferred_feast(&original_day);
        assert_eq!(transfer, Some(Feast::PhilipAndJames));
    }

    #[test]
    fn does_not_transfer_monday_feasts_observed_on_monday() {
        let date = Date::from_ymd(2020, 8, 24);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        assert_eq!(
            original_day.observed,
            LiturgicalDayId::Feast(Feast::Bartholomew)
        );

        let date = Date::from_ymd(2020, 8, 25);
        let original_day = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date, false);
        let transfer = BCP1979_CALENDAR.transferred_feast(&original_day);
        assert_eq!(transfer, None);
    }

    #[test]
    fn transfers_joseph_and_annunciation() {
        let date_1 = Date::from_ymd(2008, 3, 31);
        let day_1 = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date_1, false);
        let date_2 = Date::from_ymd(2008, 4, 1);
        let day_2 = BCP1979_CALENDAR.liturgical_day_without_transferred_feasts(date_2, false);

        let transfer_1 = BCP1979_CALENDAR.transferred_feast(&day_1);
        let transfer_2 = BCP1979_CALENDAR.transferred_feast(&day_2);

        assert_eq!(transfer_1, Some(Feast::Joseph));
        assert_eq!(transfer_2, Some(Feast::Annunciation));
    }

    #[test]
    fn no_feasts_during_holy_week() {
        let date = Date::from_ymd(2024, 3, 26);
        let day = BCP1979_CALENDAR.liturgical_day(date, false);

        assert_eq!(
            day.observed,
            LiturgicalDayId::Feast(Feast::TuesdayInHolyWeek)
        );
    }

    #[test]
    fn transfers_days_within_christmastide_properly() {
        // in 2021, Christmas is Saturday, which means 12/26 is Christmas 1 (not St. Stephen)
        // this means the whole week of feasts is displaced by a day
        // let's try it out
        let dec_26 = BCP1979_CALENDAR
            .liturgical_day_without_transferred_feasts(Date::from_ymd(2021, 12, 26), false);
        let dec_27 = BCP1979_CALENDAR
            .liturgical_day_without_transferred_feasts(Date::from_ymd(2021, 12, 27), false);
        let dec_28 = BCP1979_CALENDAR
            .liturgical_day_without_transferred_feasts(Date::from_ymd(2021, 12, 28), false);
        let dec_29 = BCP1979_CALENDAR
            .liturgical_day_without_transferred_feasts(Date::from_ymd(2021, 12, 29), false);
        let dec_30 = BCP1979_CALENDAR
            .liturgical_day_without_transferred_feasts(Date::from_ymd(2021, 12, 30), false);
        assert_eq!(BCP1979_CALENDAR.transferred_feast(&dec_30), None);
        assert_eq!(
            BCP1979_CALENDAR.transferred_feast(&dec_29),
            Some(Feast::HolyInnocents)
        );
        assert_eq!(
            BCP1979_CALENDAR.transferred_feast(&dec_28),
            Some(Feast::John)
        );
        assert_eq!(
            BCP1979_CALENDAR.transferred_feast(&dec_27),
            Some(Feast::Stephen)
        );
        assert_eq!(BCP1979_CALENDAR.transferred_feast(&dec_26), None);
        assert_eq!(
            dec_26.observed,
            LiturgicalDayId::WeekAndDay(LiturgicalWeek::Christmas1, Weekday::Sun)
        );
    }

    #[test]
    fn all_saints_sunday() {
        // should transfer to following Sunday
        let all_saints_sunday = BCP1979_CALENDAR.liturgical_day(Date::from_ymd(2021, 11, 7), false);
        assert_eq!(
            all_saints_sunday.observed,
            LiturgicalDayId::TransferredFeast(Feast::AllSaintsDay)
        );

        // but not if Sunday is itself 11/1, All Saints’
        let all_saints_on_sunday = BCP1979_CALENDAR
            .liturgical_day_without_transferred_feasts(Date::from_ymd(2020, 11, 1), false);
        assert_eq!(
            all_saints_on_sunday.observed,
            LiturgicalDayId::Feast(Feast::AllSaintsDay)
        );
    }
}
