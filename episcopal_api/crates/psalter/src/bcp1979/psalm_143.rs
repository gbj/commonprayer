use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_143: Psalm = Psalm {
        number: 143,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 798
              },
              local_name: String::from("Psalm 143"),
              latin_name: String::from("Domine, exaudi"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("LORD, hear my prayer,\nand in your faithfulness heed my supplications; *"),
                      b: String::from("answer me in your righteousness.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Enter not into judgment with your servant, *"),
                      b: String::from("for in your sight shall no one living be justified.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("For my enemy has sought my life;\nhe has crushed me to the ground; *"),
                      b: String::from("he has made me live in dark places like those who are long dead.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("My spirit faints within me; *"),
                      b: String::from("my heart within me is desolate.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("I remember the time past;\nI muse upon all your deeds; *"),
                      b: String::from("I consider the works of your hands.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("I spread out my hands to you; *"),
                      b: String::from("my soul gasps to you like a thirsty land.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("O LORD, make haste to answer me; my spirit fails me; *"),
                      b: String::from("do not hide your face from me\n or I shall be like those who go down to the Pit.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Let me hear of your loving-kindness in the morning,\nfor I put my trust in you; *"),
                      b: String::from("show me the road that I must walk,\n for I lift up my soul to you.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Deliver me from my enemies, O LORD, *"),
                      b: String::from("for I flee to you for refuge.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("Teach me to do what pleases you, for you are my God; *"),
                      b: String::from("let your good Spirit lead me on level ground.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Revive me, O LORD, for your Name’s sake; *"),
                      b: String::from("for your righteousness’ sake, bring me out of trouble.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Of your goodness, destroy my enemies\nand bring all my foes to naught, *"),
                      b: String::from("for truly I am your servant.")
                  },
              ]
            }
      ]
    };
}
