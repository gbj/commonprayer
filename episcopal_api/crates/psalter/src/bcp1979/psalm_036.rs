use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_36: Psalm = Psalm {
        number: 36,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 632
              },
              local_name: String::from("Psalm 36"),
              latin_name: String::from("Dixit injustus"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("There is a voice of rebellion deep in the heart of the wicked; *"),
                      b: String::from("there is no fear of God before his eyes.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("He flatters himself in his own eyes *"),
                      b: String::from("that his hateful sin will not be found out.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("The words of his mouth are wicked and deceitful; *"),
                      b: String::from("he has left off acting wisely and doing good.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("He thinks up wickedness upon his bed\nand has set himself in no good way; *"),
                      b: String::from("he does not abhor that which is evil.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Your love, O LORD, reaches to the heavens, *"),
                      b: String::from("and your faithfulness to the clouds.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Your righteousness is like the strong mountains,\nyour justice like the great deep; *"),
                      b: String::from("you save both man and beast, O LORD.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("How priceless is your love, O God! *"),
                      b: String::from("your people take refuge under the shadow of your wings.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("They feast upon the abundance of your house; *"),
                      b: String::from("you give them drink from the river of your delights.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("For with you is the well of life, *"),
                      b: String::from("and in your light we see light.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("Continue your loving-kindness to those who know you, *"),
                      b: String::from("and your favor to those who are true of heart.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Let not the foot of the proud come near me, *"),
                      b: String::from("nor the hand of the wicked push me aside.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("See how they are fallen, those who work wickedness! *"),
                      b: String::from("they are cast down and shall not be able to rise.")
                  },
              ]
            }
        ]
    };
}
