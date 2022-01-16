use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_130: Psalm = Psalm {
        number: 130,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 784
            },
            local_name: String::from("Psalm 130"),
            latin_name: String::from("De profundis"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from(
                        "Out of the depths have I called to you, O LORD;\nLORD, hear my voice; *"
                    ),
                    b: String::from("let your ears consider well the voice of my supplication.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("If you, LORD, were to note what is done amiss, *"),
                    b: String::from("O LORD, who could stand?")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("For there is forgiveness with you; *"),
                    b: String::from("therefore you shall be feared.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("I wait for the LORD; my soul waits for him; *"),
                    b: String::from("in his word is my hope.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from(
                        "My soul waits for the LORD,\nmore than watchmen for the morning, *"
                    ),
                    b: String::from("more than watchmen for the morning.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("O Israel, wait for the LORD, *"),
                    b: String::from("for with the LORD there is mercy;")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("With him there is plenteous redemption, *"),
                    b: String::from("and he shall redeem Israel from all their sins.")
                },
            ]
        }]
    };
}
