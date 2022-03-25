use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use library::bcp1979::burial::parallels::BURIAL_PARALLEL_TAGS;
use library::marriage_alternatives::parallels::MARRIAGE_PARALLEL_TAGS;
use library::{self, CollectData, CollectId};
use liturgy::parallel_table::{build_parallel_table, ParallelDocument};
use liturgy::{Document, Heading, HeadingLevel, Rubric, Series, Version};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TOCLiturgy {
    MP,
    NP,
    EP,
    Compline,
    Eucharist,
    NotFound,
}

impl From<&str> for TOCLiturgy {
    fn from(s: &str) -> Self {
        match s {
            "morning-prayer" => TOCLiturgy::MP,
            "evening-prayer" => TOCLiturgy::EP,
            "noonday-prayer" => TOCLiturgy::NP,
            "compline" => TOCLiturgy::Compline,
            "eucharist" => TOCLiturgy::Eucharist,
            _ => TOCLiturgy::NotFound,
        }
    }
}

impl std::fmt::Display for TOCLiturgy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TOCLiturgy::MP => "morning-prayer",
                TOCLiturgy::NP => "noonday-prayer",
                TOCLiturgy::EP => "evening-prayer",
                TOCLiturgy::Compline => "compline",
                TOCLiturgy::Eucharist => "eucharist",
                TOCLiturgy::NotFound => "404",
            }
        )
    }
}

macro_rules! hash_map {
	($( $key: expr => $val: expr ),*) => {{
		 let mut map = ::std::collections::HashMap::new();
		 $( map.insert($key, $val); )*
		 map
	}}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum PageType<'a> {
    Document(&'a str, &'a Document),
    Category(&'a str, Version, Vec<Document>),
    Parallel(
        &'a str,
        &'a str,
        &'a str,
        Vec<Vec<(ParallelDocument, usize)>>,
    ),
}

lazy_static! {
    pub static ref TABLE_OF_CONTENTS: HashMap<(&'static str, Option<Version>), Vec<PageType<'static>>> = hash_map! {
        ("office", None) => vec![
           PageType::Document("morning-prayer", &*library::rite2::office::MORNING_PRAYER_II),
           PageType::Document("noonday-prayer", &*library::bcp1979::office::NOONDAY_PRAYER),
           PageType::Document("service-of-light", &*library::bcp1979::office::AN_ORDER_OF_WORSHIP_FOR_EVENING),
           PageType::Document("evening-prayer", &*library::rite2::office::EVENING_PRAYER_II),
           PageType::Document("compline", &*library::bcp1979::office::COMPLINE),
        ],
        ("canticle", None) => vec![
            PageType::Document("1", &*library::rite1::canticles::CANTICLE_1),
            PageType::Document("2", &*library::rite1::canticles::CANTICLE_2),
            PageType::Document("3", &*library::rite1::canticles::CANTICLE_3),
            PageType::Document("4", &*library::rite1::canticles::CANTICLE_4),
            PageType::Document("5", &*library::rite1::canticles::CANTICLE_5),
            PageType::Document("6", &*library::rite1::canticles::CANTICLE_6),
            PageType::Document("7", &*library::rite1::canticles::CANTICLE_7),
            PageType::Document("8", &*library::rite2::canticles::CANTICLE_8),
            PageType::Document("9", &*library::rite2::canticles::CANTICLE_9),
            PageType::Document("10", &*library::rite2::canticles::CANTICLE_10),
            PageType::Document("11", &*library::rite2::canticles::CANTICLE_11),
            PageType::Document("12", &*library::rite2::canticles::CANTICLE_12),
            PageType::Document("13", &*library::rite2::canticles::CANTICLE_13),
            PageType::Document("14", &*library::rite2::canticles::CANTICLE_14),
            PageType::Document("15", &*library::rite2::canticles::CANTICLE_15),
            PageType::Document("16", &*library::rite2::canticles::CANTICLE_16),
            PageType::Document("17", &*library::rite2::canticles::CANTICLE_17),
            PageType::Document("18", &*library::rite2::canticles::CANTICLE_18),
            PageType::Document("19", &*library::rite2::canticles::CANTICLE_19),
            PageType::Document("20", &*library::rite2::canticles::CANTICLE_20),
            PageType::Document("21", &*library::rite2::canticles::CANTICLE_21),
            PageType::Document("12-eow", &*library::eow::canticles::CANTICLE_12_EOW),
            PageType::Document("15-eow", &*library::eow::canticles::CANTICLE_15_EOW),
            PageType::Document("16-eow", &*library::eow::canticles::CANTICLE_16_EOW),
            PageType::Document("18-eow", &*library::eow::canticles::CANTICLE_18_EOW),
            PageType::Document("21-eow", &*library::eow::canticles::CANTICLE_21_EOW),
            PageType::Document("a", &*library::eow::canticles::CANTICLE_A),
            PageType::Document("b", &*library::eow::canticles::CANTICLE_B),
            PageType::Document("c", &*library::eow::canticles::CANTICLE_C),
            PageType::Document("d", &*library::eow::canticles::CANTICLE_D),
            PageType::Document("e", &*library::eow::canticles::CANTICLE_E),
            PageType::Document("f", &*library::eow::canticles::CANTICLE_F),
            PageType::Document("g", &*library::eow::canticles::CANTICLE_G),
            PageType::Document("h", &*library::eow::canticles::CANTICLE_H),
            PageType::Document("i", &*library::eow::canticles::CANTICLE_I),
            PageType::Document("j", &*library::eow::canticles::CANTICLE_J),
            PageType::Document("k", &*library::eow::canticles::CANTICLE_K),
            PageType::Document("l", &*library::eow::canticles::CANTICLE_L),
            PageType::Document("m", &*library::eow::canticles::CANTICLE_M),
            PageType::Document("n", &*library::eow::canticles::CANTICLE_N),
            PageType::Document("o", &*library::eow::canticles::CANTICLE_O),
            PageType::Document("p", &*library::eow::canticles::CANTICLE_P),
            PageType::Document("q", &*library::eow::canticles::CANTICLE_Q),
            PageType::Document("r", &*library::eow::canticles::CANTICLE_R),
            PageType::Document("s", &*library::eow::canticles::CANTICLE_S)
        ],
        ("collects", None) => vec![
            PageType::Category(
                "Collects: Contemporary",
                Version::RiteII,
                from_collects(library::rite2::collects::COLLECTS_CONTEMPORARY.iter())
            ),
            PageType::Category(
                "Collects: Traditional",
                Version::RiteI,
                from_collects(library::rite1::collects::COLLECTS_TRADITIONAL.iter())
            ),
            PageType::Category(
                "Colectas",
                Version::LibroDeOracionComun,
                from_collects(library::loc::collects::COLECTAS.iter())
            )
        ],
        ("opening-sentences", None) => vec![
            PageType::Category("Opening Sentences", Version::RiteII, library::rite2::OPENING_SENTENCES.clone())
        ],
        ("invitatory-antiphons", None) => vec![
            PageType::Category("Invitatory Antiphons", Version::RiteII, library::rite2::INVITATORY_ANTIPHONS.clone())
        ],
        ("closing-sentences", None) => vec![
            PageType::Category("Closing Sentences", Version::RiteII, library::rite2::CLOSING_SENTENCES.clone())
        ],
        ("prayers-and-thanksgivings", None) => vec![
            PageType::Category("Prayers and Thanksgivings", Version::BCP1979, library::bcp1979::PRAYERS_AND_THANKSGIVINGS.clone())
        ],
        ("marriage", None) => vec![
            // Parallels
            PageType::Document("readings", &*library::marriage_alternatives::parallels::PARALLEL_READINGS),
            PageType::Parallel("parallels", "Marriage Service Parallels",
                "In addition to the marriage service in the Book of Common Prayer (1979), the Episcopal Church has authorized several alternative marriage services, mainly in response to the need for gender-neutral language to describe the couple following the approval of same-sex marriage. This page is intended to show the parallels between the various authorized marriage services, noting differences as they arise.\n\nNote: “The Witnessing and Blessing of a Marriage,” as its own rite, differs in structure from the other services. Its prayers are here presented as alternatives to the traditional prayers, but the layout on this page does not reflect the exact order of materials in that rite.",
                build_parallel_table("marriage", MARRIAGE_PARALLEL_TAGS, &[
                    ("celebration-and-blessing-of-a-marriage", &*library::bcp1979::marriage::CELEBRATION_AND_BLESSING_OF_A_MARRIAGE),
                    ("celebration-and-blessing-of-a-marriage-2", &*library::marriage_alternatives::liturgical_resources_1::CELEBRATION_AND_BLESSING_OF_A_MARRIAGE_2),
                    ("witnessing-and-blessing-marriage", &*library::marriage_alternatives::liturgical_resources_1::WITNESSING_AND_BLESSING_OF_A_MARRIAGE),
                    ("an-order-for-marriage", &*library::bcp1979::marriage::AN_ORDER_FOR_MARRIAGE)
                ])
            ),

            // BCP
            PageType::Document("concerning-the-service", &*library::bcp1979::marriage::CONCERNING_THE_SERVICE),
            PageType::Document("celebration-and-blessing-of-a-marriage", &*library::bcp1979::marriage::CELEBRATION_AND_BLESSING_OF_A_MARRIAGE),
            PageType::Document("blessing-of-a-civil-marriage", &*library::bcp1979::marriage::BLESSING_OF_A_CIVIL_MARRIAGE),
            PageType::Document("an-order-for-marriage", &*library::bcp1979::marriage::AN_ORDER_FOR_MARRIAGE),
            PageType::Document("additional-directions", &*library::bcp1979::marriage::ADDITIONAL_DIRECTIONS),

            // Liturgical Resources 1
            PageType::Document("witnessing-and-blessing-lifelong-covenant", &*library::marriage_alternatives::liturgical_resources_1::WITNESSING_AND_BLESSING_OF_A_LIFELONG_COVENANT),
            PageType::Document("witnessing-and-blessing-marriage", &*library::marriage_alternatives::liturgical_resources_1::WITNESSING_AND_BLESSING_OF_A_MARRIAGE),
            PageType::Document("celebration-and-blessing-of-a-marriage-2", &*library::marriage_alternatives::liturgical_resources_1::CELEBRATION_AND_BLESSING_OF_A_MARRIAGE_2),
            PageType::Document("blessing-of-a-civil-marriage", &*library::marriage_alternatives::liturgical_resources_1::BLESSING_OF_A_CIVIL_MARRIAGE),
            PageType::Document("an-order-for-marriage", &*library::marriage_alternatives::liturgical_resources_1::AN_ORDER_FOR_MARRIAGE),
            PageType::Document("additional-directions", &*library::marriage_alternatives::liturgical_resources_1::ADDITIONAL_DIRECTIONS),

            // Liturgical Resources 2
            // mostly duplicates LR1, but changes elements of An Order for Marriage
            PageType::Document("an-order-for-marriage-2", &*library::marriage_alternatives::liturgical_resources_2::AN_ORDER_FOR_MARRIAGE_2),
        ],
        ("burial", None) => vec![
            /* PageType::Parallel("parallels", "Burial Service Parallels",
                "",
                build_parallel_table("burial", BURIAL_PARALLEL_TAGS, &[
                    ("burial-of-a-child", &*library::eow::volume_2::burial_of_a_child::BURIAL_OF_A_CHILD),
                    ("an-order-for-burial", &*library::bcp1979::burial::AN_ORDER_FOR_BURIAL)
                ])
            ), */

            // BCP
            PageType::Document("an-order-for-burial", &*library::bcp1979::burial::AN_ORDER_FOR_BURIAL),

            // EOW 2
            PageType::Document("burial-of-a-child", &*library::eow::volume_2::burial_of_a_child::BURIAL_OF_A_CHILD),
        ],
        ("additional-prayers", Some(Version::EOW)) => vec![
            PageType::Category("Additional Prayers", Version::EOW, library::eow::volume_2::burial_of_a_child::ADDITIONAL_PRAYERS_BURIAL_OF_A_CHILD.clone())
        ]
    };
}

fn from_collects<'a>(
    collects: impl Iterator<Item = &'a (CollectId, CollectData)>,
) -> Vec<Document> {
    let grouped_by_category = collects.group_by(|(_, data)| data.document.tags.get(0));
    grouped_by_category
        .into_iter()
        .flat_map(|(category, data)| {
            std::iter::once(Document::from(Heading::from((
                HeadingLevel::Heading2,
                category.cloned().unwrap_or_default(),
            ))))
            .chain(data.dedup_by(|a, b| a.1.document == b.1.document).map(
                |(_, data)| {
                    let mut pieces = Vec::new();

                    if let Some(text) = &data.rubric_before {
                        pieces.push(Document::from(Rubric::from(text.clone())))
                    }
                    pieces.push(Document {
                        label: None,
                        subtitle: None,
                        ..data.document.clone()
                    });
                    if !data.preface.is_empty() {
                        pieces.push(Document::from(Rubric::from(data.preface.clone())));
                    }
                    if let Some(text) = &data.rubric_after {
                        pieces.push(Document::from(Rubric::from(text.clone())))
                    }

                    let mut series = Document::from(Series::from(pieces));
                    series.label = data.document.label.clone();
                    series.subtitle = data.document.subtitle.clone();
                    series
                },
            ))
        })
        .collect()
}
