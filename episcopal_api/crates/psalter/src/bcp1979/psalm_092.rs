use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_92: Psalm = Psalm {
        number: 92,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 720
              },
              local_name: String::from(""),
              latin_name: String::from("Bonum est confiteri"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("It is a good thing to give thanks to the LORD, *"),
                      b: String::from("and to sing praises to your Name, O Most High;")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("To tell of your loving-kindness early in the morning *"),
                      b: String::from("and of your faithfulness in the night season;")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("On the psaltery, and on the lyre, *"),
                      b: String::from("and to the melody of the harp.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("For you have made me glad by your acts, O LORD; *"),
                      b: String::from("and I shout for joy because of the works of your hands.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("LORD, how great are your works! *"),
                      b: String::from("your thoughts are very deep.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("The dullard does not know,\nnor does the fool understand, *"),
                      b: String::from("that though the wicked grow like weeds,\nand all the workers of iniquity flourish,")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("They flourish only to be destroyed for ever; *"),
                      b: String::from("but you, O LORD, are exalted for evermore.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("For lo, your enemies, O LORD,\nlo, your enemies shall perish, *"),
                      b: String::from("and all the workers of iniquity shall be scattered.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("But my horn you have exalted like the horns of wild bulls; *"),
                      b: String::from("I am anointed with fresh oil.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("My eyes also gloat over my enemies, *"),
                      b: String::from("and my ears rejoice to hear the doom of the wicked who rise up against me.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("The righteous shall flourish like a palm tree, *"),
                      b: String::from("and shall spread abroad like a cedar of Lebanon.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Those who are planted in the house of the LORD *"),
                      b: String::from("shall flourish in the courts of our God;")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("They shall still bear fruit in old age; *"),
                      b: String::from("they shall be green and succulent;")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("That they may show how upright the LORD is, *"),
                      b: String::from("my Rock, in whom there is no fault.")
                  },
              ]
            }
        ]
    };
}
