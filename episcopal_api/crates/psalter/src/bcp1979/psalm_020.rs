use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_20: Psalm = Psalm {
        number: 20,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 608
              },
              local_name: String::from(""),
              latin_name: String::from("Exaudiat te Dominus"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("May the LORD answer you in the day of trouble, *"),
                      b: String::from("the Name of the God of Jacob defend you;")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Send you help from his holy place *"),
                      b: String::from("and strengthen you out of Zion;")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Remember all your offerings *"),
                      b: String::from("and accept your burnt sacrifice;")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Grant you your heart’s desire *"),
                      b: String::from("and prosper all your plans.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("We will shout for joy at your victory\nand triumph in the Name of our God; *"),
                      b: String::from("may the LORD grant all your requests.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Now I know that the LORD gives victory to his anointed; *"),
                      b: String::from("he will answer him out of his holy heaven,\nwith the victorious strength of his right hand.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Some put their trust in chariots and some in horses, *"),
                      b: String::from("but we will call upon the Name of the LORD our God.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("They collapse and fall down, *"),
                      b: String::from("but we will arise and stand upright.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("O LORD, give victory to the king *"),
                      b: String::from("and answer us when we call.")
                  },
              ]
            }
        ]
    };
}
