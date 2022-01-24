use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_129: Psalm = Psalm {
        number: 129,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 784
            },
            local_name: String::from(""),
            latin_name: String::from("Saepe expugnaverunt"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("“Greatly have they oppressed me since my youth,” *"),
                    b: String::from("let Israel now say;")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("“Greatly have they oppressed me since my youth, *"),
                    b: String::from("but they have not prevailed against me.”")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("The plowmen plowed upon my back *"),
                    b: String::from("and made their furrows long.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("The LORD, the Righteous One, *"),
                    b: String::from("has cut the cords of the wicked.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("Let them be put to shame and thrown back, *"),
                    b: String::from("all those who are enemies of Zion.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("Let them be like grass upon the housetops, *"),
                    b: String::from("which withers before it can be plucked;")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("Which does not fill the hand of the reaper, *"),
                    b: String::from("nor the bosom of him who binds the sheaves;")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from(
                        "So that those who go by say not so much as,\n“The LORD prosper you. *"
                    ),
                    b: String::from("We wish you well in the Name of the LORD.”")
                },
            ]
        }]
    };
}
