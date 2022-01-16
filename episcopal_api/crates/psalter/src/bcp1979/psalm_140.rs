use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_140: Psalm = Psalm {
        number: 140,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 796
            },
            local_name: String::from("Psalm 140"),
            latin_name: String::from("Eripe me, Domine"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Deliver me, O LORD, from evildoers; *"),
                    b: String::from("protect me from the violent,")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Who devise evil in their hearts *"),
                    b: String::from("and stir up strife all day long.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("They have sharpened their tongues like a serpent; *"),
                    b: String::from("adder’s poison is under their lips.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("Keep me, O LORD, from the hands of the wicked; *"),
                    b: String::from(
                        "protect me from the violent,\n who are determined to trip me up."
                    )
                },
                PsalmVerse {
                    number: 5,
                    a: String::from(
                        "The proud have hidden a snare for me\nand stretched out a net of cords; *"
                    ),
                    b: String::from("they have set traps for me along the path.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("I have said to the LORD, “You are my God; *"),
                    b: String::from("listen, O LORD, to my supplication.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("O Lord GOD, the strength of my salvation, *"),
                    b: String::from("you have covered my head in the day of battle.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("Do not grant the desires of the wicked, O LORD, *"),
                    b: String::from("nor let their evil plans prosper.")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("Let not those who surround me lift up their heads; *"),
                    b: String::from("let the evil of their lips overwhelm them.")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("Let hot burning coals fall upon them; *"),
                    b: String::from("let them be cast into the mire, never to rise up again.”")
                },
                PsalmVerse {
                    number: 11,
                    a: String::from("A slanderer shall not be established on the earth, *"),
                    b: String::from("and evil shall hunt down the lawless.")
                },
                PsalmVerse {
                    number: 12,
                    a: String::from("I know that the LORD will maintain the cause of the poor *"),
                    b: String::from("and render justice to the needy.")
                },
                PsalmVerse {
                    number: 13,
                    a: String::from("Surely, the righteous will give thanks to your Name, *"),
                    b: String::from("and the upright shall continue in your sight.")
                },
            ]
        }]
    };
}
