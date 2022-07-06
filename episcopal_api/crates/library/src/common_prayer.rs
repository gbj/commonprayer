use canticle_table::{CanticleId, CanticleTable};
use lectionary::Lectionary;
use liturgy::{
    parallel_table::build_parallel_table, CanticleTables, Document, Lectionaries, SlugPath, Version,
};
use psalter::{bcp1979::BCP1979_PSALTER, Psalter};

use crate::{
    bcp1979, bos, eow, loc,
    marriage_alternatives::{self, parallels::MARRIAGE_PARALLEL_TAGS},
    rite1, rite2, Contents, Library, Section, Slug, TableOfContents,
};

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
                                    Slug::ServiceOfLight,
                                    Contents::Document(&*bcp1979::office::AN_ORDER_OF_WORSHIP_FOR_EVENING),
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
                                (Slug::SuggestedCanticles, Contents::Page { label: "Table of Suggested Canticles".into(), url: "canticle-table".into() }),
                                (
                                    Slug::Canticles,
                                    Contents::Sections {
                                        label: "Canticles".into(),
                                        contents: vec![
                                            Section {
                                                label: Some("Rite I".into()),
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
                                                ]
                                            },
                                            Section {
                                                label: Some("Rite II".into()),
                                                contents: vec![
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
                                                            label: "12. A Song of Creation".into(),
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
                                                            label: "15. The Song of Mary".into(),
                                                            documents: vec![
                                                                &*rite2::canticles::CANTICLE_15,
                                                                &*eow::canticles::CANTICLE_15_EOW,
                                                            ],
                                                        },
                                                    ),
                                                    (
                                                        Slug::Canticle(CanticleId::Canticle16),
                                                        Contents::ByVersion {
                                                            label: "16. The Song of Zechariah".into(),
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
                                                            label: "18. A Song to the Lamb".into(),
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
                                                            label: "21. You are God".into(),
                                                            documents: vec![
                                                                &*rite2::canticles::CANTICLE_21,
                                                                &*eow::canticles::CANTICLE_21_EOW,
                                                            ],
                                                        },
                                                    ),
                                                ]
                                            },
                                            Section {
                                                label: Some("Enriching our Worship 1".into()),
                                                contents: vec![
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
                                                ]
                                            }
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
                Slug::Collects,
                Contents::Category {
                    label: "The Collects".into(),
                    contents: vec![
                        (
                            Slug::Version(Version::RiteI),
                            Contents::from((
                                "The Collects: Traditional".to_string(),
                                &*rite1::collects::COLLECTS_TRADITIONAL.as_slice(),
                            )),
                        ),
                        (
                            Slug::Version(Version::RiteII),
                            Contents::from((
                                "The Collects: Contemporary".to_string(),
                                &*rite2::collects::COLLECTS_CONTEMPORARY.as_slice(),
                            )),
                        ),
                        (
                            Slug::Version(Version::LibroDeOracionComun),
                            Contents::from((
                                "Las Colectas".to_string(),
                                &*loc::collects::COLECTAS.as_slice(),
                            )),
                        ),
                    ],
                },
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
                Contents::Sections {
                    label: "Eucharist".into(),
                    contents: vec![
                        Section {
                            label: None,
                            contents: vec![
                                (
                                    Slug::Eucharist,
                                    Contents::ByVersion {
                                        label: "Holy Eucharist".into(),
                                        documents: vec![
                                            &*rite2::eucharist::HOLY_EUCHARIST_II,
                                            &*loc::eucharist::HOLY_EUCHARIST_II
                                        ],
                                    },
                                ),
                                (
                                    Slug::PrayersOfThePeople,
                                    Contents::Category {
                                        label: "Prayers of the People".into(),
                                        contents: vec![
                                            (
                                                Slug::Version(Version::BCP1979),
                                                Contents::MultiDocument {
                                                    label: "Book of Common Prayer (1979)".into(),
                                                    documents: bcp1979::eucharist::PRAYERS_OF_THE_PEOPLE.to_vec(),
                                                    hidden_in_toc: false,
                                                },
                                            ),
                                            (
                                                Slug::Version(Version::LibroDeOracionComun),
                                                Contents::MultiDocument {
                                                    label: "Libro de Oración Común".into(),
                                                    documents: loc::eucharist::PRAYERS_OF_THE_PEOPLE.to_vec(),
                                                    hidden_in_toc: false,
                                                },
                                            )
                                        ]
                                    }
                                    
                                ),
                                (
                                    Slug::GreatThanksgiving,
                                    Contents::Category {
                                        label: "Eucharistic Prayers".into(),
                                        contents: vec![
                                            (
                                                Slug::Version(Version::RiteI),
                                                Contents::Category {
                                                    label: "Rite I".into(),
                                                    contents: vec![
                                                        (Slug::PrayerI, Contents::Document(&*rite1::eucharist::PRAYER_I)),
                                                        (Slug::PrayerII, Contents::Document(&*rite1::eucharist::PRAYER_II)),
                                                    ]
                                                }
                                            ),
                                            (
                                                Slug::Version(Version::RiteII),
                                                Contents::Category {
                                                    label: "Rite II".into(),
                                                    contents: vec![
                                                        (Slug::PrayerA, Contents::Document(&*rite2::eucharist::PRAYER_A)),
                                                        (Slug::PrayerB, Contents::Document(&*rite2::eucharist::PRAYER_B)),
                                                        (Slug::PrayerC, Contents::Document(&*rite2::eucharist::PRAYER_C)),
                                                        (Slug::PrayerD, Contents::Document(&*rite2::eucharist::PRAYER_D))
                                                    ]
                                                }
                                            ),
                                            (
                                                Slug::Version(Version::EOW),
                                                Contents::Category {
                                                    label: "Enriching Our Worship".into(),
                                                    contents: vec![
                                                        (Slug::Prayer1, Contents::Document(&*eow::eucharist::PRAYER_1)),
                                                        (Slug::Prayer2, Contents::Document(&*eow::eucharist::PRAYER_2)),
                                                        (Slug::Prayer3, Contents::Document(&*eow::eucharist::PRAYER_3)),
                                                    ]
                                                }
                                            ),
                                            (
                                                Slug::Version(Version::LibroDeOracionComun),
                                                Contents::Category {
                                                    label: "Libro de Oración Común".into(),
                                                    contents: vec![
                                                        (Slug::PrayerA, Contents::Document(&*loc::eucharist::PRAYER_A)),
                                                        (Slug::PrayerB, Contents::Document(&*loc::eucharist::PRAYER_B)),
                                                        (Slug::PrayerC, Contents::Document(&*loc::eucharist::PRAYER_C)),
                                                        (Slug::PrayerD, Contents::Document(&*loc::eucharist::PRAYER_D))
                                                    ]
                                                }
                                            ),
                                        ]
                                    }
                                ),
                                (
                                    Slug::ProperPrefaces,
                                    Contents::MultiDocument { 
                                        label: "Proper Prefaces".into(),
                                        documents: vec![
                                        ],
                                        hidden_in_toc: false
                                    }
                                )
                            ]
                        },
                        Section {
                            label: None,
                            contents: vec![
                                (
                                    Slug::OffertorySentences,
                                    Contents::Category {
                                        label: "Offertory Sentences".into(),
                                        contents: vec![
                                            (
                                                Slug::Version(Version::RiteII),
                                                Contents::MultiDocument {
                                                    label: "Rite II".into(),
                                                    documents: rite2::eucharist::OFFERTORY_SENTENCES_II.to_vec(),
                                                    hidden_in_toc: false,
                                                }
                                            ),
                                            (
                                                Slug::Version(Version::LibroDeOracionComun),
                                                Contents::MultiDocument {
                                                    label: "Libro de Oración Común".into(),
                                                    documents: loc::eucharist::OFFERTORY_SENTENCES_LOC.to_vec(),
                                                    hidden_in_toc: false,
                                                }
                                            )
                                        ]
                                    }
                                ),
                                (
                                    Slug::PenitentialSentences,
                                    Contents::Category {
                                        label: "Penitential Sentences".into(),
                                        contents: vec![
                                            (
                                                Slug::Version(Version::RiteII),
                                                Contents::MultiDocument {
                                                    label: "Rite II".into(),
                                                    documents: rite2::eucharist::PENITENTIAL_SENTENCES_II.to_vec(),
                                                    hidden_in_toc: false,
                                                }
                                            ),
                                            (
                                                Slug::Version(Version::LibroDeOracionComun),
                                                Contents::MultiDocument {
                                                    label: "Libro de Oración Común".into(),
                                                    documents: loc::eucharist::PENITENTIAL_SENTENCES_LOC.to_vec(),
                                                    hidden_in_toc: false,
                                                }
                                            )
                                        ]
                                    }
                                ),
                                (Slug::ConsecratingAdditional, Contents::Document(&*bcp1979::eucharist::CONSECRATING_ADDITIONAL))        
                            ]
                        },
                        Section {
                            label: None,
                            contents: vec![(
                                Slug::Parallels,
                                Contents::Category {
                                    label: "Parallels".into(),
                                    contents: vec![
                                        (
                                            Slug::Version(Version::LibroDeOracionComun),
                                            Contents::Document(&*loc::eucharist::RITE_II_LOC_EUCHARIST_PARALLEL)
                                        )
                                    ]
                                }
                            )]
                        }                       
                    ],
                },
            ),
            (
                Slug::PastoralOffices,
                Contents::Category {
                    label: "Pastoral Offices".into(),
                    contents: vec![
                        (
                            Slug::Marriage,
                            Contents::Sections {
                                label: "Marriage".into(),
                                contents: vec![
                                    Section {
                                        label: Some("Parallels".into()),
                                        contents: vec![(
                                            Slug::Readings,
                                            Contents::Document(
                                                &*marriage_alternatives::parallels::PARALLEL_READINGS,
                                            ),
                                        ), (
                                            Slug::Parallels,
                                            Contents::Parallels {
                                                label: "Marriage Service Parallels".into(),
                                                intro: "In addition to the marriage service in the Book of Common Prayer (1979), the Episcopal Church has authorized several alternative marriage services, mainly in response to the need for gender-neutral language to describe the couple following the approval of same-sex marriage. This page is intended to show the parallels between the various authorized marriage services, noting differences as they arise.\n\nNote: “The Witnessing and Blessing of a Marriage,” as its own rite, differs in structure from the other services. Its prayers are here presented as alternatives to the traditional prayers, but the layout on this page does not reflect the exact order of materials in that rite.".into(),
                                                parallels: build_parallel_table(
                                                    MARRIAGE_PARALLEL_TAGS,
                                                    &[
                                                        (&SlugPath::from([Slug::PastoralOffices, Slug::Marriage, Slug::CelebrationAndBlessing, Slug::Version(Version::BCP1979)]), &*bcp1979::marriage::CELEBRATION_AND_BLESSING_OF_A_MARRIAGE),
                                                        (&SlugPath::from([Slug::PastoralOffices, Slug::Marriage, Slug::CelebrationAndBlessing, Slug::Version(Version::Expansive)]), &*marriage_alternatives::liturgical_resources_1::CELEBRATION_AND_BLESSING_OF_A_MARRIAGE_2),
                                                        (&SlugPath::from([Slug::PastoralOffices, Slug::Marriage, Slug::WitnessingAndBlessing]), &*marriage_alternatives::liturgical_resources_1::WITNESSING_AND_BLESSING_OF_A_MARRIAGE),
                                                        (&SlugPath::from([Slug::PastoralOffices, Slug::Marriage, Slug::Order]), &*bcp1979::marriage::AN_ORDER_FOR_MARRIAGE)
                                                    ]
                                                )
                                            },
                                        )],
                                    },
                                    Section {
                                        label: Some("Marriage Services".into()),
                                        contents: vec![
                                            (Slug::ConcerningTheService, Contents::Document(&*bcp1979::marriage::CONCERNING_THE_SERVICE)),
                                        ]
                                    },
                                    Section {
                                        label: None,
                                        contents: vec![
                                            (
                                                Slug::CelebrationAndBlessing,
                                                Contents::ByVersion {
                                                    label: "The Celebration and Blessing of a Marriage".into(),
                                                    documents: vec![
                                                        &*bcp1979::marriage::CELEBRATION_AND_BLESSING_OF_A_MARRIAGE,
                                                        &*marriage_alternatives::liturgical_resources_1::CELEBRATION_AND_BLESSING_OF_A_MARRIAGE_2
                                                    ]
                                                }
                                            ),
                                            (Slug::WitnessingAndBlessing, Contents::Document(&*marriage_alternatives::liturgical_resources_1::WITNESSING_AND_BLESSING_OF_A_MARRIAGE)),
                                            (Slug::WitnessingAndBlessingLifelongCovenant, Contents::Document(&*marriage_alternatives::liturgical_resources_1::WITNESSING_AND_BLESSING_OF_A_LIFELONG_COVENANT)),
                                        ]
                                    },
                                    Section {
                                        label: None,
                                        contents: vec![
                                            (
                                                Slug::CivilMarriage,
                                                Contents::ByVersion {
                                                    label: "The Blessing of a Civil Marriage".into(),
                                                    documents: vec![
                                                        &*bcp1979::marriage::BLESSING_OF_A_CIVIL_MARRIAGE,
                                                        &*marriage_alternatives::liturgical_resources_1::BLESSING_OF_A_CIVIL_MARRIAGE
                                                    ]
                                                }
                                            ),
                                        ]
                                    },
                                    Section {
                                        label: None,
                                        contents: vec![
                                            (
                                                Slug::Order,
                                                Contents::ByVersion {
                                                    label: "An Order for Marriage".into(),
                                                    documents: vec![
                                                        &*bcp1979::marriage::AN_ORDER_FOR_MARRIAGE,
                                                        &*marriage_alternatives::liturgical_resources_1::AN_ORDER_FOR_MARRIAGE,
                                                        &*marriage_alternatives::liturgical_resources_2::AN_ORDER_FOR_MARRIAGE_2
                                                    ]
                                                }
                                            ),
                                            (
                                                Slug::AdditionalDirections,
                                                Contents::ByVersion {
                                                    label: "Additional Directions".into(),
                                                    documents: vec![
                                                        &*bcp1979::marriage::ADDITIONAL_DIRECTIONS,
                                                        &*marriage_alternatives::liturgical_resources_1::ADDITIONAL_DIRECTIONS
                                                    ]
                                                }
                                            ),
                                        ]
                                    }
                                ],
                            },
                        ),
                        (
                            Slug::Burial,
                            Contents::Sections {
                                label: "The Burial of the Dead".into(),
                                contents: vec![
                                    Section {
                                        label: Some("Parallels".into()),
                                        contents: vec![
                                            (Slug::Readings, Contents::Document(&*bcp1979::burial::parallels::PARALLEL_READINGS)),
                                            (
                                                Slug::Parallels,
                                                Contents::Parallels {
                                                    label: "Burial Service Parallels".into(),
                                                    intro: "".into(),
                                                    parallels: build_parallel_table(
                                                        bcp1979::burial::parallels::BURIAL_PARALLEL_TAGS,
                                                        &[
                                                            (&SlugPath::from([Slug::Burial, Slug::Burial]), &*rite2::burial::BURIAL_RITE_II),
                                                            (&SlugPath::from([Slug::Burial, Slug::BurialOfAChild]), &*eow::volume_2::burial_of_a_child::BURIAL_OF_A_CHILD),
                                                            (&SlugPath::from([Slug::Burial, Slug::BurialOfANonChristian]), &*bos::BURIAL_OF_A_NON_CHRISTIAN),
                                                            (&SlugPath::from([Slug::Burial, Slug::Order]), &*bcp1979::burial::AN_ORDER_FOR_BURIAL)
                                                        ]
                                                    )
                                                }
                                            )
                                        ]
                                    },
                                    Section {
                                        label: Some("Book of Common Prayer (1979)".into()),
                                        contents: vec![
                                            (Slug::ConcerningTheService, Contents::Document(&*bcp1979::burial::CONCERNING_THE_BURIAL_SERVICE)),
                                            (Slug::Burial, Contents::ByVersion {
                                                label: "The Burial of the Dead".into(),
                                                documents: vec![
                                                    &*rite1::burial::BURIAL_RITE_I,
                                                    &*rite2::burial::BURIAL_RITE_II
                                                ]
                                            }),
                                            (Slug::Order, Contents::Document(&*bcp1979::burial::AN_ORDER_FOR_BURIAL))
                                        ]
                                    },
                                    Section {
                                        label: Some("Enriching Our Worship 2 (2000)".into()),
                                        contents: vec![
                                            (Slug::BurialOfAChild, Contents::Document(&*eow::volume_2::burial_of_a_child::BURIAL_OF_A_CHILD)),
                                        ]
                                    },
                                    Section {
                                        label: Some("Book of Occasional Services (2018)".into()),
                                        contents: vec![
                                            (Slug::BurialOfANonChristian, Contents::Document(&*bos::BURIAL_OF_A_NON_CHRISTIAN))
                                        ]
                                    },
                                    Section {
                                        label: Some("Additional Prayers".into()),
                                        contents: vec![
                                            (
                                                Slug::Version(Version::RiteI),
                                                    Contents::MultiDocument {
                                                    label: "Rite I".into(),
                                                    documents: rite1::burial::ADDITIONAL_PRAYERS_BURIAL_I.to_vec(),
                                                    hidden_in_toc: false
                                                }
                                            ),
                                            (
                                                Slug::Version(Version::RiteII),
                                                    Contents::MultiDocument {
                                                    label: "Rite II".into(),
                                                    documents: rite2::burial::ADDITIONAL_PRAYERS_BURIAL.to_vec(),
                                                    hidden_in_toc: false
                                                }
                                            ),
                                            (
                                                Slug::Version(Version::EOW),
                                                    Contents::MultiDocument {
                                                    label: "For the Burial of a Child".into(),
                                                    documents: eow::volume_2::burial_of_a_child::ADDITIONAL_PRAYERS_BURIAL_OF_A_CHILD.to_vec(),
                                                    hidden_in_toc: false
                                                }
                                            )
                                        ]
                                    }
                                ]
                            },
                        ),
                    ],
                },
            ),
             (
                            Slug::OccasionalServices,
                            Contents::Category {
                                label: "Occasional Services".into(),
                                contents: vec![
                                    (Slug::Guadalupe, Contents::Document(&*bos::OUR_LADY_OF_GUADALUPE)),
                                    (Slug::Renaming, Contents::Document(&*bos::A_SERVICE_OF_RENAMING))
                                ]
                            }
                        ),
                        (
                            Slug::Creeds,
                            Contents::Category {
                                label: "Creeds".into(),
                                contents: vec![
                                    (
                                        Slug::ApostlesCreed,
                                        Contents::ByVersion {
                                            label: "Apostles’ Creed".into(),
                                            documents: vec![
                                                &*rite1::APOSTLES_CREED_TRADITIONAL
                                            ]
                                        }
                                    ),
                                    (
                                        Slug::NiceneCreed,
                                        Contents::ByVersion {
                                            label: "Nicene Creed".into(),
                                            documents: vec![
                                                &*rite2::eucharist::NICENE_CREED_II
                                            ]
                                        }
                                    ),
                                ]
                            }
                        )
        ])
    }
}
