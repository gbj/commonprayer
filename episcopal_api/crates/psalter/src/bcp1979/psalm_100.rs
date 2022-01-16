use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_100: Psalm = Psalm {
        number: 100,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 729
              },
              local_name: String::from("Psalm 100"),
              latin_name: String::from("Jubilate Deo"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Be joyful in the LORD, all you lands; *"),
                      b: String::from("serve the LORD with gladness\n and come before his presence with a song.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Know this: The LORD himself is God; *"),
                      b: String::from("he himself has made us, and we are his;\n we are his people and the sheep of his pasture.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Enter his gates with thanksgiving;\ngo into his courts with praise; *"),
                      b: String::from("give thanks to him and call upon his Name.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("For the LORD is good;\nhis mercy is everlasting; *"),
                      b: String::from("and his faithfulness endures from age to age.")
                  },
              ]
            }
        ]
    };
}
