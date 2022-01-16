use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_150: Psalm = Psalm {
        number: 150,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 807
            },
            local_name: String::from("Psalm 150"),
            latin_name: String::from("Laudate Domi"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Hallelujah!\nPraise God in his holy temple; *"),
                    b: String::from("praise him in the firmament of his power.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Praise him for his mighty acts; *"),
                    b: String::from("praise him for his excellent greatness.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("Praise him with the blast of the ram’s-horn; *"),
                    b: String::from("Praise him with lyre and harp.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("Praise him with timbrel and dance; *"),
                    b: String::from("praise him with strings and pipe.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("Praise him with resounding cymbals; *"),
                    b: String::from("praise him with loud-clanging cymbals.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("Let everything that has breath *"),
                    b: String::from("praise the Lord.\n Hallelujah!")
                },
            ]
        }]
    };
}
