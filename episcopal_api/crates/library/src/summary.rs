use std::{collections::HashMap, convert::TryFrom};

use api::summary::{
    DailySummary, DocumentOrReading, EucharisticLectionarySummary, EucharisticObservanceSummary,
    FirstLessonAndPsalm, ObservanceSummary, PartialDailySummary, TrackedReadings,
};
use calendar::{
    Calendar, Date, Feast, LiturgicalDay, LiturgicalDayId, Weekday, BCP1979_CALENDAR,
    LFF2018_CALENDAR,
};
use canticle_table::CanticleId;
use liturgy::{Content, Document, LiturgyPreferences, Psalm, Version};
use psalter::{bcp1979::BCP1979_PSALTER, Psalter};

use language::Language;
use lectionary::{
    Lectionary, ReadingType, BCP1979_30_DAY_PSALTER, BCP1979_DAILY_OFFICE_LECTIONARY,
    BCP1979_DAILY_OFFICE_PSALTER, RCL, RCL_TRACK_1, RCL_TRACK_2, VIGIL_READING_TYPES,
};

use crate::{CommonPrayer, Library};

impl CommonPrayer {
    #[cfg(any(feature = "browser", feature = "server"))]
    pub fn daily_office_summary(date: &Date, language: Language) -> DailySummary {
        let psalter = &BCP1979_PSALTER;

        let morning = summarize_time(date, false, psalter, language);
        let evening = summarize_time(date, true, psalter, language);

        DailySummary {
            date: *date,
            morning,
            evening,
        }
    }

    #[cfg(any(feature = "browser", feature = "server"))]
    pub fn eucharistic_lectionary_summary(
        date: &Date,
        language: Language,
    ) -> EucharisticLectionarySummary {
        let psalter = &BCP1979_PSALTER;
        let day = BCP1979_CALENDAR.liturgical_day(*date, false);
        Self::eucharistic_lectionary_summary_with_day(day, language)
    }

    #[cfg(any(feature = "browser", feature = "server"))]
    pub fn eucharistic_lectionary_summary_with_day(
        day: LiturgicalDay,
        language: Language,
    ) -> EucharisticLectionarySummary {
        let psalter = &BCP1979_PSALTER;
        let alternates = day
            .alternative_services
            .iter()
            .map(|alternate| {
                summarize_eucharistic_observance(
                    &day,
                    &LiturgicalDayId::Feast(*alternate),
                    language,
                    psalter,
                )
            })
            .collect::<Vec<_>>();

        let observed = summarize_eucharistic_observance(&day, &day.observed, language, psalter);
        EucharisticLectionarySummary {
            day,
            observed,
            alternates,
        }
    }

    #[cfg(any(feature = "browser", feature = "server"))]
    pub fn alternate_service_summary(
        date: &Date,
        language: Language,
    ) -> EucharisticLectionarySummary {
        let psalter = &BCP1979_PSALTER;
        let day = BCP1979_CALENDAR.liturgical_day(*date, false);
        let observed = summarize_eucharistic_observance(&day, &day.observed, language, psalter);
        let alternates = day
            .alternate
            .as_ref()
            .map(|alternate| summarize_eucharistic_observance(&day, alternate, language, psalter))
            .into_iter()
            .collect();
        EucharisticLectionarySummary {
            day,
            observed,
            alternates,
        }
    }
}

#[cfg(any(feature = "browser", feature = "server"))]
fn summarize_eucharistic_observance(
    day: &LiturgicalDay,
    observance: &LiturgicalDayId,
    language: Language,
    psalter: &Psalter,
) -> EucharisticObservanceSummary {
    let localized_name = localize_day_name(day, observance, &BCP1979_CALENDAR, language);
    let collects = CommonPrayer::compile(
        Document::from(Content::CollectOfTheDay {
            allow_multiple: false,
        }),
        &BCP1979_CALENDAR,
        day,
        observance,
        &HashMap::new(),
        &LiturgyPreferences::default(),
    );

    let vigil_readings = vigil_readings(observance, day, &RCL, psalter);

    let tracked_readings = if let LiturgicalDayId::ProperAndDay(..) = observance {
        let track_one = Box::new(tracked_readings(observance, day, &RCL_TRACK_1, psalter));
        let track_two = Box::new(tracked_readings(observance, day, &RCL_TRACK_2, psalter));
        TrackedReadings::Tracked {
            track_one,
            track_two,
        }
    } else {
        TrackedReadings::Any(Box::new(tracked_readings(observance, day, &RCL, psalter)))
    };

    let epistle = RCL
        .reading_by_type(observance, day, ReadingType::SecondReading)
        .map(|reading| reading.citation)
        .collect();

    let gospel = RCL
        .reading_by_type(observance, day, ReadingType::Gospel)
        .map(|reading| reading.citation)
        .collect();

    let liturgy_of_the_palms = RCL
        .reading_by_type(observance, day, ReadingType::PalmsGospel)
        .map(|reading| reading.citation)
        .collect();

    EucharisticObservanceSummary {
        observance: *observance,
        localized_name,
        collects,
        vigil_readings,
        tracked_readings,
        epistle,
        gospel,
        liturgy_of_the_palms,
    }
}

#[cfg(any(feature = "browser", feature = "server"))]
fn vigil_readings(
    observance: &LiturgicalDayId,
    day: &LiturgicalDay,
    lectionary: &'static Lectionary,
    psalter: &Psalter,
) -> Vec<DocumentOrReading> {
    VIGIL_READING_TYPES
        .iter()
        .filter_map(|reading_type| {
            if reading_type.is_psalm() {
                let mut psalms = lectionary
                    .reading_by_type(observance, day, *reading_type)
                    .flat_map(|reading| {
                        psalm_citation_to_documents::<CommonPrayer>(
                            psalter,
                            Version::RiteII,
                            &reading.citation,
                        )
                        .into_iter()
                    });
                Document::choice_or_document(&mut psalms)
                    .map(|doc| DocumentOrReading::Document(Box::new(doc)))
            } else {
                let readings = lectionary
                    .reading_by_type(observance, day, *reading_type)
                    .map(|reading| reading.citation)
                    .collect::<Vec<_>>();
                if readings.is_empty() {
                    None
                } else {
                    Some(DocumentOrReading::Reading(readings))
                }
            }
        })
        .collect()
}

#[cfg(any(feature = "browser", feature = "server"))]
fn tracked_readings(
    observance: &LiturgicalDayId,
    day: &LiturgicalDay,
    lectionary: &'static Lectionary,
    psalter: &Psalter,
) -> FirstLessonAndPsalm {
    let first_lesson = lectionary
        .reading_by_type(observance, day, ReadingType::FirstReading)
        .map(|reading| reading.citation)
        .collect();
    let psalm = lectionary
        .reading_by_type(observance, day, ReadingType::Psalm)
        .flat_map(|reading| {
            psalm_citation_to_documents::<CommonPrayer>(psalter, Version::RiteII, &reading.citation)
                .into_iter()
        })
        .collect();
    FirstLessonAndPsalm {
        first_lesson,
        psalm,
    }
}

#[cfg(any(feature = "browser", feature = "server"))]
fn psalm_citation_to_documents<L: Library>(
    psalter: &Psalter,
    canticle_version: Version,
    citation: &str,
) -> Vec<Document> {
    if citation.starts_with("Canticle") || citation.starts_with("canticle-") {
        if let Some(doc) = CanticleId::try_from(citation)
            .ok()
            .and_then(|id| L::canticle(id, canticle_version))
        {
            vec![doc]
        } else {
            vec![]
        }
    } else {
        psalter
            .psalms_by_citation(citation)
            .iter()
            .cloned()
            .map(Document::from)
            .collect()
    }
}

#[cfg(any(feature = "browser", feature = "server"))]
fn summarize_time(
    date: &Date,
    evening: bool,
    psalter: &Psalter,
    language: Language,
) -> PartialDailySummary {
    let day = BCP1979_CALENDAR.liturgical_day(*date, evening);
    let lff_day = LFF2018_CALENDAR.liturgical_day(*date, evening);
    let lff_holy_days = lff_day.holy_days;
    let observed = summarize_observance(&day, &day.observed, &lff_holy_days, language);
    let alternate = day
        .alternate
        .map(|alternate| summarize_observance(&day, &alternate, &lff_holy_days, language));
    let thirty_day_psalms =
        psalms_filtered_by_time(&BCP1979_30_DAY_PSALTER, psalter, &day.observed, &day);

    PartialDailySummary {
        day,
        observed,
        alternate,
        thirty_day_psalms,
    }
}

#[cfg(any(feature = "browser", feature = "server"))]
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
        .flat_map(|reading| psalter.psalms_by_citation(&reading.citation))
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

#[cfg(any(feature = "browser", feature = "server"))]
fn summarize_observance(
    day: &LiturgicalDay,
    observance: &LiturgicalDayId,
    lff_holy_days: &[Feast],
    language: Language,
) -> ObservanceSummary {
    let localized_name = localize_day_name(day, observance, &BCP1979_CALENDAR, language);
    let bcp_black_letter_days = black_letter_days(&BCP1979_CALENDAR, day, &day.holy_days, language);
    let lff_black_letter_days = black_letter_days(&LFF2018_CALENDAR, day, lff_holy_days, language);

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

    let collects = CommonPrayer::compile(
        Document::from(Content::CollectOfTheDay {
            allow_multiple: true,
        }),
        &LFF2018_CALENDAR,
        day,
        observance,
        &HashMap::new(),
        &LiturgyPreferences::default(),
    );

    ObservanceSummary {
        observance: *observance,
        localized_name,
        bcp_black_letter_days,
        lff_black_letter_days,
        daily_office_readings,
        daily_office_psalms,
        collects,
    }
}

fn black_letter_days(
    calendar: &Calendar,
    day: &LiturgicalDay,
    holy_days: &[Feast],
    language: Language,
) -> Vec<(Feast, String)> {
    if day.weekday == Weekday::Sun {
        Vec::new()
    } else {
        holy_days
            .iter()
            .filter(|feast| {
                day.observed != LiturgicalDayId::Feast(**feast)
                    && day.alternate != Some(LiturgicalDayId::Feast(**feast))
            })
            .map(|feast| {
                (
                    *feast,
                    calendar.feast_name(*feast, language).unwrap_or_default(),
                )
            })
            .collect()
    }
}

pub fn localize_day_name(
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
            let weekday_name = day.weekday.to_string();
            if day.weekday == Weekday::Sun {
                name.to_string()
            } else {
                format!(
                    "{} {} {}",
                    language.i18n(&weekday_name),
                    language.i18n("after"),
                    name.replace("The", "the")
                )
            }
        }),
    }
    .unwrap_or_default()
}
