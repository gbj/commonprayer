use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_138: Psalm = Psalm {
        number: 138,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 793
              },
              local_name: String::from(""),
              latin_name: String::from("Confitebor tibi"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("I will give thanks to you, O LORD, with my whole heart; *"),
                      b: String::from("before the gods I will sing your praise.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("I will bow down toward your holy temple\nand praise your Name, *"),
                      b: String::from("because of your love and faithfulness;")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("For you have glorified your Name *"),
                      b: String::from("and your word above all things.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("When I called, you answered me; *"),
                      b: String::from("you increased my strength within me.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("All the kings of the earth will praise you, O LORD, *"),
                      b: String::from("when they have heard the words of your mouth.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("They will sing of the ways of the LORD, *"),
                      b: String::from("that great is the glory of the LORD.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Though the LORD be high, he cares for the lowly; *"),
                      b: String::from("he perceives the haughty from afar.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Though I walk in the midst of trouble, you keep me safe; *"),
                      b: String::from("you stretch forth your hand against the fury of my enemies;\nyour right hand shall save me.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("The LORD will make good his purpose for me; *"),
                      b: String::from("O LORD, your love endures for ever;\ndo not abandon the works of your hands.")
                  },
              ]
            }
        ]
    };
}
