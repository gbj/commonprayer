use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};

lazy_static! {
    pub static ref PSALM_148: Psalm = Psalm {
        number: 148,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 805
              },
              local_name: String::from(""),
              latin_name: String::from("Laudate Dominum"),
              verses: vec![
                            PsalmVerse {
                      number: 1,
                      a: String::from("Hallelujah!\nPraise the LORD from the heavens; *"),
                      b: String::from("praise him in the heights.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Praise him, all you angels of his; *"),
                      b: String::from("praise him, all his host.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Praise him, sun and moon; *"),
                      b: String::from("praise him, all you shining stars.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Praise him, heaven of heavens, *"),
                      b: String::from("and you waters above the heavens.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Let them praise the Name of the LORD; *"),
                      b: String::from("for he commanded, and they were created.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("He made them stand fast for ever and ever; *"),
                      b: String::from("he gave them a law which shall not pass away.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Praise the LORD from the earth, *"),
                      b: String::from("you sea-monsters and all deeps;")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Fire and hail, snow and fog, *"),
                      b: String::from("tempestuous wind, doing his will;")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Mountains and all hills, *"),
                      b: String::from("fruit trees and all cedars;")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("Wild beasts and all cattle, *"),
                      b: String::from("creeping things and wingèd birds;")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Kings of the earth and all peoples, *"),
                      b: String::from("princes and all rulers of the world;")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Young men and maidens, *"),
                      b: String::from("old and young together.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("Let them praise the Name of the LORD, *"),
                      b: String::from("for his Name only is exalted,\nhis splendor is over earth and heaven.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("He has raised up strength for his people\nand praise for all his loyal servants, *"),
                      b: String::from("the children of Israel, a people who are near him.\nHallelujah!")
                  },
              ]
            }

        ]

    };
}
