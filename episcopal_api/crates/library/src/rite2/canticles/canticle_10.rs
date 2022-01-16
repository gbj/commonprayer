use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_10: Document = Document::from(Canticle {
        number: CanticleId::Canticle10,
        citation: Some(String::from("Isaiah 55:6-11")),
        local_name: String::from("The Second Song of Isaiah"),
        latin_name: Some(String::from("Quaerite Dominum")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Seek the Lord while he wills to be found; *",
                    "call upon him when he draws near."
                )),
                CanticleVerse::from((
                    "Let the wicked forsake their ways *",
                    "and the evil ones their thoughts;"
                )),
                CanticleVerse::from((
                    "And let them turn to the Lord, and he will have compassion, *",
                    "and to our God, for he will richly pardon."
                )),
                CanticleVerse::from((
                    "For my thoughts are not your thoughts, *",
                    "nor your ways my ways, says the Lord."
                )),
                CanticleVerse::from((
                    "For as the heavens are higher than the earth, *",
                    "so are my ways higher than your ways,
and my thoughts than your thoughts."
                )),
                CanticleVerse::from((
                    "For as rain and snow fall from the heavens *",
                    "and return not again, but water the earth,"
                )),
                CanticleVerse::from((
                    "Bringing forth life and giving growth, *",
                    "seed for sowing and bread for eating,"
                )),
                CanticleVerse::from((
                    "So is my word that goes forth from my mouth; *",
                    "it will not return to me empty;"
                )),
                CanticleVerse::from((
                    "But it will accomplish that which I have purposed, *",
                    "and prosper in that for which I sent it."
                ))
            ]
        }]
    })
    .version(Version::RiteII);
}
