use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_15: Document = Document::from(Canticle {
        number: CanticleId::Canticle15,
        citation: Some(String::from("Luke 1:46-55")),
        local_name: String::from("The Song of Mary"),
        latin_name: Some(String::from("Magnificat")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "My soul proclaims the greatness of the Lord,
my spirit rejoices in God my Savior; *",
                    "for he has looked with favor on his lowly servant."
                )),
                CanticleVerse::from((
                    "From this day all generations will call me blessed: *",
                    "the Almighty has done great things for me,
and holy is his Name."
                )),
                CanticleVerse::from((
                    "He has mercy on those who fear him *",
                    "in every generation."
                )),
                CanticleVerse::from((
                    "He has shown the strength of his arm, *",
                    "he has scattered the proud in their conceit."
                )),
                CanticleVerse::from((
                    "He has cast down the mighty from their thrones, *",
                    "and has lifted up the lowly."
                )),
                CanticleVerse::from((
                    "He has filled the hungry with good things, *",
                    "and the rich he has sent away empty."
                )),
                CanticleVerse::from((
                    "He has come to the help of his servant Israel, *",
                    "for he has remembered his promise of mercy,"
                )),
                CanticleVerse::from((
                    "The promise he made to our fathers, *",
                    "to Abraham and his children for ever."
                ))
            ]
        }]
    })
    .version(Version::RiteII);
}
