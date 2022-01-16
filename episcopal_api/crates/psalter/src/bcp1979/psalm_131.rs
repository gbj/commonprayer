use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_131: Psalm = Psalm {
        number: 131,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 785
              },
              local_name: String::from("Psalm 131"),
              latin_name: String::from("Domine, non est"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("O LORD, I am not proud; *"),
                      b: String::from("I have no haughty looks.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("I do not occupy myself with great matters, *"),
                      b: String::from("or with things that are too hard for me.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("But I still my soul and make it quiet,\nlike a child upon its mother’s breast; *"),
                      b: String::from("my soul is quieted within me.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("O Israel, wait upon the LORD, *"),
                      b: String::from("from this time forth for evermore.")
                  },
              ]
            }
        ]
    };
}
