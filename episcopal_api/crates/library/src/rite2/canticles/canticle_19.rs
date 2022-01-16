use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_19: Document = Document::from(Canticle {
        number: CanticleId::Canticle19,
        citation: Some(String::from("Revelation 15:3-4")),
        local_name: String::from("The Song of the Redeemed"),
        latin_name: Some(String::from("Magna et mirabilia")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "O ruler of the universe, Lord God,
great deeds are they that you have done, *",
                    "surpassing human understanding."
                )),
                CanticleVerse::from((
                    "Your ways are ways of righteousness and truth, *",
                    "O King of all the ages."
                )),
                CanticleVerse::from((
                    "Who can fail to do you homage, Lord,
and sing the praises of your Name? *",
                    "for you only are the Holy One."
                )),
                CanticleVerse::from((
                    "All nations will draw near and fall down before you, *",
                    "because your just and holy works have been revealed."
                ))
            ]
        }]
    })
    .version(Version::RiteII);
}
