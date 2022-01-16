use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_123: Psalm = Psalm {
        number: 123,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 780
            },
            local_name: String::from("Psalm 123"),
            latin_name: String::from("Ad te levavi oculos meos"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("To you I lift up my eyes, *"),
                    b: String::from("to you enthroned in the heavens.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("As the eyes of servants look to the hand of their masters, *"),
                    b: String::from("and the eyes of a maid to the hand of her mistress,")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("So our eyes look to the LORD our God, *"),
                    b: String::from("until he show us his mercy.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("Have mercy upon us, O LORD, have mercy, *"),
                    b: String::from("for we have had more than enough of contempt,")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("Too much of the scorn of the indolent rich, *"),
                    b: String::from("and of the derision of the proud.")
                },
            ]
        }]
    };
}
