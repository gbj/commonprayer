use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Reference, Source, Version};

lazy_static! {
    pub static ref CANTICLE_A: Document = Document::from(Canticle {
        number: CanticleId::CanticleA,
        changeable: None,
        citation: Some(String::from("Wisdom 10:15-19,20b-21")),
        local_name: String::from("A Song of Wisdom"),
        latin_name: Some(String::from("Sapientia liberavit")),
        rubric: None,
        gloria_patri: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Wisdom freed from a nation of oppressors *",
                    "a holy people and a blameless race."
                )),
                CanticleVerse::from((
                    "She entered the soul of a servant of the Lord, *",
                    "withstood dread rulers with wonders and signs."
                )),
                CanticleVerse::from((
                    "To the saints she gave the reward of their labors, *",
                    "and led them by a marvelous way;"
                )),
                CanticleVerse::from((
                    "She was their shelter by day *",
                    "and a blaze of stars by night."
                )),
                CanticleVerse::from((
                    "She brought them across the Red Sea, *",
                    "she led them through mighty waters;"
                )),
                CanticleVerse::from((
                    "But their enemies she swallowed in the waves *",
                    "and spewed them out from the depths of the abyss."
                )),
                CanticleVerse::from((
                    "And then, Lord, the righteous sang hymns to your Name, *",
                    "and praised with one voice your protecting hand;"
                )),
                CanticleVerse::from((
                    "For Wisdom opened the mouths of the mute, *",
                    "and gave speech to the tongues of a new-born people."
                ))
            ]
        }]
    })
    .version(Version::EOW)
    .source(Reference {
        source: Source::EOW1,
        page: 30
    });
}
