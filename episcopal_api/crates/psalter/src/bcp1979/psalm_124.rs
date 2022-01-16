use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_124: Psalm = Psalm {
        number: 124,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 781
            },
            local_name: String::from("Psalm 124"),
            latin_name: String::from("Nisi quia Dominus"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("If the LORD had not been on our side, *"),
                    b: String::from("let Israel now say;")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("If the LORD had not been on our side, *"),
                    b: String::from("when enemies rose up against us;")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("Then would they have swallowed us up alive *"),
                    b: String::from("in their fierce anger toward us;")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("Then would the waters have overwhelmed us *"),
                    b: String::from("and the torrent gone over us;")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("Then would the raging waters *"),
                    b: String::from("have gone right over us.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("Blessed be the LORD! *"),
                    b: String::from("he has not given us over to be a prey for their teeth.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("We have escaped like a bird from the snare of the fowler; *"),
                    b: String::from("the snare is broken, and we have escaped.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("Our help is in the Name of the LORD, *"),
                    b: String::from("the maker of heaven and earth.")
                },
            ]
        }]
    };
}
