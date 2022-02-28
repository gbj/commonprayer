use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};

lazy_static! {
    pub static ref PSALM_99: Psalm = Psalm {
        number: 99,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 728
              },
              local_name: String::from(""),
              latin_name: String::from("Dominus regnavit"),
              verses: vec![
                            PsalmVerse {
                      number: 1,
                      a: String::from("The LORD is King;\nlet the people tremble; *"),
                      b: String::from("he is enthroned upon the cherubim;\nlet the earth shake.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("The LORD is great in Zion; *"),
                      b: String::from("he is high above all peoples.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Let them confess his Name, which is great and awesome; *"),
                      b: String::from("he is the Holy One.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("“O mighty King, lover of justice,\nyou have established equity; *"),
                      b: String::from("you have executed justice and righteousness in Jacob.”")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Proclaim the greatness of the LORD our God\nand fall down before his footstool; *"),
                      b: String::from("he is the Holy One.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Moses and Aaron among his priests,\nand Samuel among those who call upon his Name, *"),
                      b: String::from("they called upon the LORD, and he answered them.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("He spoke to them out of the pillar of cloud; *"),
                      b: String::from("they kept his testimonies and the decree that he gave them.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("“O LORD our God, you answered them indeed; *"),
                      b: String::from("you were a God who forgave them,\nyet punished them for their evil deeds.”")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Proclaim the greatness of the LORD our God\nand worship him upon his holy hill; *"),
                      b: String::from("for the LORD our God is the Holy One.")
                  },
              ]
            }

        ]

    };
}
