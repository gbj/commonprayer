use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_133: Psalm = Psalm {
        number: 133,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 787
            },
            local_name: String::from("Psalm 133"),
            latin_name: String::from("Ecce, quam bonum!"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Oh, how good and pleasant it is, *"),
                    b: String::from("when brethren live together in unity!")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("It is like fine oil upon the head *"),
                    b: String::from("that runs down upon the beard,")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("Upon the beard of Aaron, *"),
                    b: String::from("and runs down upon the collar of his robe.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("It is like the dew of Hermon *"),
                    b: String::from("that falls upon the hills of Zion.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("For there the LORD has ordained the blessing: *"),
                    b: String::from("life for evermore.")
                },
            ]
        }]
    };
}
