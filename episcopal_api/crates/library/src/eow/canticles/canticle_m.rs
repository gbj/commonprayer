use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Reference, Source, Version};

lazy_static! {
    pub static ref CANTICLE_M: Document = Document::from(Canticle {
        number: CanticleId::CanticleM,
        changeable: None,
        citation: Some(String::from("1 Peter 1:3-4,18-21")),
        local_name: String::from("A Song of Faith"),
        latin_name: None,
        rubric: None,
        gloria_patri: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Blessed be the God and Father of our Lord Jesus Christ, *",
                    "by divine mercy we have a new birth into a living hope;"
                )),
                CanticleVerse::from((
                    "Through the resurrection of Jesus Christ from the dead, *",
                    "we have an inheritance that is imperishable in heaven."
                )),
                CanticleVerse::from((
                    "The ransom that was paid to free us *",
                    "was not paid in silver or gold,"
                )),
                CanticleVerse::from((
                    "But in the precious blood of Christ, *",
                    "the Lamb without spot or stain."
                )),
                CanticleVerse::from((
                    "God raised Jesus from the dead and gave him glory *",
                    "so that we might have faith and hope in God."
                ))
            ]
        }]
    })
    .version(Version::EOW)
    .source(Reference {
        source: Source::EOW1,
        page: 37
    });
}
