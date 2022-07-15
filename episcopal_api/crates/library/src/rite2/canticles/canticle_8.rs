use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

use crate::rite2::GLORIA_PATRI;

lazy_static! {
    pub static ref CANTICLE_8: Document = Document::from(Canticle {
        number: CanticleId::Canticle8,
        changeable: None,
        citation: Some(String::from("Exodus 15:1-6, 11-13, 17-18")),
        local_name: String::from("The Song of Moses"),
        latin_name: Some(String::from("Cantemus Domino")),
        rubric: Some(String::from("Especially suitable for use in Easter Season")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "I will sing to the Lord, for he is lofty and uplifted; *",
                    "the horse and its rider has he hurled into the sea."
                )),
                CanticleVerse::from((
                    "The Lord is my strength and my refuge; *",
                    "the Lord has become my Savior."
                )),
                CanticleVerse::from((
                    "This is my God and I will praise him, *",
                    "the God of my people and I will exalt him."
                )),
                CanticleVerse::from(("The Lord is a mighty warrior; *", "Yahweh is his Name.")),
                CanticleVerse::from((
                    "The chariots of Pharoah and his army has he hurled into the sea; *",
                    "the finest of those who bear armor have been drowned in the Red Sea."
                )),
                CanticleVerse::from((
                    "The fathomless deep has overwhelmed them; *",
                    "they sank into the depths like a stone."
                )),
                CanticleVerse::from((
                    "Your right hand, O Lord, is glorious in might; *",
                    "your right hand, O Lord, has overthrown the enemy."
                )),
                CanticleVerse::from((
                    "Who can be compared with you, O Lord, among the gods? *",
                    "who is like you, glorious in holiness,
awesome in renown, and worker of wonders?"
                )),
                CanticleVerse::from((
                    "You stretched forth your right hand; *",
                    "the earth swallowed them up."
                )),
                CanticleVerse::from((
                    "With your constant love you led the people you redeemed; *",
                    "with your might you brought them in safety to your holy dwelling."
                )),
                CanticleVerse::from((
                    "You will bring them in and plant them *",
                    "on the mount of your possession,"
                )),
                CanticleVerse::from((
                    "The resting-place you have made for yourself, O Lord, *",
                    "the sanctuary, O Lord, that your hand has established."
                )),
                CanticleVerse::from(("The Lord shall reign *", "for ever and for ever."))
            ]
        }],
        gloria_patri: Some(GLORIA_PATRI.clone())
    })
    .version(Version::RiteII)
    .page(85);
}
