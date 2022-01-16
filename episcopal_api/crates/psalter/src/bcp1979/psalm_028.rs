use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_28: Psalm = Psalm {
        number: 28,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 619
              },
              local_name: String::from("Psalm 28"),
              latin_name: String::from("Ad te, Domine"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("O LORD, I call to you;\nmy Rock, do not be deaf to my cry; *"),
                      b: String::from("lest, if you do not hear me,\n I become like those who go down to the Pit.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Hear the voice of my prayer when I cry out to you, *"),
                      b: String::from("when I lift up my hands to your holy of holies.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Do not snatch me away with the wicked or with the evildoers, *"),
                      b: String::from("who speak peaceably with their neighbors,\n while strife is in their hearts.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Repay them according to their deeds, *"),
                      b: String::from("and according to the wickedness of their actions.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("According to the work of their hands repay them, *"),
                      b: String::from("and give them their just deserts.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("They have no understanding of the LORD’S doings,\nnor of the works of his hands; *"),
                      b: String::from("therefore he will break them down and not build them up.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Blessed is the LORD! *"),
                      b: String::from("for he has heard the voice of my prayer.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("The LORD is my strength and my shield; *"),
                      b: String::from("my heart trusts in him, and I have been helped;")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Therefore my heart dances for joy, *"),
                      b: String::from("and in my song will I praise him.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("The LORD is the strength of his people, *"),
                      b: String::from("a safe refuge for his anointed.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Save your people and bless your inheritance; *"),
                      b: String::from("shepherd them and carry them for ever.")
                  },
              ]
            }
        ]
    };
}
