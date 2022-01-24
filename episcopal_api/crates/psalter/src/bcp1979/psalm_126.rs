use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_126: Psalm = Psalm {
        number: 126,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 782
            },
            local_name: String::from(""),
            latin_name: String::from("In convertendo"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("When the LORD restored the fortunes of Zion, *"),
                    b: String::from("then were we like those who dream.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Then was our mouth filled with laughter, *"),
                    b: String::from("and our tongue with shouts of joy.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("Then they said among the nations, *"),
                    b: String::from("“The LORD has done great things for them.”")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("The LORD has done great things for us, *"),
                    b: String::from("and we are glad indeed.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("Restore our fortunes, O LORD, *"),
                    b: String::from("like the watercourses of the Negev.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("Those who sowed with tears *"),
                    b: String::from("will reap with songs of joy.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("Those who go out weeping, carrying the seed, *"),
                    b: String::from("will come again with joy, shouldering their sheaves.")
                },
            ]
        }]
    };
}
