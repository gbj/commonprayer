use canticle_table::{CanticleId, CanticleTable};
use lectionary::Lectionary;
use liturgy::{CanticleTables, Document, Lectionaries, Version};
use psalter::{bcp1979::BCP1979_PSALTER, Psalter};

use crate::{bcp1979, eow, rite1, rite2, Contents, Library, Section, Slug, TableOfContents};

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
            _ => None,
        }
    }

    fn contents<'a>() -> TableOfContents<'a> {
        TableOfContents::from(vec![
            (
                Slug::Office,
                Contents::Sections {
                    label: "The Daily Office".into(),
                    contents: vec![
                        Section {
                            label: None,
                            contents: vec![
                                (
                                    Slug::MorningPrayer,
                                    Contents::ByVersion {
                                        label: "Morning Prayer".into(),
                                        documents: vec![&*rite2::office::MORNING_PRAYER_II],
                                    },
                                ),
                                (
                                    Slug::NoondayPrayer,
                                    Contents::Document(&*bcp1979::office::NOONDAY_PRAYER),
                                ),
                                (
                                    Slug::EveningPrayer,
                                    Contents::ByVersion {
                                        label: "Evening Prayer".into(),
                                        documents: vec![&*rite2::office::EVENING_PRAYER_II],
                                    },
                                ),
                                (
                                    Slug::Compline,
                                    Contents::Document(&*bcp1979::office::COMPLINE),
                                ),
                            ],
                        },
                        Section {
                            label: None,
                            contents: vec![
                                (Slug::SuggestedCanticles, Contents::Page("canticle-table")),
                                (
                                    Slug::Canticles,
                                    Contents::Category {
                                        label: "Canticles".into(),
                                        contents: vec![
                                            (
                                                Slug::Canticle(CanticleId::Canticle1),
                                                Contents::Document(&*rite1::canticles::CANTICLE_1),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle2),
                                                Contents::Document(&*rite1::canticles::CANTICLE_2),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle3),
                                                Contents::Document(&*rite1::canticles::CANTICLE_3),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle4),
                                                Contents::Document(&*rite1::canticles::CANTICLE_4),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle5),
                                                Contents::Document(&*rite1::canticles::CANTICLE_5),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle6),
                                                Contents::Document(&*rite1::canticles::CANTICLE_6),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle7),
                                                Contents::Document(&*rite1::canticles::CANTICLE_7),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle8),
                                                Contents::Document(&*rite2::canticles::CANTICLE_8),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle9),
                                                Contents::Document(&*rite2::canticles::CANTICLE_9),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle10),
                                                Contents::Document(&*rite2::canticles::CANTICLE_10),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle11),
                                                Contents::Document(&*rite2::canticles::CANTICLE_11),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle12),
                                                Contents::ByVersion {
                                                    label: "Canticle 12".into(),
                                                    documents: vec![
                                                        &*rite2::canticles::CANTICLE_12,
                                                        &*eow::canticles::CANTICLE_12_EOW,
                                                    ],
                                                },
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle13),
                                                Contents::Document(&*rite2::canticles::CANTICLE_13),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle14),
                                                Contents::Document(&*rite2::canticles::CANTICLE_14),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle15),
                                                Contents::ByVersion {
                                                    label: "Canticle 15".into(),
                                                    documents: vec![
                                                        &*rite2::canticles::CANTICLE_15,
                                                        &*eow::canticles::CANTICLE_15_EOW,
                                                    ],
                                                },
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle16),
                                                Contents::ByVersion {
                                                    label: "Canticle 16".into(),
                                                    documents: vec![
                                                        &*rite2::canticles::CANTICLE_16,
                                                        &*eow::canticles::CANTICLE_16_EOW,
                                                    ],
                                                },
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle17),
                                                Contents::Document(&*rite2::canticles::CANTICLE_17),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle18),
                                                Contents::ByVersion {
                                                    label: "Canticle 18".into(),
                                                    documents: vec![
                                                        &*rite2::canticles::CANTICLE_18,
                                                        &*eow::canticles::CANTICLE_18_EOW,
                                                    ],
                                                },
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle19),
                                                Contents::Document(&*rite2::canticles::CANTICLE_19),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle20),
                                                Contents::Document(&*rite2::canticles::CANTICLE_20),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::Canticle21),
                                                Contents::ByVersion {
                                                    label: "Canticle 21".into(),
                                                    documents: vec![
                                                        &*rite2::canticles::CANTICLE_21,
                                                        &*eow::canticles::CANTICLE_21_EOW,
                                                    ],
                                                },
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleA),
                                                Contents::Document(&*eow::canticles::CANTICLE_A),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleB),
                                                Contents::Document(&*eow::canticles::CANTICLE_B),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleC),
                                                Contents::Document(&*eow::canticles::CANTICLE_C),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleD),
                                                Contents::Document(&*eow::canticles::CANTICLE_D),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleE),
                                                Contents::Document(&*eow::canticles::CANTICLE_E),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleF),
                                                Contents::Document(&*eow::canticles::CANTICLE_F),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleG),
                                                Contents::Document(&*eow::canticles::CANTICLE_G),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleH),
                                                Contents::Document(&*eow::canticles::CANTICLE_H),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleI),
                                                Contents::Document(&*eow::canticles::CANTICLE_I),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleJ),
                                                Contents::Document(&*eow::canticles::CANTICLE_J),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleK),
                                                Contents::Document(&*eow::canticles::CANTICLE_K),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleL),
                                                Contents::Document(&*eow::canticles::CANTICLE_L),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleM),
                                                Contents::Document(&*eow::canticles::CANTICLE_M),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleN),
                                                Contents::Document(&*eow::canticles::CANTICLE_N),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleO),
                                                Contents::Document(&*eow::canticles::CANTICLE_O),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleP),
                                                Contents::Document(&*eow::canticles::CANTICLE_P),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleQ),
                                                Contents::Document(&*eow::canticles::CANTICLE_Q),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleR),
                                                Contents::Document(&*eow::canticles::CANTICLE_R),
                                            ),
                                            (
                                                Slug::Canticle(CanticleId::CanticleS),
                                                Contents::Document(&*eow::canticles::CANTICLE_S),
                                            ),
                                        ],
                                    },
                                ),
                            ],
                        },
                        // Hidden from TOC navigation, but used in compilation and manual navigation within liturgies
                        Section {
                            label: None,
                            contents: vec![
                                (
                                    Slug::OpeningSentences,
                                    Contents::MultiDocument {
                                        label: "Opening Sentences".into(),
                                        documents: rite2::OPENING_SENTENCES.to_vec(),
                                        hidden_in_toc: true,
                                    },
                                ),
                                (
                                    Slug::InvitatoryAntiphons,
                                    Contents::MultiDocument {
                                        label: "Invitatory Antiphons".into(),
                                        documents: rite2::INVITATORY_ANTIPHONS.to_vec(),
                                        hidden_in_toc: true,
                                    },
                                ),
                                (
                                    Slug::ClosingSentences,
                                    Contents::MultiDocument {
                                        label: "Closing Sentences".into(),
                                        documents: rite2::CLOSING_SENTENCES.to_vec(),
                                        hidden_in_toc: true,
                                    },
                                ),
                            ],
                        },
                    ],
                },
            ),
            (
                Slug::GreatLitany,
                Contents::Document(&*bcp1979::THE_GREAT_LITANY),
            ),
            (
                Slug::PrayersAndThanksgivings,
                Contents::MultiDocument {
                    label: "Prayers and Thanksgivings".into(),
                    documents: (&*bcp1979::PRAYERS_AND_THANKSGIVINGS).clone(),
                    hidden_in_toc: false,
                },
            ),
            (
                Slug::Baptism,
                Contents::Category {
                    label: "Baptism".into(),
                    contents: vec![
                        (
                            Slug::ConcerningTheService,
                            Contents::Document(&*bcp1979::baptism::CONCERNING_THE_SERVICE),
                        ),
                        (
                            Slug::Baptism,
                            Contents::Document(&*bcp1979::baptism::HOLY_BAPTISM),
                        ),
                        (
                            Slug::AdditionalDirections,
                            Contents::Document(&*bcp1979::baptism::ADDITIONAL_DIRECTIONS),
                        ),
                    ],
                },
            ),
            (
                Slug::Eucharist,
                Contents::Category {
                    label: "Eucharist".into(),
                    contents: vec![
                        (
                            Slug::Eucharist,
                            Contents::ByVersion {
                                label: "Holy Eucharist".into(),
                                documents: vec![&*rite2::eucharist::HOLY_EUCHARIST_II],
                            },
                        ),
                        (
                            Slug::PrayersOfThePeople,
                            Contents::MultiDocument {
                                label: "Prayers of the People".into(),
                                documents: bcp1979::eucharist::PRAYERS_OF_THE_PEOPLE.to_vec(),
                                hidden_in_toc: false,
                            },
                        ),
                        // Hidden
                        (
                            Slug::OffertorySentences,
                            Contents::MultiDocument {
                                label: "Offertory Sentences".into(),
                                documents: rite2::eucharist::OFFERTORY_SENTENCES_II.to_vec(),
                                hidden_in_toc: true,
                            },
                        ),
                    ],
                },
            ),
        ])
    }
}
