use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_30: Psalm = Psalm {
        number: 30,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 621
            },
            local_name: String::from("Psalm 30"),
            latin_name: String::from("Exaltabo te, Domine"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("I will exalt you, O LORD,\nbecause you have lifted me up *"),
                    b: String::from("and have not let my enemies triumph over me.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("O LORD my God, I cried out to you, *"),
                    b: String::from("and you restored me to health.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("You brought me up, O LORD, from the dead; *"),
                    b: String::from("you restored my life as I was going down to the grave.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("Sing to the LORD, you servants of his; *"),
                    b: String::from("give thanks for the remembrance of his holiness.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("For his wrath endures but the twinkling of an eye, *"),
                    b: String::from("his favor for a lifetime.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("Weeping may spend the night, *"),
                    b: String::from("but joy comes in the morning.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("While I felt secure, I said,\n“I shall never be disturbed. *"),
                    b: String::from(
                        "You, LORD, with your favor, made me as strong as the mountains.”"
                    )
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("Then you hid your face, *"),
                    b: String::from("and I was filled with fear.")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("I cried to you, O LORD; *"),
                    b: String::from("I pleaded with the Lord, saying,")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from(
                        "“What profit is there in my blood, if I go down to the Pit? *"
                    ),
                    b: String::from("will the dust praise you or declare your faithfulness?")
                },
                PsalmVerse {
                    number: 11,
                    a: String::from("Hear, O LORD, and have mercy upon me; *"),
                    b: String::from("O LORD, be my helper.”")
                },
                PsalmVerse {
                    number: 12,
                    a: String::from("You have turned my wailing into dancing; *"),
                    b: String::from("you have put off my sack-cloth and clothed me with joy.")
                },
                PsalmVerse {
                    number: 13,
                    a: String::from("Therefore my heart sings to you without ceasing; *"),
                    b: String::from("O LORD my God, I will give you thanks for ever.")
                },
            ]
        }]
    };
}
