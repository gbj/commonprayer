use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_8: Psalm = Psalm {
        number: 8,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 592
            },
            local_name: String::from("Psalm 8"),
            latin_name: String::from("Domine, Dominus noster"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("O LORD our Governor, *"),
                    b: String::from("how exalted is your Name in all the world!")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Out of the mouths of infants and children *"),
                    b: String::from("your majesty is praised above the heavens.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("You have set up a stronghold against your adversaries, *"),
                    b: String::from("to quell the enemy and the avenger.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("When I consider your heavens, the work of your fingers, *"),
                    b: String::from("the moon and the stars you have set in their courses,")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("What is man that you should be mindful of him? *"),
                    b: String::from("the son of man that you should seek him out?")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("You have made him but little lower than the angels; *"),
                    b: String::from("you adorn him with glory and honor;")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("You give him mastery over the works of your hands; *"),
                    b: String::from("you put all things under his feet:")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("All sheep and oxen, *"),
                    b: String::from("even the wild beasts of the field,")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("The birds of the air, the fish of the sea, *"),
                    b: String::from("and whatsoever walks in the paths of the sea.")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("O LORD our Governor, *"),
                    b: String::from("how exalted is you Name in all the world!")
                },
            ]
        }]
    };
}
