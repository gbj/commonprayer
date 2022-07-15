use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

use crate::rite2::GLORIA_PATRI;

lazy_static! {
    pub static ref CANTICLE_17: Document = Document::from(Canticle {
        number: CanticleId::Canticle17,
        changeable: None,
        citation: Some(String::from("Luke 2:29-32")),
        local_name: String::from("The Song of Simeon"),
        latin_name: Some(String::from("Nunc Dimittis")),
        rubric: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Lord, you now have set your servant free *",
                    "to go in peace as you have promised;"
                )),
                CanticleVerse::from((
                    "For these eyes of mine have seen the Savior, *",
                    "whom you have prepared for all the world to see:"
                )),
                CanticleVerse::from((
                    "A Light to enlighten the nations, *",
                    "and the glory of your people Israel."
                ))
            ]
        }],
        gloria_patri: Some(GLORIA_PATRI.clone()),
    })
    .version(Version::RiteII)
    .page(93);
}
