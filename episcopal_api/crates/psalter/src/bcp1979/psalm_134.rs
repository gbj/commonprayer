use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_134: Psalm = Psalm {
        number: 134,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 787
            },
            local_name: String::from(""),
            latin_name: String::from("Ecce nunc"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Behold now, bless the LORD, all you servants of the LORD, *"),
                    b: String::from("you that stand by night in the house of the LORD.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Lift up your hands in the holy place and bless the LORD; *"),
                    b: String::from("the LORD who made heaven and earth bless you out of Zion.")
                },
            ]
        }]
    };
}
