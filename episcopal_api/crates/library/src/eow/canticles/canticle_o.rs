use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Reference, Source, Version};

lazy_static! {
    pub static ref CANTICLE_O: Document = Document::from(Canticle {
        number: CanticleId::CanticleO,
        changeable: None,
        citation: Some(String::from("Revelation 21:22-26, 22:1-4")),
        local_name: String::from("A Song of the Heavenly City"),
        latin_name: None,
        rubric: None,
        gloria_patri: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "I saw no temple in the city, *",
                    "for its temple is the God of surpassing strength and the Lamb."
                )),
                CanticleVerse::from((
                    "And the city has no need of sun or moon to light it, *",
                    "for the glory of God shines on it, and its lamp is the Lamb."
                )),
                CanticleVerse::from((
                    "By its light the nations shall walk, *",
                    "and the rulers of the world lay their honor and glory there."
                )),
                CanticleVerse::from((
                    "Its gates shall never be shut by day, nor shall there be any night; *",
                    "into it they will bring the honor and glory of nations."
                )),
                CanticleVerse::from((
                    "I saw the clean river of the water of life, bright as crystal, *",
                    "flowing from the throne of God and of the Lamb."
                )),
                CanticleVerse::from((
                    "The tree of life spanned the river, giving fruit every month, *",
                    "and the leaves of the tree were for the healing of nations."
                )),
                CanticleVerse::from((
                    "All curses cease where the throne of God and the Lamb stands,
and all servants give worship there; *",
                    "there they will see God’s face, whose Name shall be on their foreheads."
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
