use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_4: Document = Document::from(Canticle {
        number: CanticleId::Canticle4,
        citation: Some(String::from("Luke 1: 68-79")),
        local_name: String::from("The Song of Zechariah"),
        latin_name: Some(String::from("Benedictus Dominus Deus")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Blessed be the Lord God of Israel, *",
                    "for he hath visited and redeemed his people;"
                )),
                CanticleVerse::from((
                    "And hath raised up a mighty salvation for us *",
                    "in the house of his servant David,"
                )),
                CanticleVerse::from((
                    "As he spake by the mouth of his holy prophets, *",
                    "which have been since the world began:"
                )),
                CanticleVerse::from((
                    "That we should be saved from our enemies, *",
                    "and from the hand of all that hate us;"
                )),
                CanticleVerse::from((
                    "To perform the mercy promised to our forefathers, *",
                    "and to remember his holy covenant;"
                )),
                CanticleVerse::from((
                    "To perform the oath which he sware to our forefather Abraham, *",
                    "that he would give us,"
                )),
                CanticleVerse::from((
                    "That we being delivered out of the hand of our enemies *",
                    "might serve him without fear,"
                )),
                CanticleVerse::from((
                    "In holiness and righteousness before him, *",
                    "all the days of our life."
                )),
                CanticleVerse::from((
                    "And thou, child, shalt be called the prophet of the Highest, *",
                    "for thou shalt go before the face of the Lord to prepare his ways;"
                )),
                CanticleVerse::from((
                    " To give knowledge of salvation unto his people *",
                    "for the remission of their sins,"
                )),
                CanticleVerse::from((
                    "Through the tender mercy of our God, *",
                    "whereby the dayspring from on high hath visited us;"
                )),
                CanticleVerse::from((
                    "To give light to them that sit in darkness
and in the shadow of death, *",
                    "and to guide our feet into the way of peace."
                ))
            ]
        }]
    })
    .version(Version::RiteI);
}
