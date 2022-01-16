use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_I: Document = Document::from(Canticle {
        number: CanticleId::CanticleI,
        citation: Some(String::from("Jonah 2:2-7,9")),
        local_name: String::from("Canticle I"),
        latin_name: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "I called to you, O God, out of my distress, and you answered me; *",
                    "out of the belly of Sheol I cried, and you heard my voice."
                )),
                CanticleVerse::from((
                    "You cast me into the deep, into the heart of the seas, *",
                    "and the flood surrounded me;
all your waves and billows passed over me."
                )),
                CanticleVerse::from((
                    "Then I said, “I am driven away from your sight; *",
                    "how shall I ever look again upon your holy temple?”"
                )),
                CanticleVerse::from((
                    "The waters closed in over me, the deep was round about me; *",
                    "weeds were wrapped around my head at the roots of the mountains."
                )),
                CanticleVerse::from((
                    "I went down to the land beneath the earth, *",
                    "yet you brought up my life from the depths, O God."
                )),
                CanticleVerse::from((
                    "As my life was ebbing away, I remembered you, O God, *",
                    "and my prayer came to you, into your holy temple."
                )),
                CanticleVerse::from((
                    "With the voice of thanksgiving, I will sacrifice to you; *",
                    "what I have vowed I will pay, for deliverance belongs to the Lord!"
                ))
            ]
        }]
    })
    .version(Version::EOW);
}
