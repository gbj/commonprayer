use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_6: Psalm = Psalm {
        number: 6,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 589
            },
            local_name: String::from(""),
            latin_name: String::from("Domine, ne in furore"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("LORD, do not rebuke me in your anger; *"),
                    b: String::from("do not punish me in your wrath.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Have pity on me, LORD, for I am weak; *"),
                    b: String::from("heal me, LORD, for my bones are racked.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("My spirit shakes with terror; *"),
                    b: String::from("how long, O LORD, how long?")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("Turn, O LORD, and deliver me; *"),
                    b: String::from("save me for your mercy’s sake.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("For in death no one remembers you; *"),
                    b: String::from("and who will give you thanks in the grave?")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("I grow weary because of my groaning; *"),
                    b: String::from("every night I drench my bed\nand flood my couch with tears.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("My eyes are wasted with grief *"),
                    b: String::from("and worn away because of all my enemies.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("Depart from me, all evildoers, *"),
                    b: String::from("for the LORD has heard the sound of my weeping.")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("The LORD has heard my supplication; *"),
                    b: String::from("the LORD accepts my prayer.")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("All my enemies shall be confounded and quake with fear; *"),
                    b: String::from("they shall turn back and suddenly be put to shame.")
                },
            ]
        }]
    };
}
