use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_5: Document = Document::from(Canticle {
        number: CanticleId::Canticle5,
        citation: Some(String::from("Luke 2:29-32")),
        local_name: String::from("The Song of Simeon"),
        latin_name: Some(String::from("Nunc Dimittis")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Lord, now lettest thou thy servant depart in peace, *",
                    "according to thy word;"
                )),
                CanticleVerse::from((
                    "For mine eyes have seen thy salvation, *",
                    "which thou hast prepared before the face of all people,"
                )),
                CanticleVerse::from((
                    "To be a light to lighten the Gentiles, *",
                    "and to be the glory of thy people Israel."
                ))
            ]
        }]
    })
    .version(Version::RiteI);
}
