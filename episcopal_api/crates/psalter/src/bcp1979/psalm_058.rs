use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_58: Psalm = Psalm {
        number: 58,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 664
            },
            local_name: String::from("Psalm 58"),
            latin_name: String::from("Si vere utique"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Do you indeed decree righteousness, you rulers? *"),
                    b: String::from("do you judge the peoples with equity?")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("No; you devise evil in your hearts, *"),
                    b: String::from("and your hands deal out violence in the land.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("The wicked are perverse from the womb; *"),
                    b: String::from("liars go astray from their birth.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("They are as venomous as a serpent, *"),
                    b: String::from("they are like the deaf adder which stops its ears,")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("Which does not heed the voice of the charmer, *"),
                    b: String::from("no matter how skillful his charming.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("O God, break their teeth in their mouths; *"),
                    b: String::from("pull the fangs of the young lions, O LORD.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("Let them vanish like water that runs off; *"),
                    b: String::from("let them wither like trodden grass.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("Let them be like the snail that melts away, *"),
                    b: String::from("like a stillborn child that never sees the sun.")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("Before they bear fruit, let them be cut down like a brier; *"),
                    b: String::from("like thorns and thistles let them be swept away.")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("The righteous will be glad when they see the vengeance; *"),
                    b: String::from("they will bathe their feet in the blood of the wicked.")
                },
                PsalmVerse {
                    number: 11,
                    a: String::from(
                        "And they will say,\n“Surely, there is a reward for the righteous; *"
                    ),
                    b: String::from("surely, there is a God who rules in the earth.”")
                },
            ]
        }]
    };
}
