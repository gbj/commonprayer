use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_117: Psalm = Psalm {
        number: 117,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 760
            },
            local_name: String::from("Psalm 117"),
            latin_name: String::from("Laudate Dominum"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Praise the LORD, all you nations; *"),
                    b: String::from("laud him, all you peoples.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("For his loving-kindness toward us is great, *"),
                    b: String::from(
                        "and the faithfulness of the LORD endures for ever.\n Hallelujah!"
                    )
                },
            ]
        }]
    };
}
