use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Reference, Source, Version};

lazy_static! {
    pub static ref CANTICLE_P: Document = Document::from(Canticle {
        number: CanticleId::CanticleP,
        changeable: None,
        citation: Some(String::from("Revelation 22:12-17")),
        local_name: String::from("A Song of the Spirit"),
        latin_name: None,
        rubric: None,
        gloria_patri: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "“Behold, I am coming soon,” says the Lord,
“and bringing my reward with me, *",
                    "to give to everyone according to their deeds."
                )),
                CanticleVerse::from((
                    "“I am the Alpha and the Omega, the first and the last, *",
                    "the beginning and the end.”"
                )),
                CanticleVerse::from((
                    "Blessed are those who do God’s commandments,
that they may have the right to the tree of life, *",
                    "and may enter the city through the gates."
                )),
                CanticleVerse::from((
                    "“I, Jesus, have sent my angel to you, *",
                    "with this testimony for all the churches."
                )),
                CanticleVerse::from((
                    "“I am the root and the offspring of David, *",
                    "I am the bright morning star.”"
                )),
                CanticleVerse::from((
                    "“Come!” say the Spirit and the Bride; *",
                    "“Come!” let each hearer reply!"
                )),
                CanticleVerse::from((
                    "Come forward, you who are thirsty, *",
                    "let those who desire take the water of life as a gift."
                ))
            ]
        }]
    })
    .version(Version::EOW)
    .source(Reference {
        source: Source::EOW1,
        page: 38
    });
}
