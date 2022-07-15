use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_21: Document = Document::from(Canticle {
        number: CanticleId::Canticle21,
        changeable: None,
        citation: None,
        local_name: String::from("You are God"),
        latin_name: Some(String::from("Te Deum laudamus")),
        rubric: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "You are God: we praise you;\nYou are the Lord; we acclaim you;\nYou are the eternal Father:\nAll creation worships you.\nTo you all angels, all the powers of heaven,\nCherubim and Seraphim, sing in endless praise:\n    Holy, holy, holy Lord, God of power and might,\n    heaven and earth are full of your glory.\nThe glorious company of apostles praise you.\nThe noble fellowship of prophets praise you.\nThe white-robed army of martyrs praise you.\nThroughout the world the holy Church acclaims you;\n    Father, of majesty unbounded,\n    your true and only Son, worthy of all worship,\n    and the Holy Spirit, advocate and guide. \nYou, Christ, are the king of glory,\nthe eternal Son of the Father.\nWhen you became man to set us free\nyou did not shun the Virgin’s womb.\nYou overcame the sting of death\nand opened the kingdom of heaven to all believers.\nYou are seated at God’s right hand in glory.\nWe believe that you will come and be our judge.\n    Come then, Lord, and help your people,\n    bought with the price of your own blood,\n    and bring us with your saints\n    to glory everlasting. ",
                    ""
                ))
            ]
        }],
        gloria_patri: None,
    })
    .version(Version::RiteII)
    .page(95);



    //Document::from("You are God: we praise you;\nYou are the Lord; we acclaim you;\nYou are the eternal Father:\nAll creation worships you.\nTo you all angels, all the powers of heaven,\nCherubim and Seraphim, sing in endless praise:\n    Holy, holy, holy Lord, God of power and might,\n    heaven and earth are full of your glory.\nThe glorious company of apostles praise you.\nThe noble fellowship of prophets praise you.\nThe white-robed army of martyrs praise you.\nThroughout the world the holy Church acclaims you;\n    Father, of majesty unbounded,\n    your true and only Son, worthy of all worship,\n    and the Holy Spirit, advocate and guide. \nYou, Christ, are the king of glory,\nthe eternal Son of the Father.\nWhen you became man to set us free\nyou did not shun the Virgin’s womb.\nYou overcame the sting of death\nand opened the kingdom of heaven to all believers.\nYou are seated at God’s right hand in glory.\nWe believe that you will come and be our judge.\n    Come then, Lord, and help your people,\n    bought with the price of your own blood,\n    and bring us with your saints\n    to glory everlasting. ").version(Version::RiteII);
}
