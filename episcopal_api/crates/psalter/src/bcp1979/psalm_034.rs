use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_34: Psalm = Psalm {
        number: 34,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 627
            },
            local_name: String::from("Psalm 34"),
            latin_name: String::from("Benedicam Dominum"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("I will bless the LORD at all times; *"),
                    b: String::from("his praise shall ever be in my mouth.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("I will glory in the LORD; *"),
                    b: String::from("let the humble hear and rejoice.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("Proclaim with me the greatness of the LORD; *"),
                    b: String::from("let us exalt his Name together.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("I sought the LORD, and he answered me *"),
                    b: String::from("and delivered me out of all my terror.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("Look upon him and be radiant, *"),
                    b: String::from("and let not your faces be ashamed.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("I called in my affliction and the LORD heard me *"),
                    b: String::from("and saved me from all my troubles.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("The angel of the LORD encompasses those who fear him, *"),
                    b: String::from("and he will deliver them.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("Taste and see that the LORD is good; *"),
                    b: String::from("happy are they who trust in him!")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("Fear the LORD, you that are his saints, *"),
                    b: String::from("for those who fear him lack nothing.")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("The young lions lack and suffer hunger, *"),
                    b: String::from("but those who seek the LORD lack nothing that is good.")
                },
                PsalmVerse {
                    number: 11,
                    a: String::from("Come, children, and listen to me; *"),
                    b: String::from("I will teach you the fear of the LORD.")
                },
                PsalmVerse {
                    number: 12,
                    a: String::from("Who among you loves life *"),
                    b: String::from("and desires long life to enjoy prosperity?")
                },
                PsalmVerse {
                    number: 13,
                    a: String::from("Keep your tongue from evil-speaking *"),
                    b: String::from("and your lips from lying words.")
                },
                PsalmVerse {
                    number: 14,
                    a: String::from("Turn from evil and do good; *"),
                    b: String::from("seek peace and pursue it.")
                },
                PsalmVerse {
                    number: 15,
                    a: String::from("The eyes of the LORD are upon the righteous, *"),
                    b: String::from("and his ears are open to their cry.")
                },
                PsalmVerse {
                    number: 16,
                    a: String::from("The face of the LORD is against those who do evil, *"),
                    b: String::from("to root out the remembrance of them from the earth.")
                },
                PsalmVerse {
                    number: 17,
                    a: String::from("The righteous cry, and the LORD hears them *"),
                    b: String::from("and delivers them from all their troubles.")
                },
                PsalmVerse {
                    number: 18,
                    a: String::from("The LORD is near to the brokenhearted *"),
                    b: String::from("and will save those whose spirits are crushed.")
                },
                PsalmVerse {
                    number: 19,
                    a: String::from("Many are the troubles of the righteous, *"),
                    b: String::from("but the LORD will deliver him out of them all.")
                },
                PsalmVerse {
                    number: 20,
                    a: String::from("He will keep safe all his bones; *"),
                    b: String::from("not one of them shall be broken.")
                },
                PsalmVerse {
                    number: 21,
                    a: String::from("Evil shall slay the wicked, *"),
                    b: String::from("and those who hate the righteous will be punished.")
                },
                PsalmVerse {
                    number: 22,
                    a: String::from("The LORD ransoms the life of his servants, *"),
                    b: String::from("and none will be punished who trust in him.")
                },
            ]
        }]
    };
}
