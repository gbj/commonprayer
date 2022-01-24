use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_76: Psalm = Psalm {
        number: 76,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 692
            },
            local_name: String::from(""),
            latin_name: String::from("Notus in Judaea"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("In Judah is God known; *"),
                    b: String::from("his Name is great in Israel.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("At Salem is his tabernacle, *"),
                    b: String::from("and his dwelling is in Zion.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("There he broke the flashing arrows, *"),
                    b: String::from("the shield, the sword, and the weapons of battle.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("How glorious you are! *"),
                    b: String::from("more splendid than the everlasting mountains!")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from(
                        "The strong of heart have been despoiled;\nthey sink into sleep; *"
                    ),
                    b: String::from("none of the warriors can lift a hand.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("At your rebuke, O God of Jacob, *"),
                    b: String::from("both horse and rider lie stunned.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("What terror you inspire! *"),
                    b: String::from("who can stand before you when you are angry?")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("From heaven you pronounced judgment; *"),
                    b: String::from("the earth was afraid and was still;")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("When God rose up to judgment *"),
                    b: String::from("and to save all the oppressed of the earth.")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("Truly, wrathful Edom will give you thanks, *"),
                    b: String::from("and the remnant of Hamath will keep your feasts.")
                },
                PsalmVerse {
                    number: 11,
                    a: String::from("Make a vow to the LORD your God and keep it; *"),
                    b: String::from(
                        "let all around him bring gifts to him who is worthy to be feared."
                    )
                },
                PsalmVerse {
                    number: 12,
                    a: String::from("He breaks the spirit of princes, *"),
                    b: String::from("and strikes terror in the kings of the earth.")
                },
            ]
        }]
    };
}
