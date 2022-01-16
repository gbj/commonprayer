use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_12: Psalm = Psalm {
        number: 12,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 597
            },
            local_name: String::from("Psalm 12"),
            latin_name: String::from("Salvum me fac"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Help me, LORD, for there is no godly one left; *"),
                    b: String::from("the faithful have vanished from among us.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Everyone speaks falsely with his neighbor; *"),
                    b: String::from("with a smooth tongue they speak from a double heart.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("Oh, that the LORD would cut off all smooth tongues, *"),
                    b: String::from("and close the lips that utter proud boasts!")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("Those who say, “With our tongue will we prevail; *"),
                    b: String::from("our lips are our own; who is lord over us?”")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from(
                        "“Because the needy are oppressed,\nand the poor cry out in misery, *"
                    ),
                    b: String::from(
                        "I will rise up,” says the LORD,\n “and give them the help they long for.”"
                    )
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("The words of the LORD are pure words, *"),
                    b: String::from(
                        "like silver refined from ore\n and purified seven times in the fire."
                    )
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("O LORD, watch over us *"),
                    b: String::from("and save us from this generation for ever.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("The wicked prowl on every side, *"),
                    b: String::from("and that which is worthless is highly prized by everyone.")
                },
            ]
        }]
    };
}
