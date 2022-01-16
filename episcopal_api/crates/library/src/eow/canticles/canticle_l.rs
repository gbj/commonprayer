use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_L: Document = Document::from(Canticle {
        number: CanticleId::CanticleL,
        citation: Some(String::from("Philippians 2:6-11")),
        local_name: String::from("Canticle L"),
        latin_name: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Though in the form of God, *",
                    "Christ Jesus did not cling to equality with God, "
                )),
                CanticleVerse::from((
                    "But emptied himself, taking the form of a servant, *",
                    "and was born in human likeness."
                )),
                CanticleVerse::from((
                    "Being found in human form, he humbled himself *",
                    "and became obedient to death, even death on a cross."
                )),
                CanticleVerse::from((
                    "Therefore, God has highly exalted him *",
                    "and given him the name above every name,"
                )),
                CanticleVerse::from((
                    "That at the name of Jesus, every knee shall bow, *",
                    "in heaven and on earth and under the earth,"
                )),
                CanticleVerse::from((
                    "And every tongue confess that Jesus Christ is Lord, *",
                    "to the glory of God the Father."
                ))
            ]
        }]
    })
    .version(Version::EOW);
}
