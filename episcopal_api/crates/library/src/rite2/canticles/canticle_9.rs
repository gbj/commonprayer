use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

use crate::rite2::GLORIA_PATRI;

lazy_static! {
    pub static ref CANTICLE_9: Document = Document::from(Canticle {
        number: CanticleId::Canticle9,
        changeable: None,
        citation: Some(String::from("Isaiah 12:2-6")),
        local_name: String::from("The First Song of Isaiah"),
        latin_name: Some(String::from("Ecce, Deus")),
        rubric: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Surely, it is God who saves me; *",
                    "I will trust in him and not be afraid."
                )),
                CanticleVerse::from((
                    "For the Lord is my stronghold and my sure defense, *",
                    "and he will be my Savior."
                )),
                CanticleVerse::from((
                    "Therefore you shall draw water with rejoicing *",
                    "from the springs of salvation."
                )),
                CanticleVerse::from((
                    "And on that day you shall say, *",
                    "Give thanks to the Lord and call upon his Name;"
                )),
                CanticleVerse::from((
                    "Make his deeds known among the peoples; *",
                    "see that they remember that his Name is exalted."
                )),
                CanticleVerse::from((
                    "Sing praises of the Lord, for he has done great things, *",
                    "and this is known in all the world."
                )),
                CanticleVerse::from((
                    "Cry aloud, inhabitants of Zion, ring out your joy, *",
                    "for the great one in the midst of you is the Holy One of Israel."
                ))
            ]
        }],
        gloria_patri: Some(GLORIA_PATRI.clone())
    })
    .version(Version::RiteII)
    .page(86);
}
