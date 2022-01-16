use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_H: Document = Document::from(Canticle {
        number: CanticleId::CanticleH,
        citation: Some(String::from("Hosea 6:1-3")),
        local_name: String::from("Canticle H"),
        latin_name: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Come, let us return to our God, *",
                    "who has torn us and will heal us."
                )),
                CanticleVerse::from((
                    "God has struck us and will bind up our wounds, *",
                    "after two days revive us,"
                )),
                CanticleVerse::from((
                    "On the third day restore us, *",
                    "that in God’s presence we may live."
                )),
                CanticleVerse::from((
                    "Let us humble ourselves, let us strive to know the Lord, *",
                    "whose justice dawns like morning light,
its dawning as sure as the sunrise."
                )),
                CanticleVerse::from((
                    "God’s justice will come to us like a shower, *",
                    "like spring rains that water the earth."
                ))
            ]
        }]
    })
    .version(Version::EOW);
}
