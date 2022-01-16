use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_149: Psalm = Psalm {
        number: 149,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 807
            },
            local_name: String::from("Psalm 149"),
            latin_name: String::from("Cantate Domino"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Hallelujah!\nSing to the LORD a new song; *"),
                    b: String::from("sing his praise in the congregation of the faithful.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Let Israel rejoice in his Maker; *"),
                    b: String::from("let the children of Zion be joyful in their King.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("Let them praise his Name in the dance; *"),
                    b: String::from("let them sing praise to him with timbrel and harp.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("For the LORD takes pleasure in his people *"),
                    b: String::from("and adorns the poor with victory.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("Let the faithful rejoice in triumph; *"),
                    b: String::from("let them be joyful on their beds.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("Let the praises of God be in their throat *"),
                    b: String::from("and a two-edged sword in their hand;")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("To wreak vengeance on the nations *"),
                    b: String::from("and punishment on the peoples;")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("To bind their kings in chains *"),
                    b: String::from("and their nobles with links of iron;")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("To inflict on them the judgment decreed; *"),
                    b: String::from("this is glory for all his faithful people.\n Hallelujah!")
                },
            ]
        }]
    };
}
