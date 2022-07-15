use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Reference, Source, Version};

lazy_static! {
    pub static ref CANTICLE_21_EOW: Document = Document::from(Canticle {
        number: CanticleId::Canticle21,
        changeable: None,
        citation: None,
        local_name: String::from("We Praise You, O God"),
        latin_name: Some(String::from("Te Deum laudamus")),
        rubric: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "We praise you, O God, \nwe acclaim you as Lord; \nall creation worships you, \nthe  Father  everlasting. \nTo you all angels, all the powers of heaven, \nthe cherubim and seraphim, sing in endless praise: \n	Holy, holy, holy Lord, God of power and might, \n	heaven and earth are full of your glory. \nThe glorious company of apostles praise you. \nThe noble fellowship of prophets praise you. \nThe white-robed army of martyrs praise you. \nThroughout the world the holy Church acclaims you: \n	Father, of majesty unbounded, \n	your true and only Son, worthy of all worship, \n	and the Holy Spirit, advocate and guide. \nYou, Christ, are the king of glory, \nthe eternal Son of the Father. \nWhen you took our flesh to set us free \nyou humbly chose the Virgin’s womb. \nYou overcame the sting of death \nand opened the kingdom of heaven to all believers. \nYou are seated at God’s right hand in glory. \nWe believe that you will come to be our judge. \n	Come then, Lord, and help your people, \n	bought with the price of your own blood, \n	and bring us with your saints \n	to glory everlasting.",
                    ""
                ))
            ]
        }],
        gloria_patri: None,
    })
    .version(Version::EOW)
    .source(Reference {
        source: Source::EOW1,
        page: 28
    });
}
