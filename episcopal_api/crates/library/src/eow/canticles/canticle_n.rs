use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_N: Document = Document::from(Canticle {
        number: CanticleId::CanticleN,
        citation: Some(String::from("1 John 4:7-11")),
        local_name: String::from("Canticle N"),
        latin_name: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from(("Beloved, let us love one another, *", "for love is of God.")),
                CanticleVerse::from((
                    "Whoever does not love does not know God, *",
                    "for God is Love."
                )),
                CanticleVerse::from((
                    "In this the love of God was revealed among us, *",
                    "that God sent his only Son into the world,
so that we might live through Jesus Christ."
                )),
                CanticleVerse::from((
                    "In this is love, not that we loved God but that God loved us *",
                    "and sent his Son that sins might be forgiven."
                )),
                CanticleVerse::from((
                    "Beloved, since God loved us so much, *",
                    "we ought also to love one another."
                )),
                CanticleVerse::from((
                    "For if we love one another, God abides in us, *",
                    "and God’s love will be perfected in us."
                ))
            ]
        }]
    })
    .version(Version::EOW)
    ;
}
