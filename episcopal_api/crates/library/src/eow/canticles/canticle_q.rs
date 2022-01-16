use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_Q: Document = Document::from(Canticle {
        number: CanticleId::CanticleQ,
        citation: Some(String::from("Anselm of Canterbury")),
        local_name: String::from("Canticle Q"),
        latin_name: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Jesus, as a mother you gather your people to you; *",
                    "you are gentle with us as a mother with her children."
                )),
                CanticleVerse::from((
                    "Often you weep over our sins and our pride, *",
                    "tenderly you draw us from hatred and judgment."
                )),
                CanticleVerse::from((
                    "You comfort us in sorrow and bind up our wounds, *",
                    "in sickness you nurse us and with pure milk you feed us."
                )),
                CanticleVerse::from((
                    "Jesus, by your dying, we are born to new life; *",
                    "by your anguish and labor we come forth in joy."
                )),
                CanticleVerse::from((
                    "Despair turns to hope through your sweet goodness; *",
                    "through your gentleness, we find comfort in fear."
                )),
                CanticleVerse::from((
                    "Your warmth gives life to the dead, *",
                    "your touch makes sinners righteous."
                )),
                CanticleVerse::from((
                    "Lord Jesus, in your mercy, heal us; *",
                    "in your love and tenderness, remake us."
                )),
                CanticleVerse::from((
                    "In your compassion, bring grace and forgiveness, *",
                    "for the beauty of heaven, may your love prepare us."
                ))
            ]
        }]
    })
    .version(Version::EOW)
    ;
}
