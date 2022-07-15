use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_18: Document = Document::from(Canticle {
        number: CanticleId::Canticle18,
        changeable: None,
        citation: Some(String::from("Revelation 4:11, 5:9-10, 13")),
        local_name: String::from("A Song to the Lamb"),
        latin_name: Some(String::from("Dignus es")),
        rubric: None,
        sections: vec![
            CanticleSection {
                title: None,
                verses: vec![
                    CanticleVerse::from((
                        "Splendor and honor and kingly power *",
                        "are yours by right, O Lord our God,"
                    )),
                    CanticleVerse::from((
                        "For you created everything that is, *",
                        "and by your will they were created and have their being;"
                    )),
                ]
            },
            CanticleSection {
                title: None,
                verses: vec![
                    CanticleVerse::from((
                        "And yours by right, O Lamb that was slain, *",
                        "for with your blood you have redeemed for God,"
                    )),
                    CanticleVerse::from((
                        "From every family, language, people, and nation, *",
                        "a kingdom of priests to serve our God."
                    )),
                ]
            },
            CanticleSection {
                title: None,
                verses: vec![
                    CanticleVerse::from((
                        "And so, to him who sits upon the throne, *",
                        "and to Christ the Lamb,"
                    )),
                    CanticleVerse::from((
                        "Be worship and praise, dominion and splendor, *",
                        "for ever and for evermore."
                    ))
                ]
            }
        ],
        gloria_patri: None,
    })
    .version(Version::RiteII)
    .page(93);
}
