use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_18_EOW: Document = Document::from(Canticle {
        number: CanticleId::Canticle18,
        citation: Some(String::from("Revelation 4:11, 5:9-10, 13")),
        local_name: String::from("A Song to the Lamb"),
        latin_name: Some(String::from("Dignus es")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Splendor and honor and royal power *",
                    "are yours by right, O God Most High,"
                )),
                CanticleVerse::from((
                    "For you created everything that is, *",
                    "and by your will they were created and have their being;"
                )),
                CanticleVerse::from((
                    "And yours by right, O Lamb that was slain, *",
                    "for with your blood you have redeemed for God,"
                )),
                CanticleVerse::from((
                    "From every family, language, people, and nation, *",
                    "a royal priesthood to serve our God."
                )),
                CanticleVerse::from((
                    "And so, to the One who sits upon the throne, *",
                    "and to Christ the Lamb,"
                )),
                CanticleVerse::from((
                    "Be worship and praise, dominion and splendor, *",
                    "for ever and for evermore."
                ))
            ]
        }]
    })
    .version(Version::EOW);
}
