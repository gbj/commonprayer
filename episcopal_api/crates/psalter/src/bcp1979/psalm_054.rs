use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_54: Psalm = Psalm {
        number: 54,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 659
              },
              local_name: String::from("Psalm 54"),
              latin_name: String::from("Deus, in nomine"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Save me, O God, by your Name; *"),
                      b: String::from("in your might, defend my cause.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Hear my prayer, O God; *"),
                      b: String::from("give ear to the words of my mouth.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("For the arrogant have risen up against me,\nand the ruthless have sought my life, *"),
                      b: String::from("those who have no regard for God.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Behold, God is my helper; *"),
                      b: String::from("it is the Lord who sustains my life.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Render evil to those who spy on me; *"),
                      b: String::from("in your faithfulness, destroy them.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("I will offer you a freewill sacrifice *"),
                      b: String::from("and praise your Name, O LORD, for it is good.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("For you have rescued me from every trouble, *"),
                      b: String::from("and my eye has seen the ruin of my foes.")
                  },
              ]
            }
        ]
    };
}
