use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_46: Psalm = Psalm {
        number: 46,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 649
              },
              local_name: String::from(""),
              latin_name: String::from("Deus noster refugium"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("God is our refuge and strength, *"),
                      b: String::from("a very present help in trouble.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Therefore we will not fear, though the earth be moved, *"),
                      b: String::from("and though the mountains be toppled into the depths of the sea;")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Though its waters rage and foam, *"),
                      b: String::from("and though the mountains tremble at its tumult.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("The LORD of hosts is with us; *"),
                      b: String::from("the God of Jacob is our stronghold.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("There is a river whose streams make glad the city of God, *"),
                      b: String::from("the holy habitation of the Most High.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("God is in the midst of her;\nshe shall not be overthrown; *"),
                      b: String::from("God shall help her at the break of day.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("The nations make much ado, and the kingdoms are shaken; *"),
                      b: String::from("God has spoken, and the earth shall melt away.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("The LORD of hosts is with us; *"),
                      b: String::from("the God of Jacob is our stronghold.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Come now and look upon the works of the LORD, *"),
                      b: String::from("what awesome things he has done on earth.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("It is he who makes war to cease in all the world; *"),
                      b: String::from("he breaks the bow, and shatters the spear,\nand burns the shields with fire.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("“Be still, then, and know that I am God; *"),
                      b: String::from("I will be exalted among the nations;\nI will be exalted in the earth.”")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("The LORD of hosts is with us; *"),
                      b: String::from("the God of Jacob is our stronghold.")
                  },
              ]
            }
        ]
    };
}
