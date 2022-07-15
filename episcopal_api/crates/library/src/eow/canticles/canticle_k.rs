use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Reference, Source, Version};

lazy_static! {
    pub static ref CANTICLE_K: Document = Document::from(Canticle {
        number: CanticleId::CanticleK,
        changeable: None,
        citation: Some(String::from("Ephesians 1:3-10")),
        local_name: String::from("A Song of Our Adoption"),
        latin_name: None,
        rubric: None,
        gloria_patri: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Blessed are you, the God and Father of our Lord Jesus Christ, *",
                    "for you have blessed us in Christ
with every spiritual blessing in the heavenly places."
                )),
                CanticleVerse::from((
                    "Before the world was made, you chose us to be yours in Christ, *",
                    "that we should be holy and blameless before you."
                )),
                CanticleVerse::from((
                    "You destined us for adoption as your children through Jesus Christ, *",
                    "according to the good pleasure of your will,"
                )),
                CanticleVerse::from((
                    "To the praise of your glorious grace, *",
                    "that you have freely given us in the Beloved."
                )),
                CanticleVerse::from((
                    "In you, we have redemption through the blood of Christ, *",
                    "the forgiveness of our sins,"
                )),
                CanticleVerse::from((
                    "According to the riches of your grace *",
                    "which you have lavished upon us."
                )),
                CanticleVerse::from((
                    "You have made known to us, in all wisdom and insight, *",
                    "the mystery of your will,"
                )),
                CanticleVerse::from((
                    "According to your good pleasure which you set forth in Christ, *",
                    "as a plan for the fullness of time,"
                )),
                CanticleVerse::from((
                    "To gather together all things in Christ, *",
                    "things in heaven and things on earth."
                ))
            ]
        }]
    })
    .version(Version::EOW)
    .source(Reference {
        source: Source::EOW1,
        page: 36
    });
}
