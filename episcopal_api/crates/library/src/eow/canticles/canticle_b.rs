use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_B: Document = Document::from(Canticle {
        number: CanticleId::CanticleB,
        citation: Some(String::from("Ecclesiasticus 51:13-16,20b-22")),
        local_name: String::from("Canticle B"),
        latin_name: Some(String::from("Priusquam errarem")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Before I ventured forth,
even while I was very young, *",
                    "I sought wisdom openly in my prayer."
                )),
                CanticleVerse::from((
                    "In the forecourts of the temple I asked for her, *",
                    "and I will seek her to the end."
                )),
                CanticleVerse::from((
                    "From first blossom to early fruit, *",
                    "she has been the delight of my heart."
                )),
                CanticleVerse::from((
                    "My foot has kept firmly to the true path, *",
                    "diligently from my youth have I pursued her."
                )),
                CanticleVerse::from((
                    "I inclined my ear a little and received her; *",
                    "I found for myself much wisdom and became adept in her."
                )),
                CanticleVerse::from((
                    "To the one who gives me wisdom will I give glory, *",
                    "for I have resolved to live according to her way."
                )),
                CanticleVerse::from((
                    "From the beginning I gained courage from her, *",
                    "therefore I will not be forsaken."
                )),
                CanticleVerse::from((
                    "In my inmost being I have been stirred to seek her, *",
                    "therefore have I gained a good possession."
                )),
                CanticleVerse::from((
                    "As my reward the Almighty has given me the gift of language, *",
                    "and with it will I offer praise to God."
                ))
            ]
        }]
    })
    .version(Version::EOW);
}
