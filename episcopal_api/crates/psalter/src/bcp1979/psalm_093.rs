use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_93: Psalm = Psalm {
        number: 93,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 722
              },
              local_name: String::from("Psalm 93"),
              latin_name: String::from("Dominus regnavit"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("The LORD is King;\nhe has put on splendid apparel; *"),
                      b: String::from("the LORD has put on his apparel\n and girded himself with strength.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("He has made the whole world so sure *"),
                      b: String::from("that it cannot be moved;")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Ever since the world began, your throne has been established; *"),
                      b: String::from("you are from everlasting.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("The waters have lifted up, O LORD,\nthe waters have lifted up their voice; *"),
                      b: String::from("the waters have lifted up their pounding waves.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Mightier than the sound of many waters,\nmightier than the breakers of the sea, *"),
                      b: String::from("mightier is the LORD who dwells on high.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Your testimonies are very sure, *"),
                      b: String::from("and holiness adorns your house, O LORD,\n for ever and for evermore.")
                  },
              ]
            }
        ]
    };
}
