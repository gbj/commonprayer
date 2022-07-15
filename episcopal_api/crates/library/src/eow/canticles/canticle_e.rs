use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Reference, Source, Version};

lazy_static! {
    pub static ref CANTICLE_E: Document = Document::from(Canticle {
        number: CanticleId::CanticleE,
        changeable: None,
        citation: Some(String::from("Isaiah 66:10-14")),
        local_name: String::from("A Song of Jerusalem Our Mother"),
        latin_name: None,
        rubric: None,
        gloria_patri: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Rejoice with Jerusalem and be glad for her *",
                    "all you who love her,"
                )),
                CanticleVerse::from((
                    "Rejoice, rejoice with her, *",
                    "all you who mourn over her,"
                )),
                CanticleVerse::from((
                    "That you may drink deeply with delight *",
                    "from her comforting breast."
                )),
                CanticleVerse::from((
                    "For thus says our God, *",
                    "“I will extend peace to her like a river,
the wealth of nations like an overflowing stream."
                )),
                CanticleVerse::from((
                    "“You shall nurse and be carried on her arm, *",
                    "and you shall nestle in her lap."
                )),
                CanticleVerse::from((
                    "“As a mother comforts her child, so will I comfort you; *",
                    "you shall be comforted in Jerusalem."
                )),
                CanticleVerse::from((
                    "“You shall see, and your heart shall rejoice, *",
                    "you shall flourish like the grass of the fields.”"
                ))
            ]
        }]
    })
    .version(Version::EOW)
    .source(Reference {
        source: Source::EOW1,
        page: 32
    });
}
