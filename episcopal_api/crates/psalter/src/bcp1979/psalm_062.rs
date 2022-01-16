use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_62: Psalm = Psalm {
        number: 62,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 669
            },
            local_name: String::from("Psalm 62"),
            latin_name: String::from("Nonne Deo?"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("For God alone my soul in silence waits; *"),
                    b: String::from("from him comes my salvation.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("He alone is my rock and my salvation, *"),
                    b: String::from("my stronghold, so that I shall not be greatly shaken.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from(
                        "How long will you assail me to crush me,\nall of you together, *"
                    ),
                    b: String::from("as if you were a leaning fence, a toppling wall?")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("They seek only to bring me down from my place of honor; *"),
                    b: String::from("lies are their chief delight.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("They bless with their lips, *"),
                    b: String::from("but in their hearts they curse.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("For God alone my soul in silence waits; *"),
                    b: String::from("truly, my hope is in him.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("He alone is my rock and my salvation, *"),
                    b: String::from("my stronghold, so that I shall not be shaken.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("In God is my safety and my honor; *"),
                    b: String::from("God is my strong rock and my refuge.")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("Put your trust in him always, O people, *"),
                    b: String::from("pour out your hearts before him, for God is our refuge.")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("Those of high degree are but a fleeting breath, *"),
                    b: String::from("even those of low estate cannot be trusted.")
                },
                PsalmVerse {
                    number: 11,
                    a: String::from("On the scales they are lighter than a breath, *"),
                    b: String::from("all of them together.")
                },
                PsalmVerse {
                    number: 12,
                    a: String::from(
                        "Put no trust in extortion;\nin robbery take no empty pride; *"
                    ),
                    b: String::from("though wealth increase, set not your heart upon it.")
                },
                PsalmVerse {
                    number: 13,
                    a: String::from("God has spoken once, twice have I heard it, *"),
                    b: String::from("that power belongs to God.")
                },
                PsalmVerse {
                    number: 14,
                    a: String::from("Steadfast love is yours, O Lord, *"),
                    b: String::from("for you repay everyone according to his deeds.")
                },
            ]
        }]
    };
}
