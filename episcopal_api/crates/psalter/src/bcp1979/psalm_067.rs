use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_67: Psalm = Psalm {
        number: 67,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 675
              },
              local_name: String::from("Psalm 67"),
              latin_name: String::from("Deus misereatur"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("May God be merciful to us and bless us, *"),
                      b: String::from("show us the light of his countenance and come to us.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Let your ways be known upon earth, *"),
                      b: String::from("your saving health among all nations.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Let the peoples praise you, O God; *"),
                      b: String::from("let all the peoples praise you.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Let the nations be glad and sing for joy, *"),
                      b: String::from("for you judge the peoples with equity\n and guide all the nations upon earth.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Let the peoples praise you, O God; *"),
                      b: String::from("let all the peoples praise you.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("The earth has brought forth her increase; *"),
                      b: String::from("may God, our own God, give us his blessing.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("May God give us his blessing, *"),
                      b: String::from("and may all the ends of the earth stand in awe of him.")
                  },
              ]
            }
        ]
    };
}
