use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_13: Document = Document::from(Canticle {
        number: CanticleId::Canticle13,
        citation: Some(String::from("Song of the Three Young Men, 29-34")),
        local_name: String::from("A Song of Praise"),
        latin_name: Some(String::from("Benedictus es, Domine")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Glory to you, Lord God of our fathers; *",
                    "you are worthy of praise; glory to you."
                )),
                CanticleVerse::from((
                    "Glory to you for the radiance of your holy Name; *",
                    "we will praise you and highly exalt you for ever."
                )),
                CanticleVerse::from((
                    "Glory to you in the splendor of your temple; *",
                    "on the throne of your majesty, glory to you."
                )),
                CanticleVerse::from((
                    "Glory to you, seated between the Cherubim; *",
                    "we will praise you and highly exalt you for ever."
                )),
                CanticleVerse::from((
                    "Glory to you, beholding the depths; *",
                    "in the high vault of heaven, glory to you."
                )),
                CanticleVerse::from((
                    "Glory to you, Father, Son, and Holy Spirit; *",
                    "we will praise you and highly exalt you for ever."
                ))
            ]
        }]
    })
    .version(Version::RiteII);
}
