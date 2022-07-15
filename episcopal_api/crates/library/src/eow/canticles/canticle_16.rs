use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Reference, Source, Version};

lazy_static! {
    pub static ref CANTICLE_16_EOW: Document = Document::from(Canticle {
        number: CanticleId::Canticle16,
        changeable: None,
        citation: Some(String::from("Luke 1: 68-79")),
        local_name: String::from("The Song of Zechariah"),
        latin_name: Some(String::from("Benedictus Dominus Deus")),
        rubric: None,
        gloria_patri: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Blessed be the Lord, the God of Israel; *",
                    "you have come to your people and set them free."
                )),
                CanticleVerse::from((
                    "You have raised up for us a mighty savior, *",
                    "born of the house of your servant David."
                )),
                CanticleVerse::from((
                    "Through your holy prophets you promised of old,
that you would save us from our enemies, *",
                    "from the hands of all who hate us."
                )),
                CanticleVerse::from((
                    "To show mercy to our forebears *",
                    "and to remember your holy covenant."
                )),
                CanticleVerse::from((
                    "This was the oath you swore to our father Abraham, *",
                    "to set us free from the hands of our enemies,"
                )),
                CanticleVerse::from((
                    "Free to worship you without fear, *",
                    "holy and righteous before you,
all the days of our life."
                )),
                CanticleVerse::from((
                    "And you, child, shall be called the prophet of the Most High, *",
                    "for you will go before the Lord to prepare the way,"
                )),
                CanticleVerse::from((
                    "To give God’s people knowledge of salvation *",
                    "by the forgiveness of their sins."
                )),
                CanticleVerse::from((
                    "In the tender compassion of our God *",
                    "the dawn from on high shall break upon us,"
                )),
                CanticleVerse::from((
                    "To shine on those who dwell in darkness
and the shadow of death, *",
                    "and to guide our feet into the way of peace."
                ))
            ]
        }]
    })
    .version(Version::EOW)
    .source(Reference {
        source: Source::EOW1,
        page: 27
    });
}
