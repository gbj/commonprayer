use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_125: Psalm = Psalm {
        number: 125,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 781
              },
              local_name: String::from(""),
              latin_name: String::from("Qui confidunt"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Those who trust in the LORD are like Mount Zion, *"),
                      b: String::from("which cannot be moved, but stands fast for ever.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("The hills stand about Jerusalem; *"),
                      b: String::from("so does the LORD stand round about his people,\nfrom this time forth for evermore.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("The scepter of the wicked shall not hold sway over the land allotted to the just, *"),
                      b: String::from("so that the just shall not put their hands to evil.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Show your goodness, O LORD, to those who are good *"),
                      b: String::from("and to those who are true of heart.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("As for those who turn aside to crooked ways,\nthe LORD will lead them away with the evildoers; *"),
                      b: String::from("but peace be upon Israel.")
                  },
              ]
            }
        ]
    };
}
