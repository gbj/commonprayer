use api::summary::{DailySummary, ObservanceSummary, PartialDailySummary};
use calendar::{Calendar, Date, LiturgicalDay, LiturgicalDayId, Weekday, BCP1979_CALENDAR};
use liturgy::Psalm;
use psalter::{bcp1979::BCP1979_PSALTER, Psalter};

use language::Language;
use lectionary::{
    Lectionary, ReadingType, BCP1979_30_DAY_PSALTER, BCP1979_DAILY_OFFICE_LECTIONARY,
    BCP1979_DAILY_OFFICE_PSALTER,
};

use crate::CommonPrayer;

impl CommonPrayer {
    pub fn summarize_date(date: &Date, language: Language) -> DailySummary {
        let calendar = &BCP1979_CALENDAR; // TODO allow LFF
        let psalter = &BCP1979_PSALTER;

        let morning = summarize_time(
            &calendar.liturgical_day(*date, false),
            calendar,
            psalter,
            language,
        );
        let evening = summarize_time(
            &calendar.liturgical_day(*date, true),
            calendar,
            psalter,
            language,
        );

        DailySummary { morning, evening }
    }
}

fn summarize_time(
    day: &LiturgicalDay,
    calendar: &Calendar,
    psalter: &Psalter,
    language: Language,
) -> PartialDailySummary {
    let observed = summarize_observance(day, &day.observed, calendar, language);
    let alternate = day
        .alternate
        .map(|alternate| summarize_observance(day, &alternate, calendar, language));
    let thirty_day_psalms =
        psalms_filtered_by_time(&BCP1979_30_DAY_PSALTER, psalter, &day.observed, day);

    PartialDailySummary {
        day: day.clone(),
        observed,
        alternate,
        thirty_day_psalms,
    }
}

fn psalms_filtered_by_time(
    psalm_cycle: &'static Lectionary,
    psalter: &Psalter,
    observance: &LiturgicalDayId,
    day: &LiturgicalDay,
) -> Vec<Psalm> {
    let mut psalms = psalm_cycle
        .readings_by_day(observance, day)
        .filter(|reading| {
            (day.evening && reading.reading_type == ReadingType::EveningPsalm)
                || (!day.evening && reading.reading_type == ReadingType::MorningPsalm)
        })
        .map(|reading| psalter.psalms_by_citation(&reading.citation))
        .flatten()
        .collect::<Vec<_>>();
    // sort by the number and first verse of each psalm
    psalms.sort_by_key(|psalm| {
        (
            psalm.number,
            psalm
                .sections
                .get(0)
                .and_then(|section| section.verses.get(0).map(|verse| verse.number)),
        )
    });
    psalms
}

fn summarize_observance(
    day: &LiturgicalDay,
    observance: &LiturgicalDayId,
    calendar: &Calendar,
    language: Language,
) -> ObservanceSummary {
    let localized_name = localize_day_name(day, observance, calendar, language);
    let black_letter_days = if day.weekday == Weekday::Sun {
        Vec::new()
    } else {
        day.holy_days
            .iter()
            .filter(|feast| {
                day.observed != LiturgicalDayId::Feast(**feast)
                    && day.alternate != Some(LiturgicalDayId::Feast(**feast))
            })
            .map(|feast| {
                (
                    *feast,
                    calendar
                        .feast_name(*feast, language)
                        .unwrap_or_default()
                        .to_string(),
                )
            })
            .collect()
    };

    let daily_office_readings = BCP1979_DAILY_OFFICE_LECTIONARY
        .readings_by_day(observance, day)
        // for Holy Days, only list morning readings in morning & evening readings in evening
        .filter(|reading| {
            !matches!(
                (reading.reading_type, day.evening),
                (ReadingType::Morning1, true)
                    | (ReadingType::Morning2, true)
                    | (ReadingType::Evening1, false)
                    | (ReadingType::Evening2, false)
            )
        })
        .collect();

    let daily_office_psalms = psalms_filtered_by_time(
        &BCP1979_DAILY_OFFICE_PSALTER,
        &BCP1979_PSALTER,
        observance,
        day,
    );

    ObservanceSummary {
        observance: *observance,
        localized_name,
        black_letter_days,
        daily_office_readings,
        daily_office_psalms,
    }
}

fn localize_day_name(
    day: &LiturgicalDay,
    id: &LiturgicalDayId,
    calendar: &Calendar,
    language: Language,
) -> String {
    match id {
        LiturgicalDayId::Feast(feast) | LiturgicalDayId::TransferredFeast(feast) => {
            calendar.feast_name(*feast, language).map(String::from)
        }
        _ => calendar.week_name(day.week, language).map(|name| {
            if day.weekday == Weekday::Sun {
                name.to_string()
            } else {
                format!(
                    "{} {} {}",
                    language.i18n(&day.weekday.to_string()),
                    language.i18n("after"),
                    name.replace("The", "the")
                )
            }
        }),
    }
    .unwrap_or_default()
}
