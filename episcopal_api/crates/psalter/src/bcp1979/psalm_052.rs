use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_52: Psalm = Psalm {
        number: 52,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 657
              },
              local_name: String::from(""),
              latin_name: String::from("Quid gloriaris?"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("You tyrant, why do you boast of wickedness *"),
                      b: String::from("against the godly all day long?")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("You plot ruin;\nyour tongue is like a sharpened razor, *"),
                      b: String::from("O worker of deception.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("You love evil more than good *"),
                      b: String::from("and lying more than speaking the truth.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("You love all words that hurt, *"),
                      b: String::from("O you deceitful tongue.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Oh, that God would demolish you utterly, *"),
                      b: String::from("topple you, and snatch you from your dwelling,\nand root you out of the land of the living!")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("The righteous shall see and tremble, *"),
                      b: String::from("and they shall laugh at him, saying,")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("“This is the one who did not take God for a refuge, *"),
                      b: String::from("but trusted in great wealth\nand relied upon wickedness.”")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("But I am like a green olive tree in the house of God; *"),
                      b: String::from("I trust in the mercy of God for ever and ever.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("I will give you thanks for what you have done *"),
                      b: String::from("and declare the goodness of your Name in the presence of the godly.")
                  },
              ]
            }
        ]
    };
}
