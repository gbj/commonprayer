use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_70: Psalm = Psalm {
        number: 70,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 682
            },
            local_name: String::from(""),
            latin_name: String::from("Deus, in adjutorium"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Be pleased, O God, to deliver me; *"),
                    b: String::from("O LORD, make haste to help me.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from(
                        "Let those who seek my life be ashamed\nand altogether dismayed; *"
                    ),
                    b: String::from(
                        "let those who take pleasure in my misfortune\ndraw back and be disgraced."
                    )
                },
                PsalmVerse {
                    number: 3,
                    a: String::from(
                        "Let those who say to me “Aha!” and gloat over me turn back, *"
                    ),
                    b: String::from("because they are ashamed.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("Let all who seek you rejoice and be glad in you; *"),
                    b: String::from(
                        "let those who love your salvation say for ever,\n“Great is the LORD!”"
                    )
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("But as for me, I am poor and needy; *"),
                    b: String::from("come to me speedily, O God.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("You are my helper and my deliverer; *"),
                    b: String::from("O LORD, do not tarry.")
                },
            ]
        }]
    };
}
