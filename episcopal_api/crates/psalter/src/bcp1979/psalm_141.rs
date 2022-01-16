use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_141: Psalm = Psalm {
        number: 141,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 797
              },
              local_name: String::from("Psalm 141"),
              latin_name: String::from("Domine, clamavi"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("O LORD, I call to you; come to me quickly; *"),
                      b: String::from("hear my voice when I cry to you.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Let my prayer be set forth in your sight as incense, *"),
                      b: String::from("the lifting up of my hands as the evening sacrifice.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Set a watch before my mouth, O LORD,\nand guard the door of my lips; *"),
                      b: String::from("let not my heart incline to any evil thing.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Let me not be occupied in wickedness with evildoers, *"),
                      b: String::from("nor eat of their choice foods.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Let the righteous smite me in friendly rebuke;\nlet not the oil of the unrighteous anoint my head; *"),
                      b: String::from("for my prayer is continually against their wicked deeds.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Let their rulers be overthrown in stony places, *"),
                      b: String::from("that they may know my words are true.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("As when a plowman turns over the earth in furrows, *"),
                      b: String::from("let their bones be scattered at the mouth of the grave.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("But my eyes are turned to you, Lord GOD; *"),
                      b: String::from("in you I take refuge;\n do not strip me of my life.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Protect me from the snare which they have laid for me *"),
                      b: String::from("and from the traps of the evildoers.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("Let the wicked fall into their own nets, *"),
                      b: String::from("while I myself escape.")
                  },
              ]
            }
        ]
    };
}
