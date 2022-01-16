use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_97: Psalm = Psalm {
        number: 97,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 726
              },
              local_name: String::from("Psalm 97"),
              latin_name: String::from("Dominus regnavit"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("The LORD is King;\nlet the earth rejoice; *"),
                      b: String::from("let the multitude of the isles be glad.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Clouds and darkness are round about him, *"),
                      b: String::from("righteousness and justice are the foundations of his throne.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("A fire goes before him *"),
                      b: String::from("and burns up his enemies on every side.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("His lightnings light up the world; *"),
                      b: String::from("the earth sees it and is afraid.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("The mountains melt like wax at the presence of the LORD, *"),
                      b: String::from("at the presence of the Lord of the whole earth.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("The heavens declare his righteousness, *"),
                      b: String::from("and all the peoples see his glory.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Confounded be all who worship carved images\nand delight in false gods! *"),
                      b: String::from("Bow down before him, all you gods.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Zion hears and is glad, and the cities of Judah rejoice, *"),
                      b: String::from("because of your judgments, O LORD.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("For you are the LORD,\nmost high over all the earth; *"),
                      b: String::from("you are exalted far above all gods.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("The LORD loves those who hate evil; *"),
                      b: String::from("he preserves the lives of his saints\n and delivers them from the hand of the wicked.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Light has sprung up for the righteous, *"),
                      b: String::from("and joyful gladness for those who are truehearted.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Rejoice in the LORD, you righteous, *"),
                      b: String::from("and give thanks to his holy Name.")
                  },
              ]
            }
        ]
    };
}
