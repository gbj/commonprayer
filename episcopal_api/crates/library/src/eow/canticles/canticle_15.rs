use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_15_EOW: Document = Document::from(Canticle {
        number: CanticleId::Canticle15,
        citation: Some(String::from("Luke 1:46-55")),
        local_name: String::from("The Song of Mary"),
        latin_name: Some(String::from("Magnificat")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "My soul proclaims the greatness of the Lord,
my spirit rejoices in you, O God my Savior; *",
                    "for you have looked with favor on your lowly servant."
                )),
                CanticleVerse::from((
                    "From this day all generations will call me blessed: *",
                    "you, the Almighty, have done great things for me,
and holy is your Name."
                )),
                CanticleVerse::from((
                    "You have mercy on those who fear you *",
                    "from generation to generation."
                )),
                CanticleVerse::from((
                    "You have shown strength with your arm, *",
                    "and scattered the proud in their conceit,"
                )),
                CanticleVerse::from((
                    "Casting down the mighty from their thrones,",
                    "and lifting up the lowly."
                )),
                CanticleVerse::from((
                    "You have filled the hungry with good things, *",
                    "and sent the rich away empty."
                )),
                CanticleVerse::from((
                    "You have come to the help of your servant Israel, *",
                    "for you have remembered your promise of mercy,"
                )),
                CanticleVerse::from((
                    "The promise you made to our forebears, *",
                    "to Abraham and his children for ever."
                ))
            ]
        }]
    })
    .version(Version::EOW)
    .version_label("EOW");
}
