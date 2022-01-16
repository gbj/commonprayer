use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_61: Psalm = Psalm {
        number: 61,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 668
            },
            local_name: String::from("Psalm 61"),
            latin_name: String::from("Exaudi, Deus"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Hear my cry, O God, *"),
                    b: String::from("and listen to my prayer.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from(
                        "I call upon you from the ends of the earth\nwith heaviness in my heart; *"
                    ),
                    b: String::from("set me upon the rock that is higher than I.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("For you have been my refuge, *"),
                    b: String::from("a strong tower against the enemy.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("I will dwell in your house for ever; *"),
                    b: String::from("I will take refuge under the cover of your wings.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("For you, O God, have heard my vows; *"),
                    b: String::from(
                        "you have granted me the heritage of those who fear your Name."
                    )
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("Add length of days to the king’s life; *"),
                    b: String::from("let his years extend over many generations.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("Let him sit enthroned before God for ever; *"),
                    b: String::from("bid love and faithfulness watch over him.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("So will I always sing the praise of your Name, *"),
                    b: String::from("and day by day I will fulfill my vows.")
                },
            ]
        }]
    };
}
