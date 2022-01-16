use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_14: Document = Document::from(Canticle {
        number: CanticleId::Canticle14,
        citation: Some(String::from("Prayer of Manasseh 1-2, 4, 6-7, 11-15")),
        local_name: String::from("A Song of Penitence"),
        latin_name: Some(String::from("Kyrie Pantokrator")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "O Lord and Ruler of the hosts of heaven, *",
                    "God of Abraham, Isaac, and Jacob,
and of all their righteous offspring:"
                )),
                CanticleVerse::from((
                    "You made the heavens and the earth, *",
                    "with all their vast array."
                )),
                CanticleVerse::from((
                    "All things quake with fear at your presence; *",
                    "they tremble because of your power."
                )),
                CanticleVerse::from((
                    "But your merciful promise is beyond all measure; *",
                    "it surpasses all that our minds can fathom."
                )),
                CanticleVerse::from((
                    "O Lord, you are full of compassion, *",
                    "long-suffering, and abounding in mercy."
                )),
                CanticleVerse::from((
                    "You hold back your hand; *",
                    "you do not punish as we deserve."
                )),
                CanticleVerse::from((
                    "In your great goodness, Lord,
you have promised forgiveness to sinners, *",
                    "that they may repent of their sin and be saved."
                )),
                CanticleVerse::from((
                    "And now, O Lord, I bend the knee of my heart, *",
                    "and make my appeal, sure of your gracious goodness."
                )),
                CanticleVerse::from((
                    "I have sinned, O Lord, I have sinned, *",
                    "and I know my wickedness only too well."
                )),
                CanticleVerse::from((
                    "Therefore I make this prayer to you: *",
                    "Forgive me, Lord, forgive me."
                )),
                CanticleVerse::from((
                    "Do not let me perish in my sin, *",
                    "nor condemn me to the depths of the earth."
                )),
                CanticleVerse::from((
                    "For you, O Lord, are the God of those who repent, *",
                    "and in me you will show forth your goodness."
                )),
                CanticleVerse::from((
                    "Unworthy as I am, you will save me,
in accordance with your great mercy, *",
                    "and I will praise you without ceasing all the days of my life."
                )),
                CanticleVerse::from((
                    "For all the powers of heaven sing your praises, *",
                    "and yours is the glory to ages of ages. Amen."
                ))
            ]
        }]
    })
    .version(Version::RiteII);
}
