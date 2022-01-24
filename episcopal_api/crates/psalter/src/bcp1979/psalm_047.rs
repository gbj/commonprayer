use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_47: Psalm = Psalm {
        number: 47,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 650
            },
            local_name: String::from(""),
            latin_name: String::from("Omnes gentes, plaudite"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Clap your hands, all you peoples; *"),
                    b: String::from("shout to God with a cry of joy.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("For the LORD Most High is to be feared; *"),
                    b: String::from("he is the great King over all the earth.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("He subdues the peoples under us, *"),
                    b: String::from("and the nations under our feet.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("He chooses our inheritance for us, *"),
                    b: String::from("the pride of Jacob whom he loves.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("God has gone up with a shout, *"),
                    b: String::from("the LORD with the sound of the ram’s-horn.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("Sing praises to God, sing praises; *"),
                    b: String::from("sing praises to our King, sing praises.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("For God is King of all the earth; *"),
                    b: String::from("sing praises with all your skill.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("God reigns over the nations; *"),
                    b: String::from("God sits upon his holy throne.")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("The nobles of the peoples have gathered together *"),
                    b: String::from("with the people of the God of Abraham.")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("The rulers of the earth belong to God, *"),
                    b: String::from("and he is highly exalted.")
                },
            ]
        }]
    };
}
