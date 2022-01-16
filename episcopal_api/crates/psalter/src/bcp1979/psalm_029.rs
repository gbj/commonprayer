use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_29: Psalm = Psalm {
        number: 29,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 620
              },
              local_name: String::from("Psalm 29"),
              latin_name: String::from("Afferte Domino"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Ascribe to the LORD, you gods, *"),
                      b: String::from("ascribe to the LORD glory and strength.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Ascribe to the LORD the glory due his Name; *"),
                      b: String::from("worship the LORD in the beauty of holiness.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("The voice of the LORD is upon the waters;\nthe God of glory thunders; *"),
                      b: String::from("the LORD is upon the mighty waters.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("The voice of the LORD is a powerful voice; *"),
                      b: String::from("the voice of the LORD is a voice of splendor.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("The voice of the LORD breaks the cedar trees; *"),
                      b: String::from("the LORD breaks the cedars of Lebanon;")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("He makes Lebanon skip like a calf, *"),
                      b: String::from("and Mount Hermon like a young wild ox.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("The voice of the LORD splits the flames of fire;\nthe voice of the LORD shakes the wilderness; *"),
                      b: String::from("the LORD shakes the wilderness of Kadesh.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("The voice of the LORD makes the oak trees writhe *"),
                      b: String::from("and strips the forests bare.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("And in the temple of the LORD *"),
                      b: String::from("all are crying, “Glory!”")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("The LORD sits enthroned above the flood; *"),
                      b: String::from("the LORD sits enthroned as King for evermore.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("The LORD shall give strength to his people; *"),
                      b: String::from("the LORD shall give his people the blessing of peace.")
                  },
              ]
            }
        ]
    };
}
