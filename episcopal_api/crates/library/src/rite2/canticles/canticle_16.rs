use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_16: Document = Document::from(Canticle {
        number: CanticleId::Canticle16,
        citation: Some(String::from("Luke 1: 68-79")),
        local_name: String::from("The Song of Zechariah"),
        latin_name: Some(String::from("Benedictus Dominus Deus")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Blessed be the Lord, the God of Israel; *",
                    "he has come to his people and set them free."
                )),
                CanticleVerse::from((
                    "He has raised up for us a mighty savior, *",
                    "born of the house of his servant David."
                )),
                CanticleVerse::from((
                    "Through his holy prophets he promised of old,
that he would save us from our enemies, *",
                    "from the hands of all who hate us."
                )),
                CanticleVerse::from((
                    "He promised to show mercy to our fathers *",
                    "and to remember his holy covenant."
                )),
                CanticleVerse::from((
                    "This was the oath he swore to our father Abraham, *",
                    "to set us free from the hands of our enemies,"
                )),
                CanticleVerse::from((
                    "Free to worship him without fear, *",
                    "holy and righteous in his sight
all the days of our life."
                )),
                CanticleVerse::from((
                    "You, my child, shall be called the prophet of the Most High, *",
                    "for you will go before the Lord to prepare his way,"
                )),
                CanticleVerse::from((
                    "To give his people knowledge of salvation *",
                    "by the forgiveness of their sins."
                )),
                CanticleVerse::from((
                    "In the tender compassion of our God *",
                    "the dawn from on high shall break upon us,"
                )),
                CanticleVerse::from((
                    "To shine on those who dwell in darkness and the shadow of death, *",
                    "and to guide our feet into the way of peace."
                ))
            ]
        }]
    })
    .version(Version::RiteII);
}
