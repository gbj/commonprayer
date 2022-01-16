use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_96: Psalm = Psalm {
        number: 96,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 725
              },
              local_name: String::from("Psalm 96"),
              latin_name: String::from("Cantate Domino"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Sing to the LORD a new song; *"),
                      b: String::from("sing to the LORD, all the whole earth.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Sing to the LORD and bless his Name; *"),
                      b: String::from("proclaim the good news of his salvation from day to day.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Declare his glory among the nations *"),
                      b: String::from("and his wonders among all peoples.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("For great is the LORD and greatly to be praised; *"),
                      b: String::from("he is more to be feared than all gods.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("As for all the gods of the nations, they are but idols; *"),
                      b: String::from("but it is the LORD who made the heavens.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Oh, the majesty and magnificence of his presence! *"),
                      b: String::from("Oh, the power and the splendor of his sanctuary!")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Ascribe to the LORD, you families of the peoples; *"),
                      b: String::from("ascribe to the LORD honor and power.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Ascribe to the LORD the honor due his Name; *"),
                      b: String::from("bring offerings and come into his courts.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Worship the LORD in the beauty of holiness; *"),
                      b: String::from("let the whole earth tremble before him.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("Tell it out among the nations: “The LORD is King! *"),
                      b: String::from("he has made the world so firm that it cannot be moved;\n he will judge the peoples with equity.”")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Let the heavens rejoice, and let the earth be glad;\nlet the sea thunder and all that is in it; *"),
                      b: String::from("let the field be joyful and all that is therein.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Then shall all the trees of the wood shout for joy\nbefore the LORD when he comes, *"),
                      b: String::from("when he comes to judge the earth.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("He will judge the world with righteousness *"),
                      b: String::from("and the peoples with his truth.")
                  },
              ]
            }
        ]
    };
}
