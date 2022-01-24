use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_113: Psalm = Psalm {
        number: 113,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 756
            },
            local_name: String::from(""),
            latin_name: String::from("Laudate, pueri"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Hallelujah!\nGive praise, you servants of the LORD; *"),
                    b: String::from("praise the Name of the LORD.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Let the Name of the LORD be blessed, *"),
                    b: String::from("from this time forth for evermore.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("From the rising of the sun to its going down *"),
                    b: String::from("let the Name of the LORD be praised.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("The LORD is high above all nations, *"),
                    b: String::from("and his glory above the heavens.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("Who is like the LORD our God, who sits enthroned on high, *"),
                    b: String::from("but stoops to behold the heavens and the earth?")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("He takes up the weak out of the dust *"),
                    b: String::from("and lifts up the poor from the ashes.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("He sets them with the princes, *"),
                    b: String::from("with the princes of his people.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("He makes the woman of a childless house *"),
                    b: String::from("to be a joyful mother of children.")
                },
            ]
        }]
    };
}
