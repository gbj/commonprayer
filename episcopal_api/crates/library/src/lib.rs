use calendar::{
    feasts::CommonOfSaints, Calendar, Feast, LiturgicalDay, LiturgicalDayId, LiturgicalWeek,
    Proper, Season, Weekday,
};
use canticle_table::{CanticleId, CanticleTable};
use lectionary::{Lectionary, ReadingType};
use liturgy::*;
use psalter::{bcp1979::BCP1979_PSALTER, Psalter};

use serde::{Deserialize, Serialize};

#[macro_use]
extern crate lazy_static;

pub mod conditions;
pub mod eow;
pub mod lff2018;
pub mod rite1;
pub mod rite2;
pub mod summary;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CommonPrayerLiturgies {
    NoondayPrayer,
    Compline,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CollectId {
    Week(LiturgicalWeek),
    Proper(Proper),
    Season(Season),
    Feast(Feast),
    CommonOfSaints(CommonOfSaints),
}

pub trait Library {
    fn psalter(psalter: Version) -> &'static Psalter;

    fn lectionary(lectionary: Lectionaries) -> &'static Lectionary;

    fn canticle_table(table: CanticleTables) -> &'static CanticleTable;

    fn canticle(canticle: CanticleId, version: Version) -> Option<Document>;

    fn category(category: Categories, version: Version) -> Vec<Document>;

    fn compile(
        mut document: Document,
        calendar: &Calendar,
        day: &LiturgicalDay,
        observed: &LiturgicalDayId,
        prefs: &impl ClientPreferences,
        liturgy_prefs: &LiturgyPreferences,
    ) -> Option<Document> {
        document.is_compiled = true;

        let preference_value_for_key = |key: &PreferenceKey| {
            prefs
                .value(key)
                .or_else(|| liturgy_prefs.default_value_for_key(key))
        };

        let include = document.include(calendar, day, prefs, liturgy_prefs)
            && document.display != Show::TemplateOnly
            && document.display != Show::Hidden;
        if !include {
            None
        } else {
            match &document.content {
                // Category Lookup
                Content::Category(category) => {
                    let category_docs = Self::category(category.name, document.version);

                    Document::choice_or_document(&mut category_docs.into_iter())
                        .and_then(|docs| {
                            Self::compile(docs, calendar, day, observed, prefs, liturgy_prefs)
                        })
                        .map(|mut doc| {
                            if let Content::Choice(ref mut choice) = doc.content {
                                choice.rotate(&day.date);
                            }
                            doc
                        })
                }

                // Lectionaries
                Content::LectionaryReading(lectionary_reading) => {
                    let chosen_lectionary = match &lectionary_reading.lectionary {
                        LectionaryTableChoice::Preference(key) => {
                            match preference_value_for_key(key) {
                                Some(PreferenceValue::Lectionary(lectionary)) => *lectionary,
                                _ => Lectionaries::default(),
                            }
                        }
                        LectionaryTableChoice::Selected(lectionary) => *lectionary,
                    };

                    let reading_type = match &lectionary_reading.reading_type {
                        ReadingTypeTable::Preference(key) => match preference_value_for_key(key) {
                            Some(PreferenceValue::ReadingType(reading_type)) => Some(*reading_type),
                            _ => None,
                        },
                        ReadingTypeTable::Selected(reading_type) => Some(*reading_type),
                    };

                    let lectionary = Self::lectionary(chosen_lectionary);

                    if let Some(reading_type) = reading_type {
                        let mut docs = lectionary.reading_by_type(observed, day, reading_type).map(
                            |reading| {
                                if reading_type.is_psalm() {
                                    Self::compile(
                                        Document::from(PsalmCitation::from(reading.citation)),
                                        calendar,
                                        day,
                                        observed,
                                        prefs,
                                        liturgy_prefs,
                                    )
                                    .unwrap()
                                } else {
                                    let intro = lectionary_reading.intro.as_ref().map(|intro| {
                                        BiblicalReadingIntro::from(intro.compile(&reading.citation))
                                    });
                                    Document {
                                        content: Content::BiblicalCitation(BiblicalCitation {
                                            citation: reading.citation,
                                            intro,
                                        }),
                                        ..document.clone()
                                    }
                                }
                            },
                        );

                        // MorningPsalm and EveningPsalm are the only ones that include multiple of the same reading type in sequence
                        if matches!(
                            reading_type,
                            ReadingType::MorningPsalm | ReadingType::EveningPsalm
                        ) {
                            Document::series_or_document(&mut docs)
                        } else {
                            Document::choice_or_document(&mut docs)
                        }
                    } else {
                        Some(Document::from(DocumentError::from(
                            "An invalid reading type preference was selected.",
                        )))
                    }
                }

                // Canticle Tables
                Content::CanticleTableEntry(entry) => {
                    let chosen_table = match &entry.table {
                        CanticleTableChoice::Preference(key) => {
                            match preference_value_for_key(key) {
                                Some(PreferenceValue::CanticleTable(table)) => *table,
                                _ => CanticleTables::default(),
                            }
                        }
                        CanticleTableChoice::Selected(table) => *table,
                    };

                    let table = Self::canticle_table(chosen_table);

                    let entries = table.find(calendar, day, entry.nth, None, false);

                    let mut docs = entries.iter().map(|id| {
                        Self::canticle(*id, document.version).unwrap_or_else(|| {
                            Document::from(DocumentError::from(format!(
                                "{:#?} not available in {:#?}",
                                id, document.version
                            )))
                        })
                    });
                    Document::choice_or_document(&mut docs)
                }

                // Headings
                // Insert day/date into heading if necessary
                Content::Heading(heading) => match heading {
                    Heading::InsertDate => Some(Document::from(Heading::Date(
                        day.date.to_localized_name(document.language),
                    ))),

                    Heading::InsertDay => {
                        let observed = day.observed;
                        let name = match observed {
                            LiturgicalDayId::Feast(feast) => calendar
                                .feast_name(feast, document.language)
                                .map(|name| name.to_string()),
                            LiturgicalDayId::TransferredFeast(feast) => {
                                calendar.feast_name(feast, document.language).map(|name| {
                                    format!(
                                        "{} \n\n({})",
                                        name,
                                        document.language.i18n("Transferred")
                                    )
                                })
                            }
                            _ => calendar.week_name(day.week, document.language).map(|name| {
                                if day.weekday == Weekday::Sun {
                                    name.to_string()
                                } else {
                                    format!(
                                        "{} {} {}",
                                        document.language.i18n(&day.weekday.to_string()),
                                        document.language.i18n("after"),
                                        name.replace("The", "the")
                                    )
                                }
                            }),
                        }
                        .unwrap_or_default();

                        let proper = day
                            .proper
                            .and_then(|proper| calendar.proper_name(proper, document.language))
                            .map(String::from);

                        let holy_days = if day.holy_days.is_empty() {
                            None
                        } else {
                            Some(
                                day.holy_days
                                    .iter()
                                    .filter(|s_feast| match &day.observed {
                                        LiturgicalDayId::Feast(feast) => feast != *s_feast,
                                        LiturgicalDayId::TransferredFeast(feast) => {
                                            feast != *s_feast
                                        }
                                        _ => true,
                                    })
                                    .filter_map(|feast| {
                                        calendar.feast_name(*feast, document.language)
                                    })
                                    .map(|name| name.to_string())
                                    .collect::<Vec<_>>(),
                            )
                        };

                        Some(Document::from(Heading::Day {
                            name,
                            proper,
                            holy_days,
                        }))
                    }

                    // ordinary headings and completed days/dates are passed through
                    _ => Some(document),
                },

                // Lookup types
                Content::PsalmCitation(citation) => {
                    let psalter_pref = match preference_value_for_key(&PreferenceKey::from(
                        GlobalPref::PsalterVersion,
                    )) {
                        Some(PreferenceValue::Version(v)) => Some(*v),
                        _ => None,
                    }
                    .unwrap_or_default();
                    let psalter = Self::psalter(psalter_pref);
                    let psalms: Vec<Psalm> = psalter.psalms_by_citation(citation.as_str());
                    if psalms.is_empty() {
                        None
                    } else if psalms.len() == 1 {
                        Some(Document::from(psalms[0].clone()))
                    } else {
                        Some(Document::from(Series::from(psalms)))
                    }
                }
                // Collection types
                Content::Liturgy(liturgy) => Some(Document {
                    content: Content::Liturgy(Liturgy {
                        body: Series::from(
                            liturgy
                                .body
                                .iter()
                                .filter_map(|doc| {
                                    Self::compile(
                                        doc.clone(),
                                        calendar,
                                        day,
                                        observed,
                                        prefs,
                                        liturgy_prefs,
                                    )
                                })
                                .collect::<Vec<_>>(),
                        ),
                        evening: liturgy.evening,
                        preferences: liturgy.preferences.clone(),
                    }),
                    ..document
                }),
                Content::Series(sub) => Some(Document {
                    content: Content::Series(Series::from(
                        sub.iter()
                            .filter_map(|doc| {
                                Self::compile(
                                    doc.clone(),
                                    calendar,
                                    day,
                                    observed,
                                    prefs,
                                    liturgy_prefs,
                                )
                            })
                            .collect::<Vec<_>>(),
                    )),
                    ..document
                }),
                Content::Parallel(sub) => Some(Document {
                    content: Content::Parallel(Parallel::from(
                        sub.iter()
                            .filter_map(|doc| {
                                Self::compile(
                                    doc.clone(),
                                    calendar,
                                    day,
                                    observed,
                                    prefs,
                                    liturgy_prefs,
                                )
                            })
                            .collect::<Vec<_>>(),
                    )),
                    ..document
                }),
                Content::Choice(sub) => {
                    // try, when filtering selections, to maintain the currently-selected item -- or default to 0
                    let prev_selection = sub.options.get(sub.selected);
                    let index_of_prev_selection = prev_selection
                        .and_then(|prev| sub.options.iter().position(|search| search == prev));

                    Some(Document {
                        content: Content::Choice(Choice {
                            options: sub
                                .options
                                .iter()
                                .filter_map(|doc| {
                                    Self::compile(
                                        doc.clone(),
                                        calendar,
                                        day,
                                        observed,
                                        prefs,
                                        liturgy_prefs,
                                    )
                                })
                                .collect(),
                            selected: index_of_prev_selection.unwrap_or(0),
                            rotated: sub.rotated,
                        }),
                        ..document
                    })
                }
                // Every else just passes through as is
                _ => Some(document),
            }
        }
    }
}

pub struct CommonPrayer {}

impl Library for CommonPrayer {
    fn psalter(_psalter: Version) -> &'static Psalter {
        &BCP1979_PSALTER
    }

    fn lectionary(lectionary: Lectionaries) -> &'static Lectionary {
        match lectionary {
            Lectionaries::BCP1979DailyOffice => &lectionary::BCP1979_DAILY_OFFICE_LECTIONARY,
            Lectionaries::BCP1979DailyOfficePsalms => &lectionary::BCP1979_DAILY_OFFICE_PSALTER,
            Lectionaries::BCP1979ThirtyDayPsalms => &lectionary::BCP1979_30_DAY_PSALTER,
            Lectionaries::RCLTrack1 => &lectionary::RCL_TRACK_1,
            Lectionaries::RCLTrack2 => &lectionary::RCL_TRACK_2,
        }
    }

    fn canticle_table(table: CanticleTables) -> &'static CanticleTable {
        match table {
            CanticleTables::BCP1979RiteI => &canticle_table::bcp1979::BCP1979_CANTICLE_TABLE_RITE_I,
            CanticleTables::BCP1979RiteII => {
                &canticle_table::bcp1979::BCP1979_CANTICLE_TABLE_RITE_II
            }
            CanticleTables::EOW => &canticle_table::eow::EOW_CANTICLE_TABLE,
            CanticleTables::Classical => todo!(),
        }
    }

    fn canticle(canticle: CanticleId, version: Version) -> Option<Document> {
        match (canticle, version) {
            (CanticleId::Canticle1, _) => Some(rite1::CANTICLE_1.clone()),
            (CanticleId::Canticle2, _) => Some(rite1::CANTICLE_2.clone()),
            (CanticleId::Canticle3, _) => Some(rite1::CANTICLE_3.clone()),
            (CanticleId::Canticle4, _) => Some(rite1::CANTICLE_4.clone()),
            (CanticleId::Canticle5, _) => Some(rite1::CANTICLE_5.clone()),
            (CanticleId::Canticle6, _) => Some(rite1::CANTICLE_6.clone()),
            (CanticleId::Canticle7, _) => Some(rite1::CANTICLE_7.clone()),
            (CanticleId::Canticle8, _) => Some(rite2::CANTICLE_8.clone()),
            (CanticleId::Canticle9, _) => Some(rite2::CANTICLE_9.clone()),
            (CanticleId::Canticle10, _) => Some(rite2::CANTICLE_10.clone()),
            (CanticleId::Canticle11, _) => Some(rite2::CANTICLE_11.clone()),
            (CanticleId::Canticle12, Version::EOW) => Some(eow::CANTICLE_12_EOW.clone()),
            (CanticleId::Canticle12, _) => Some(rite2::CANTICLE_12.clone()),
            (CanticleId::Canticle13, _) => Some(rite2::CANTICLE_13.clone()),
            (CanticleId::Canticle14, _) => Some(rite2::CANTICLE_14.clone()),
            (CanticleId::Canticle15, Version::EOW) => Some(eow::CANTICLE_15_EOW.clone()),
            (CanticleId::Canticle15, _) => Some(rite2::CANTICLE_15.clone()),
            (CanticleId::Canticle16, Version::EOW) => Some(eow::CANTICLE_16_EOW.clone()),
            (CanticleId::Canticle16, _) => Some(rite2::CANTICLE_16.clone()),
            (CanticleId::Canticle17, _) => Some(rite2::CANTICLE_17.clone()),
            (CanticleId::Canticle18, Version::EOW) => Some(eow::CANTICLE_18_EOW.clone()),
            (CanticleId::Canticle18, _) => Some(rite2::CANTICLE_18.clone()),
            (CanticleId::Canticle19, _) => Some(rite2::CANTICLE_19.clone()),
            (CanticleId::Canticle20, _) => Some(rite2::CANTICLE_20.clone()),
            (CanticleId::Canticle21, Version::EOW) => Some(eow::CANTICLE_21_EOW.clone()),
            (CanticleId::Canticle21, _) => Some(rite2::CANTICLE_21.clone()),
            (CanticleId::CanticleA, _) => Some(eow::CANTICLE_A.clone()),
            (CanticleId::CanticleB, _) => Some(eow::CANTICLE_B.clone()),
            (CanticleId::CanticleC, _) => Some(eow::CANTICLE_C.clone()),
            (CanticleId::CanticleD, _) => Some(eow::CANTICLE_D.clone()),
            (CanticleId::CanticleE, _) => Some(eow::CANTICLE_E.clone()),
            (CanticleId::CanticleF, _) => Some(eow::CANTICLE_F.clone()),
            (CanticleId::CanticleG, _) => Some(eow::CANTICLE_G.clone()),
            (CanticleId::CanticleH, _) => Some(eow::CANTICLE_H.clone()),
            (CanticleId::CanticleI, _) => Some(eow::CANTICLE_I.clone()),
            (CanticleId::CanticleJ, _) => Some(eow::CANTICLE_J.clone()),
            (CanticleId::CanticleK, _) => Some(eow::CANTICLE_K.clone()),
            (CanticleId::CanticleL, _) => Some(eow::CANTICLE_L.clone()),
            (CanticleId::CanticleM, _) => Some(eow::CANTICLE_M.clone()),
            (CanticleId::CanticleN, _) => Some(eow::CANTICLE_N.clone()),
            (CanticleId::CanticleO, _) => Some(eow::CANTICLE_O.clone()),
            (CanticleId::CanticleP, _) => Some(eow::CANTICLE_P.clone()),
            (CanticleId::CanticleQ, _) => Some(eow::CANTICLE_Q.clone()),
            (CanticleId::CanticleR, _) => Some(eow::CANTICLE_R.clone()),
            (CanticleId::CanticleS, _) => Some(eow::CANTICLE_S.clone()),
        }
    }

    fn category(category: Categories, version: Version) -> Vec<Document> {
        match (category, version) {
            (Categories::OpeningSentences, Version::RiteII) => {
                rite2::office::OPENING_SENTENCES.clone()
            }
            (Categories::InvitatoryAntiphons, Version::RiteII) => {
                rite2::office::INVITATORY_ANTIPHONS.clone()
            }
            (Categories::ClosingSentences, Version::RiteII) => {
                rite2::office::CLOSING_SENTENCES.clone()
            }
            _ => Vec::new(),
        }
    }
}
