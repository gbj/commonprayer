use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_3: Document = Document::from(Canticle {
        number: CanticleId::Canticle3,
        citation: Some(String::from("Luke 1:46-55")),
        local_name: String::from("The Song of Mary"),
        latin_name: Some(String::from("Magnificat")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "My soul doth magnify the Lord, *",
                    "and my spirit hath rejoiced in God my Savior."
                )),
                CanticleVerse::from(("For he hath regarded *", "the lowliness of his handmaiden.")),
                CanticleVerse::from((
                    "For behold from henceforth *",
                    "all generations shall call me blessed."
                )),
                CanticleVerse::from((
                    "For he that is mighty hath magnified me, *",
                    "and holy is his Name."
                )),
                CanticleVerse::from((
                    "And his mercy is on them that fear him *",
                    "throughout all generations."
                )),
                CanticleVerse::from((
                    "He hath showed strength with his arm; *",
                    "he hath scattered the proud in the imagination of their hearts."
                )),
                CanticleVerse::from((
                    "He hath put down the mighty from their seat, *",
                    "and hath exalted the humble and meek."
                )),
                CanticleVerse::from((
                    "He hath filled the hungry with good things, *",
                    "and the rich he hath sent empty away."
                )),
                CanticleVerse::from((
                    "He remembering his mercy hath holpen his servant Israel, *",
                    "as he promised to our forefathers,
Abraham and his seed for ever."
                ))
            ]
        }]
    })
    .version(Version::RiteI);
}
