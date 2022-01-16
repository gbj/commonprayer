use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_95: Psalm = Psalm {
        number: 95,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 724
              },
              local_name: String::from("Psalm 95"),
              latin_name: String::from("Venite, exultemus"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Come, let us sing to the LORD; *"),
                      b: String::from("let us shout for joy to the Rock of our salvation.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Let us come before his presence with thanksgiving *"),
                      b: String::from("and raise a loud shout to him with psalms.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("For the LORD is a great God, *"),
                      b: String::from("and a great King above all gods.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("In his hand are the caverns of the earth, *"),
                      b: String::from("and the heights of the hills are his also.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("The sea is his, for he made it, *"),
                      b: String::from("and his hands have molded the dry land.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Come, let us bow down, and bend the knee, *"),
                      b: String::from("and kneel before the LORD our Maker.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("For he is our God,\nand we are the people of his pasture and the sheep of his hand. *"),
                      b: String::from("Oh, that today you would hearken to his voice!")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Harden not your hearts,\nas your forebears did in the wilderness, *"),
                      b: String::from("at Meribah, and on that day at Massah,\n when they tempted me.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("They put me to the test, *"),
                      b: String::from("though they had seen my works.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("Forty years long I detested that generation and said, *"),
                      b: String::from("“This people are wayward in their hearts;\n they do not know my ways.”")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("So I swore in my wrath, *"),
                      b: String::from("“They shall not enter into my rest.”")
                  },
              ]
            }
        ]
    };
}
