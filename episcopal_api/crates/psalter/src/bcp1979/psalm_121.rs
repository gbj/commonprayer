use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_121: Psalm = Psalm {
        number: 121,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 779
            },
            local_name: String::from(""),
            latin_name: String::from("Levavi oculum"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("I lift up my eyes to the hills; *"),
                    b: String::from("from where is my help to come?")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("My help comes from the LORD, *"),
                    b: String::from("the maker of heaven and earth.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("He will not let your foot be moved *"),
                    b: String::from("and he who watches over you will not fall asleep.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("Behold, he who keeps watch over Israel *"),
                    b: String::from("shall neither slumber nor sleep;")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("The LORD himself watches over you; *"),
                    b: String::from("the LORD is your shade at your right hand,")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("So that the sun shall not strike you by day, *"),
                    b: String::from("nor the moon by night.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("The LORD shall preserve you from all evil; *"),
                    b: String::from("it is he who shall keep you safe.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from(
                        "The LORD shall watch over your going out and your coming in, *"
                    ),
                    b: String::from("from this time forth for evermore.")
                },
            ]
        }]
    };
}
