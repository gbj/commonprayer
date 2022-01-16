use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_15: Psalm = Psalm {
        number: 15,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 599
            },
            local_name: String::from("Psalm 15"),
            latin_name: String::from("Domine, quis habitabit?"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("LORD, who may dwell in your tabernacle? *"),
                    b: String::from("who may abide upon your holy hill?")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Whoever leads a blameless life and does what is right, *"),
                    b: String::from("who speaks the truth from his heart.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from(
                        "There is no guile upon his tongue;\nhe does no evil to his friend; *"
                    ),
                    b: String::from("he does not heap contempt upon his neighbor.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("In his sight the wicked is rejected, *"),
                    b: String::from("but he honors those who fear the LORD.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("He has sworn to do no wrong *"),
                    b: String::from("and does not take back his word.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("He does not give his money in hope of gain, *"),
                    b: String::from("nor does he take a bribe against the innocent.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("Whoever does these things *"),
                    b: String::from("shall never be overthrown.")
                },
            ]
        }]
    };
}
