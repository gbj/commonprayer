use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_81: Psalm = Psalm {
        number: 81,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 704
              },
              local_name: String::from("Psalm 81"),
              latin_name: String::from("Exultate Deo"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Sing with joy to God our strength *"),
                      b: String::from("and raise a loud shout to the God of Jacob.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Raise a song and sound the timbrel, *"),
                      b: String::from("the merry harp, and the lyre.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Blow the ram’s-horn at the new moon, *"),
                      b: String::from("and at the full moon, the day of our feast.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("For this is a statute for Israel, *"),
                      b: String::from("a law of the God of Jacob.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("He laid it as a solemn charge upon Joseph, *"),
                      b: String::from("when he came out of the land of Egypt.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("I heard an unfamiliar voice saying, *"),
                      b: String::from("“I eased his shoulder from the burden;\n his hands were set free from bearing the load.”")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("You called on me in trouble, and I saved you; *"),
                      b: String::from("I answered you from the secret place of thunder\n and tested you at the waters of Meribah.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Hear, O my people, and I will admonish you: *"),
                      b: String::from("O Israel, if you would but listen to me!")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("There shall be no strange god among you; *"),
                      b: String::from("you shall not worship a foreign god.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("I am the LORD your God,\nwho brought you out of the land of Egypt and said, *"),
                      b: String::from("“Open your mouth wide, and I will fill it.”")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("And yet my people did not hear my voice, *"),
                      b: String::from("and Israel would not obey me.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("So I gave them over to the stubbornness of their hearts, *"),
                      b: String::from("to follow their own devices.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("Oh, that my people would listen to me! *"),
                      b: String::from("that Israel would walk in my ways!")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("I should soon subdue their enemies *"),
                      b: String::from("and turn my hand against their foes.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("Those who hate the LORD would cringe before him, *"),
                      b: String::from("and their punishment would last for ever.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("But Israel would I feed with the finest wheat *"),
                      b: String::from("and satisfy him with honey from the rock.")
                  },
              ]
            }
        ]
    };
}
