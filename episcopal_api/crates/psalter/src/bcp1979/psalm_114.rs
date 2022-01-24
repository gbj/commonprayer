use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_114: Psalm = Psalm {
        number: 114,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 756
            },
            local_name: String::from(""),
            latin_name: String::from("In exitu Israel"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Hallelujah!\nWhen Israel came out of Egypt, *"),
                    b: String::from("the house of Jacob from a people of strange speech,")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Judah became God’s sanctuary *"),
                    b: String::from("and Israel his dominion.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("The sea beheld it and fled; *"),
                    b: String::from("Jordan turned and went back.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("The mountains skipped like rams, *"),
                    b: String::from("and the little hills like young sheep.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("What ailed you, O sea, that you fled? *"),
                    b: String::from("O Jordan, that you turned back?")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("You mountains, that you skipped like rams? *"),
                    b: String::from("you little hills like young sheep?")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("Tremble, O earth, at the presence of the Lord, *"),
                    b: String::from("at the presence of the God of Jacob,")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("Who turned the hard rock into a pool of water *"),
                    b: String::from("and flint-stone into a flowing spring.")
                },
            ]
        }]
    };
}
