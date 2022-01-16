use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_64: Psalm = Psalm {
        number: 64,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 671
            },
            local_name: String::from("Psalm 64"),
            latin_name: String::from("Exaudi, Deus"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Hear my voice, O God, when I complain; *"),
                    b: String::from("protect my life from fear of the enemy.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Hide me from the conspiracy of the wicked, *"),
                    b: String::from("from the mob of evildoers.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("They sharpen their tongue like a sword, *"),
                    b: String::from("and aim their bitter words like arrows,")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("That they may shoot down the blameless from ambush; *"),
                    b: String::from("they shoot without warning and are not afraid.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("They hold fast to their evil course; *"),
                    b: String::from("they plan how they may hide their snares.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("They say, “Who will see us?\nwho will find out our crimes? *"),
                    b: String::from("we have thought out a perfect plot.”")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("The human mind and heart are a mystery; *"),
                    b: String::from(
                        "but God will loose an arrow at them,\n and suddenly they will be wounded."
                    )
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("He will make them trip over their tongues, *"),
                    b: String::from("and all who see them will shake their heads.")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("Everyone will stand in awe and declare God’s deeds; *"),
                    b: String::from("they will recognize his works.")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from(
                        "The righteous will rejoice in the LORD and put their trust in him, *"
                    ),
                    b: String::from("and all who are true of heart will glory.")
                },
            ]
        }]
    };
}
