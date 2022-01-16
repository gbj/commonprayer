use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_146: Psalm = Psalm {
        number: 146,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 803
              },
              local_name: String::from("Psalm 146"),
              latin_name: String::from("Lauda, anima mea"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Hallelujah!\nPraise the LORD, O my soul! *"),
                      b: String::from("I will praise the LORD as long as I live;\n I will sing praises to my God while I have my being.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Put not your trust in rulers, nor in any child of earth, *"),
                      b: String::from("for there is no help in them.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("When they breathe their last, they return to earth, *"),
                      b: String::from("and in that day their thoughts perish.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Happy are they who have the God of Jacob for their help! *"),
                      b: String::from("whose hope is in the LORD their God;")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Who made heaven and earth, the seas, and all that is in them; *"),
                      b: String::from("who keeps his promise for ever;")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Who gives justice to those who are oppressed, *"),
                      b: String::from("and food to those who hunger.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("The LORD sets the prisoners free;\nthe LORD opens the eyes of the blind; *"),
                      b: String::from("the LORD lifts up those who are bowed down;")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("The LORD loves the righteous;\nthe LORD cares for the stranger; *"),
                      b: String::from("he sustains the orphan and widow,\n but frustrates the way of the wicked.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("The LORD shall reign for ever, *"),
                      b: String::from("your God, O Zion, throughout all generations.\n Hallelujah!")
                  },
              ]
            }
        ]
    };
}
