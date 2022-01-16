use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_115: Psalm = Psalm {
        number: 115,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 757
            },
            local_name: String::from("Psalm 115"),
            latin_name: String::from("Non nobis, Domine"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from(
                        "Not to us, O LORD, not to us,\nbut to your Name give glory; *"
                    ),
                    b: String::from("because of your love and because of your faithfulness.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Why should the heathen say, *"),
                    b: String::from("“Where then is their God?”")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("Our God is in heaven; *"),
                    b: String::from("whatever he wills to do he does.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("Their idols are silver and gold, *"),
                    b: String::from("the work of human hands.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("They have mouths, but they cannot speak; *"),
                    b: String::from("eyes have they, but they cannot see;")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("They have ears but they cannot hear; *"),
                    b: String::from("noses, but they cannot smell;")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from(
                        "They have hands, but they cannot feel;\nfeet, but they cannot walk; *"
                    ),
                    b: String::from("they make no sound with their throat.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("Those who make them are like them, *"),
                    b: String::from("and so are all who put their trust in them.")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("O Israel, trust in the LORD; *"),
                    b: String::from("he is their help and their shield.")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("O house of Aaron, trust in the LORD; *"),
                    b: String::from("he is their help and their shield.")
                },
                PsalmVerse {
                    number: 11,
                    a: String::from("You who fear the LORD, trust in the LORD; *"),
                    b: String::from("he is their help and their shield.")
                },
                PsalmVerse {
                    number: 12,
                    a: String::from("The LORD has been mindful of us, and he will bless us; *"),
                    b: String::from(
                        "he will bless the house of Israel;\n he will bless the house of Aaron;"
                    )
                },
                PsalmVerse {
                    number: 13,
                    a: String::from("He will bless those who fear the LORD, *"),
                    b: String::from("both small and great together.")
                },
                PsalmVerse {
                    number: 14,
                    a: String::from("May the LORD increase you more and more, *"),
                    b: String::from("you and your children after you.")
                },
                PsalmVerse {
                    number: 15,
                    a: String::from("May you be blessed by the LORD, *"),
                    b: String::from("the maker of heaven and earth.")
                },
                PsalmVerse {
                    number: 16,
                    a: String::from("The heaven of heavens is the LORD’S, *"),
                    b: String::from("but he entrusted the earth to its peoples.")
                },
                PsalmVerse {
                    number: 17,
                    a: String::from("The dead do not praise the LORD, *"),
                    b: String::from("nor all those who go down into silence;")
                },
                PsalmVerse {
                    number: 18,
                    a: String::from("But we will bless the LORD, *"),
                    b: String::from("from this time forth for evermore.\n Hallelujah!")
                },
            ]
        }]
    };
}
