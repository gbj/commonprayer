use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_116: Psalm = Psalm {
        number: 116,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 759
              },
              local_name: String::from(""),
              latin_name: String::from("Dilexi, quoniam"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("I love the LORD, because he has heard the voice of my supplication, *"),
                      b: String::from("because he has inclined his ear to me whenever I called upon him.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("The cords of death entangled me;\nthe grip of the grave took hold of me; *"),
                      b: String::from("I came to grief and sorrow.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Then I called upon the Name of the LORD: *"),
                      b: String::from("“O LORD, I pray you, save my life.”")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Gracious is the LORD and righteous; *"),
                      b: String::from("our God is full of compassion.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("The LORD watches over the innocent; *"),
                      b: String::from("I was brought very low, and he helped me.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Turn again to your rest, O my soul. *"),
                      b: String::from("for the LORD has treated you well.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("For you have rescued my life from death, *"),
                      b: String::from("my eyes from tears, and my feet from stumbling.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("I will walk in the presence of the LORD *"),
                      b: String::from("in the land of the living.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("I believed, even when I said,\n“I have been brought very low.” *"),
                      b: String::from("In my distress I said, “No one can be trusted.”")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("How shall I repay the LORD *"),
                      b: String::from("for all the good things he has done for me?")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("I will lift up the cup of salvation *"),
                      b: String::from("and call upon the Name of the LORD.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("I will fulfill my vows to the LORD *"),
                      b: String::from("in the presence of all his people.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("Precious in the sight of the LORD *"),
                      b: String::from("is the death of his servants.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("O LORD, I am your servant; *"),
                      b: String::from("I am your servant and the child of your handmaid;\nyou have freed me from my bonds.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("I will offer you the sacrifice of thanksgiving *"),
                      b: String::from("and call upon the Name of the LORD.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("I will fulfill my vows to the LORD *"),
                      b: String::from("in the presence of all his people,")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("In the courts of the LORD’s house, *"),
                      b: String::from("in the midst of you, O Jerusalem.\nHallelujah!")
                  },
              ]
            }
        ]
    };
}
