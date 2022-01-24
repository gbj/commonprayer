use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_1: Psalm = Psalm {
        number: 1,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 585
            },
            local_name: String::from(""),
            latin_name: String::from("Beatus vir qui non abiit"),
            verses: vec![
              PsalmVerse {
                            number: 1,
                            a: String::from("Happy are they who have not walked in the counsel of the wicked, *"),
                            b: String::from("nor lingered in the way of sinners,\nnor sat in the seats of the scornful!")
                        },
                      PsalmVerse {
                            number: 2,
                            a: String::from("Their delight is in the law of the LORD, *"),
                            b: String::from("and they meditate on his law day and night.")
                        },
                      PsalmVerse {
                            number: 3,
                            a: String::from("They are like trees planted by streams of water,\nbearing fruit in due season, with leaves that do not wither; *"),
                            b: String::from("everything they do shall prosper.")
                        },
                      PsalmVerse {
                            number: 4,
                            a: String::from("It is not so with the wicked; *"),
                            b: String::from("they are like chaff which the wind blows away.")
                        },
                      PsalmVerse {
                            number: 5,
                            a: String::from("Therefore the wicked shall not stand upright when judgment comes, *"),
                            b: String::from("nor the sinner in the council of the righteous.")
                        },
                      PsalmVerse {
                            number: 6,
                            a: String::from("For the LORD knows the way of the righteous, *"),
                            b: String::from("but the way of the wicked is doomed.")
                        },
                        ]
        }]
    };
}
