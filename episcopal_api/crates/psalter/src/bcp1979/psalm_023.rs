use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_23: Psalm = Psalm {
        number: 23,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 612
              },
              local_name: String::from("Psalm 23"),
              latin_name: String::from("Dominus regit me"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("The LORD is my shepherd; *"),
                      b: String::from("I shall not be in want.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("He makes me lie down in green pastures *"),
                      b: String::from("and leads me beside still waters.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("He revives my soul *"),
                      b: String::from("and guides me along right pathways for his Name’s sake.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Though I walk through the valley of the shadow of death,\nI shall fear no evil; *"),
                      b: String::from("for you are with me;\n your rod and your staff, they comfort me.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("You spread a table before me in the presence of those who trouble me; *"),
                      b: String::from("you have anointed my head with oil,\n and my cup is running over.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Surely your goodness and mercy shall follow me all the days of my life, *"),
                      b: String::from("and I will dwell in the house of the LORD for ever.")
                  },
              ]
            }
        ]
    };
}
