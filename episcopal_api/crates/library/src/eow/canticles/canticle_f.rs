use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_F: Document = Document::from(Canticle {
        number: CanticleId::CanticleF,
        citation: Some(String::from("Lamentations 1:12,16; 3:19,22-24,26")),
        local_name: String::from("Canticle F"),
        latin_name: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Is it nothing to you, all you who pass by? *",
                    "Look and see if there is any sorrow like my sorrow,"
                )),
                CanticleVerse::from((
                    "Which was brought upon me, *",
                    "inflicted by God’s fierce anger."
                )),
                CanticleVerse::from((
                    "For these things I weep; my eyes flow with tears, *",
                    "for a comforter is far from me, one to revive my courage."
                )),
                CanticleVerse::from((
                    "Remember my affliction and my bitterness, *",
                    "wormwood and gall!"
                )),
                CanticleVerse::from((
                    "The steadfast love of God never ceases, *",
                    "God’s mercies never end."
                )),
                CanticleVerse::from((
                    "They are new every morning; *",
                    "great is your faithfulness."
                )),
                CanticleVerse::from((
                    "“God is my portion,” says my soul, *",
                    "“therefore will I hope in God.”"
                )),
                CanticleVerse::from((
                    "It is good that we should wait quietly *",
                    "for the coming of God’s salvation."
                ))
            ]
        }]
    })
    .version(Version::EOW);
}
