use calendar::{Calendar, LiturgicalDay, LiturgicalDayId, Rank, Weekday};
use canticle_table::{CanticleId, CanticleTable};
use itertools::Itertools;
use language::Language;
use lectionary::{Lectionary, ReadingType};
use liturgy::*;
use loc::collects::COLECTAS;
use psalter::{bcp1979::BCP1979_PSALTER, Psalter};

use rite1::GLORIA_PATRI_TRADITIONAL;
use serde::{Deserialize, Serialize};

use crate::{
    lff2018::collects::{LFF_COLLECTS_CONTEMPORARY, LFF_COLLECTS_TRADITIONAL},
    rite1::collects::COLLECTS_TRADITIONAL,
    rite2::collects::COLLECTS_CONTEMPORARY,
};

#[macro_use]
extern crate lazy_static;

pub mod bcp1979;
pub mod collect;
pub mod conditions;
pub mod eow;
pub mod lff2018;
pub mod loc;
pub mod rite1;
pub mod rite2;
pub mod summary;
pub use collect::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CommonPrayerLiturgies {
    NoondayPrayer,
    Compline,
}

pub trait Library {
    fn psalter(psalter: Version) -> &'static Psalter<'static>;

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
                        let mut docs = lectionary
                            .reading_by_type_with_override(
                                observed,
                                day,
                                reading_type,
                                lectionary_reading.reading_type_overridden_by,
                            )
                            .map(|reading| {
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
                                        version: prefs
                                            .value(&PreferenceKey::from(GlobalPref::BibleVersion))
                                            .and_then(|value| match value {
                                                PreferenceValue::Version(version) => Some(*version),
                                                _ => None,
                                            })
                                            .unwrap_or(Version::NRSV),
                                        ..document.clone()
                                    }
                                }
                            });

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
                        let mut canticle =
                            Self::canticle(*id, document.version).unwrap_or_else(|| {
                                Document::from(DocumentError::from(format!(
                                    "{:#?} not available in {:#?}",
                                    id, document.version
                                )))
                            });
                        // Switch between contemporary and traditional Gloria Patri depending on preference
                        if prefs.value(&PreferenceKey::from(GlobalPref::GloriaPatriTraditional))
                            == Some(&PreferenceValue::Bool(true))
                        {
                            if let Content::Canticle(ref mut canticle) = &mut canticle.content {
                                if canticle.gloria_patri.is_some() {
                                    canticle.gloria_patri = Some(GLORIA_PATRI_TRADITIONAL.clone());
                                }
                            }
                        }
                        canticle
                    });
                    Document::choice_or_document(&mut docs)
                }

                // Switch between contemporary and traditional Gloria Patri depending on preference
                Content::GloriaPatri(_) => {
                    if prefs.value(&PreferenceKey::from(GlobalPref::GloriaPatriTraditional))
                        == Some(&PreferenceValue::Bool(true))
                    {
                        Some(Document {
                            content: Content::GloriaPatri(GLORIA_PATRI_TRADITIONAL.clone()),
                            ..document
                        })
                    } else {
                        Some(document)
                    }
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
                            LiturgicalDayId::Feast(feast) => {
                                calendar.feast_name(feast, document.language)
                            }
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
                                        calendar
                                            .feast_name(*feast, document.language)
                                            .map(|name| (*feast, name))
                                    })
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

                // Insert seasonal antiphon for invitatories
                Content::Invitatory(invitatory) => match invitatory.antiphon {
                    SeasonalAntiphon::Insert => {
                        if let Some(antiphon) = Self::compile(
                            Document::from(Content::Category(Category::from(
                                Categories::InvitatoryAntiphons,
                            ))),
                            calendar,
                            day,
                            observed,
                            prefs,
                            liturgy_prefs,
                        ) {
                            match (&antiphon.content, &mut document.content) {
                                (
                                    Content::Antiphon(ant),
                                    Content::Invitatory(ref mut invitatory),
                                ) => {
                                    invitatory.antiphon = SeasonalAntiphon::Antiphon(ant.clone());
                                    Some(document)
                                }
                                (
                                    Content::Choice(choice),
                                    Content::Invitatory(ref mut invitatory),
                                ) => {
                                    let selection = &choice.options[choice.selected];
                                    if let Content::Antiphon(ant) = &selection.content {
                                        invitatory.antiphon =
                                            SeasonalAntiphon::Antiphon(ant.clone());
                                    }
                                    Some(document)
                                }
                                _ => Some(document),
                            }
                        } else {
                            Some(document)
                        }
                    }
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
                Content::CollectOfTheDay { allow_multiple } => {
                    let language = document.language;
                    let traditional_language = matches!(document.version, Version::RiteI);
                    let use_black_letter_collects = prefs
                        .value(&PreferenceKey::from(GlobalPref::UseBlackLetterCollects))
                        .and_then(|value| match value {
                            PreferenceValue::Bool(bool) => Some(*bool),
                            _ => None,
                        })
                        .unwrap_or(true);

                    // create collect list, based on version + calendar + black-letter collect preferences
                    let collects: Box<dyn Iterator<Item = &(CollectId, CollectData)>> =
                        match (language, traditional_language, use_black_letter_collects) {
                            (Language::Es, _, _) => Box::new(COLECTAS.iter()),
                            (_, true, true) => Box::new(
                                COLLECTS_TRADITIONAL
                                    .iter()
                                    .chain(LFF_COLLECTS_TRADITIONAL.iter()),
                            ),
                            (_, true, false) => Box::new(COLLECTS_TRADITIONAL.iter()),
                            (_, false, true) => Box::new(
                                COLLECTS_CONTEMPORARY
                                    .iter()
                                    .chain(LFF_COLLECTS_CONTEMPORARY.iter()),
                            ),
                            (_, false, false) => Box::new(COLLECTS_CONTEMPORARY.iter()),
                        };
                    let collects = collects.collect::<Vec<_>>();

                    let day_rank = calendar.rank(day);
                    let holy_day_collect = match observed {
                        LiturgicalDayId::Feast(feast) => {
                            let feast = calendar.feast_eve_following_day(feast).unwrap_or(*feast);

                            let id = COLLECT_LINKS.linked_id(&CollectId::Feast(feast));

                            if day_rank >= Rank::PrecedenceOverWeekday {
                                Some(Document::choice_or_document(
                                    &mut collects
                                        .iter()
                                        .filter_map(move |(s_id, data)| {
                                            if s_id == &id {
                                                Some(data.document.clone())
                                            } else {
                                                None
                                            }
                                        })
                                        .unique_by(|doc| doc.content.clone()),
                                ))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                    .flatten();

                    let sunday_collect = if holy_day_collect.is_none() {
                        let week_id = if let Some(proper) = day.proper {
                            CollectId::Proper(proper)
                        } else {
                            CollectId::Week(day.week)
                        };
                        let week_id = COLLECT_LINKS.linked_id(&week_id);

                        Document::choice_or_document(&mut collects.iter().filter_map(
                            move |(id, data)| {
                                if *id == week_id {
                                    Some(data.document.clone())
                                } else {
                                    None
                                }
                            },
                        ))
                    } else {
                        None
                    };

                    let sunday_or_holy_day = match (holy_day_collect, sunday_collect) {
                        (None, None) => None,
                        (None, Some(sunday)) => Some(sunday),
                        (Some(holy_day), None) => Some(holy_day),
                        (Some(holy_day), Some(_)) => Some(holy_day),
                    };

                    let day_season = calendar.season(day);
                    let season_id = COLLECT_LINKS.linked_id(&CollectId::Season(day_season));
                    let seasonal_collect = collects
                        .iter()
                        .find(|(id, _)| id == &season_id)
                        .map(|(_, document)| document.clone());

                    let black_letter_feast_ids = day
                        .holy_days
                        .iter()
                        .filter(|feast| {
                            day.observed != LiturgicalDayId::Feast(**feast)
                                && day.alternate != Some(LiturgicalDayId::Feast(**feast))
                        })
                        .map(|feast| CollectId::Feast(*feast))
                        .collect::<Vec<_>>();

                    let black_letter_collects = if use_black_letter_collects {
                        Some(
                            collects
                                .iter()
                                .filter_map(|(id, data)| {
                                    if black_letter_feast_ids.contains(id) {
                                        Some(data.document.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>(),
                        )
                    } else {
                        None
                    };

                    let mut collects = Vec::new();
                    // Sunday or Holy Day
                    if let Some(collect) = sunday_or_holy_day {
                        collects.push(collect);
                    }
                    // Black-letter collects
                    if let Some(mut bl_collects) = black_letter_collects {
                        collects.append(&mut bl_collects);
                    }
                    // Seasonal collect
                    if let Some(collect) = seasonal_collect {
                        collects.push(collect.document);
                    }

                    // deduplicate by content
                    let collects = collects
                        .into_iter()
                        .unique_by(|doc| doc.content.clone())
                        .collect::<Vec<_>>();

                    if *allow_multiple {
                        Document::series_or_document(&mut collects.into_iter())
                    } else {
                        collects.get(0).cloned()
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

                    let mut choice = Choice {
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
                        should_rotate: sub.should_rotate,
                    };

                    if choice.should_rotate {
                        choice.rotate(&day.date);
                    }

                    Some(Document {
                        content: Content::Choice(choice),
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
    fn psalter(_psalter: Version) -> &'static Psalter<'static> {
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
