use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Reference, Source, Version};

lazy_static! {
    pub static ref CANTICLE_C: Document = Document::from(Canticle {
        number: CanticleId::CanticleC,
        changeable: None,
        citation: Some(String::from("1 Samuel 2:1-8")),
        local_name: String::from("The Song of Hannah"),
        latin_name: None,
        rubric: None,
        gloria_patri: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "My heart exults in you, O God; *",
                    "my triumph song is lifted in you."
                )),
                CanticleVerse::from((
                    "My mouth derides my enemies, *",
                    "for I rejoice in your salvation."
                )),
                CanticleVerse::from((
                    "There is none holy like you, *",
                    "nor any rock to be compared to you, our God."
                )),
                CanticleVerse::from((
                    "Do not heap up prideful words or speak in arrogance; *",
                    "Only God is knowing and weighs all actions."
                )),
                CanticleVerse::from((
                    "The bows of the mighty are broken, *",
                    "but the weak are clothed in strength."
                )),
                CanticleVerse::from((
                    "Those once full now labor for bread, *",
                    "those who hungered now are well fed."
                )),
                CanticleVerse::from((
                    "The childless woman has borne sevenfold, *",
                    "while the mother of many is forlorn."
                )),
                CanticleVerse::from((
                    "God destroys and brings to life, casts down and raises up; *",
                    "gives wealth or takes it away, humbles and dignifies."
                )),
                CanticleVerse::from((
                    "God raises the poor from the dust; *",
                    "and lifts the needy from the ash heap"
                )),
                CanticleVerse::from((
                    "To make them sit with the rulers *",
                    "and inherit a place of honor."
                )),
                CanticleVerse::from((
                    "For the pillars of the earth are God’s *",
                    "on which the whole earth is founded."
                ))
            ]
        }]
    })
    .version(Version::EOW)
    .source(Reference {
        source: Source::EOW1,
        page: 31
    });
}
