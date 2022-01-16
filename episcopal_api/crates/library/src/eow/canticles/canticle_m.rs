use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_M: Document = Document::from(Canticle {
        number: CanticleId::CanticleM,
        citation: Some(String::from("1 Peter 1:3-4,18-21")),
        local_name: String::from("Canticle M"),
        latin_name: None,
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
    ;
}
