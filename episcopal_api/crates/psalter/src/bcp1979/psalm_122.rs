use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_122: Psalm = Psalm {
        number: 122,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 779
            },
            local_name: String::from(""),
            latin_name: String::from("Laetatus sum"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("I was glad when they said to me, *"),
                    b: String::from("“Let us go to the house of the LORD.”")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Now our feet are standing *"),
                    b: String::from("within your gates, O Jerusalem.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("Jerusalem is built as a city *"),
                    b: String::from("that is at unity with itself;")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("To which the tribes go up,\nthe tribes of the LORD, *"),
                    b: String::from("the assembly of Israel,\nto praise the Name of the LORD.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("For there are the thrones of judgment, *"),
                    b: String::from("the thrones of the house of David.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("Pray for the peace of Jerusalem: *"),
                    b: String::from("“May they prosper who love you.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("Peace be within your walls *"),
                    b: String::from("and quietness within your towers.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("For my brethren and companions’ sake, *"),
                    b: String::from("I pray for your prosperity.")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("Because of the house of the LORD our God, *"),
                    b: String::from("I will seek to do you good.”")
                },
            ]
        }]
    };
}
