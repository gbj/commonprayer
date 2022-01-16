use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_111: Psalm = Psalm {
        number: 111,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 754
              },
              local_name: String::from("Psalm 111"),
              latin_name: String::from("Confitebor tibi"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Hallelujah!\nI will give thanks to the LORD with my whole heart, *"),
                      b: String::from("in the assembly of the upright, in the congregation.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Great are the deeds of the LORD! *"),
                      b: String::from("they are studied by all who delight in them.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("His work is full of majesty and splendor, *"),
                      b: String::from("and his righteousness endures for ever.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("He makes his marvelous works to be remembered; *"),
                      b: String::from("the LORD is gracious and full of compassion.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("He gives food to those who fear him; *"),
                      b: String::from("he is ever mindful of his covenant.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("He has shown his people the power of his works *"),
                      b: String::from("in giving them the lands of the nations.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("The works of his hands are faithfulness and justice; *"),
                      b: String::from("all his commandments are sure.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("They stand fast for ever and ever, *"),
                      b: String::from("because they are done in truth and equity.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("He sent redemption to his people;\nhe commanded his covenant for ever; *"),
                      b: String::from("holy and awesome is his Name.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("The fear of the LORD is the beginning of wisdom; *"),
                      b: String::from("those who act accordingly have a good understanding;\n his praise endures for ever.")
                  },
              ]
            }
        ]
    };
}
