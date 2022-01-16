use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_S: Document = Document::from(Canticle {
        number: CanticleId::CanticleS,
        citation: Some(String::from("Julian of Norwich")),
        local_name: String::from("Canticle S"),
        latin_name: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Christ revealed our frailty and our falling, *",
                    "our trespasses and our humiliations."
                )),
                CanticleVerse::from((
                    "Christ also revealed his blessed power, *",
                    "his blessed wisdom and love."
                )),
                CanticleVerse::from((
                    "He protects us as tenderly and as sweetly when we are in greatest need; *",
                    "he raises us in spirit
and turns everything to glory and joy without ending."
                )),
                CanticleVerse::from((
                    "God is the ground and the substance, the very essence of nature; *",
                    "God is the true father and mother of natures."
                )),
                CanticleVerse::from((
                    "We are all bound to God by nature, *",
                    "and we are all bound to God by grace."
                )),
                CanticleVerse::from((
                    "And this grace is for all the world, *",
                    "because it is our precious mother, Christ."
                )),
                CanticleVerse::from((
                    "For this fair nature was prepared by Christ
for the honor and nobility of all, *",
                    "and for the joy and bliss of salvation."
                ))
            ]
        }]
    })
    .version(Version::EOW)
    ;
}
