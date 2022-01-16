use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_J: Document = Document::from(Canticle {
        number: CanticleId::CanticleJ,
        citation: Some(String::from("Judith 16:13-16")),
        local_name: String::from("Canticle J"),
        latin_name: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "I will sing a new song to my God, *",
                    "for you are great and glorious, wonderful in strength, invincible."
                )),
                CanticleVerse::from((
                    "Let the whole creation serve you, *",
                    "for you spoke and all things came into being."
                )),
                CanticleVerse::from((
                    "You sent your breath and it formed them, *",
                    "no one is able to resist your voice."
                )),
                CanticleVerse::from((
                    "Mountains and seas are stirred to their depths, *",
                    "rocks melt like wax at your presence."
                )),
                CanticleVerse::from((
                    "But to those who fear you, *",
                    "you continue to show mercy."
                )),
                CanticleVerse::from((
                    "No sacrifice, however fragrant, can please you, *",
                    "but whoever fears the Lord shall stand in your sight for ever."
                ))
            ]
        }]
    })
    .version(Version::EOW)
    ;
}
