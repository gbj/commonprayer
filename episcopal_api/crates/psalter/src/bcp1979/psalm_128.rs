use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_128: Psalm = Psalm {
        number: 128,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 783
            },
            local_name: String::from("Psalm 128"),
            latin_name: String::from("Beati omnes"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Happy are they all who fear the LORD, *"),
                    b: String::from("and who follow in his ways!")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("You shall eat the fruit of your labor; *"),
                    b: String::from("happiness and prosperity shall be yours.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("Your wife shall be like a fruitful vine within your house, *"),
                    b: String::from("your children like olive shoots round about your table.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("The man who fears the LORD *"),
                    b: String::from("shall thus indeed be blessed.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("The LORD bless you from Zion, *"),
                    b: String::from(
                        "and may you see the prosperity of Jerusalem all the days of your life."
                    )
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("May you live to see your children’s children; *"),
                    b: String::from("may peace be upon Israel.")
                },
            ]
        }]
    };
}
