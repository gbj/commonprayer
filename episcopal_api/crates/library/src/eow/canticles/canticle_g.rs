use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_G: Document = Document::from(Canticle {
        number: CanticleId::CanticleG,
        citation: Some(String::from("Ezekiel 36:24-28")),
        local_name: String::from("Canticle G"),
        latin_name: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "I will take you from among all nations; *",
                    "and gather you from all lands to bring you home."
                )),
                CanticleVerse::from((
                    "I will sprinkle clean water upon you; *",
                    "and purify you from false gods and uncleanness."
                )),
                CanticleVerse::from((
                    "A new heart I will give you *",
                    "and a new spirit put within you."
                )),
                CanticleVerse::from((
                    "I will take the stone heart from your chest *",
                    "and give you a heart of flesh."
                )),
                CanticleVerse::from((
                    "I will help you walk in my laws *",
                    "and cherish my commandments and do them."
                )),
                CanticleVerse::from(("You shall be my people, *", "and I will be your God."))
            ]
        }]
    })
    .version(Version::EOW);
}
