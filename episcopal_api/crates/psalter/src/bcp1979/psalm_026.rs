use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_26: Psalm = Psalm {
        number: 26,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 616
            },
            local_name: String::from(""),
            latin_name: String::from("Judica me, Domine"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from(
                        "Give judgment for me, O LORD,\nfor I have lived with integrity; *"
                    ),
                    b: String::from("I have trusted in the Lord and have not faltered.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Test me, O LORD, and try me; *"),
                    b: String::from("examine my heart and my mind.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("For your love is before my eyes; *"),
                    b: String::from("I have walked faithfully with you.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("I have not sat with the worthless, *"),
                    b: String::from("nor do I consort with the deceitful.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("I have hated the company of evildoers; *"),
                    b: String::from("I will not sit down with the wicked.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("I will wash my hands in innocence, O LORD, *"),
                    b: String::from("that I may go in procession round your altar,")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("Singing aloud a song of thanksgiving *"),
                    b: String::from("and recounting all your wonderful deeds.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("LORD, I love the house in which you dwell *"),
                    b: String::from("and the place where your glory abides.")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("Do not sweep me away with sinners, *"),
                    b: String::from("nor my life with those who thirst for blood,")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("Whose hands are full of evil plots, *"),
                    b: String::from("and their right hand full of bribes.")
                },
                PsalmVerse {
                    number: 11,
                    a: String::from("As for me, I will live with integrity; *"),
                    b: String::from("redeem me, O LORD, and have pity on me.")
                },
                PsalmVerse {
                    number: 12,
                    a: String::from("My foot stands on level ground; *"),
                    b: String::from("in the full assembly I will bless the LORD.")
                },
            ]
        }]
    };
}
