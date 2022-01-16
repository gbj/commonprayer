use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_E: Document = Document::from(Canticle {
        number: CanticleId::CanticleE,
        citation: Some(String::from("Isaiah 66:10-14")),
        local_name: String::from("Canticle E"),
        latin_name: None,
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
    ;
}
