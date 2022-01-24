use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};

lazy_static! {
    pub static ref PSALM_120: Psalm = Psalm {
        number: 120,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 778
            },
            local_name: String::from(""),
            latin_name: String::from("Ad Dominum"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("When I was in trouble, I called to the LORD; *"),
                    b: String::from("I called to the LORD, and he answered me.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Deliver me, O LORD, from lying lips *"),
                    b: String::from("and from the deceitful tongue.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("What shall be done to you, and what more besides, *"),
                    b: String::from("O you deceitful tongue?")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("The sharpened arrows of a warrior, *"),
                    b: String::from("along with hot glowing coals.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("How hateful it is that I must lodge in Meshech *"),
                    b: String::from("and dwell among the tents of Kedar!")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("Too long have I had to live *"),
                    b: String::from("among the enemies of peace.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("I am on the side of peace, *"),
                    b: String::from("but when I speak of it, they are for war.")
                },
            ]
        }]
    };
}
