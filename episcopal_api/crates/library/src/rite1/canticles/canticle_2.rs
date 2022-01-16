use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_2: Document = Document::from(Canticle {
        number: CanticleId::Canticle2,
        citation: Some(String::from("Song of the Three Young Men, 29-34")),
        local_name: String::from("A Song of Praise"),
        latin_name: Some(String::from("Benedictus es, Domine")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Blessed art thou, O Lord God of our fathers; *",
                    "praised and exalted above all for ever."
                )),
                CanticleVerse::from((
                    "Blessed art thou for the name of thy Majesty; *",
                    "praised and exalted above all for ever."
                )),
                CanticleVerse::from((
                    "Blessed art thou in the temple of thy holiness; *",
                    "praised and exalted above all for ever."
                )),
                CanticleVerse::from((
                    "Blessed art thou that beholdest the depths,
and dwellest between the Cherubim; *",
                    "praised and exalted above all for ever."
                )),
                CanticleVerse::from((
                    "Blessed art thou on the glorious throne of thy kingdom; *",
                    "praised and exalted above all for ever."
                )),
                CanticleVerse::from((
                    "Blessed art thou in the firmament of heaven; *",
                    "praised and exalted above all for ever."
                )),
                CanticleVerse::from((
                    "Blessed art thou, O Father, Son, and Holy Spirit; *",
                    "praised and exalted above all for ever."
                ))
            ]
        }]
    })
    .version(Version::RiteI);
}
