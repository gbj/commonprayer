use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_R: Document = Document::from(Canticle {
        number: CanticleId::CanticleR,
        citation: Some(String::from("Julian of Norwich")),
        local_name: String::from("Canticle R"),
        latin_name: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "God chose to be our mother in all things *",
                    "and so made the foundation of his work,
most humbly and most pure, in the Virgin’s womb."
                )),
                CanticleVerse::from((
                    "God, the perfect wisdom of all, *",
                    "arrayed himself in this humble place."
                )),
                CanticleVerse::from((
                    "Christ came in our poor flesh *",
                    "to share a mother’s care."
                )),
                CanticleVerse::from((
                    "Our mothers bear us for pain and for death; *",
                    "our true mother, Jesus, bears us for joy and endless life."
                )),
                CanticleVerse::from((
                    "Christ carried us within him in love and travail, *",
                    "until the full time of his passion."
                )),
                CanticleVerse::from((
                    "And when all was completed and he had carried us so for joy, *",
                    "still all this could not satisfy the power of his wonderful love."
                )),
                CanticleVerse::from((
                    "All that we owe is redeemed in truly loving God, *",
                    "for the love of Christ works in us;
Christ is the one whom we love."
                ))
            ]
        }]
    })
    .version(Version::EOW)
    ;
}
