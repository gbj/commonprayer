use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_43: Psalm = Psalm {
        number: 43,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 644
              },
              local_name: String::from("Psalm 43"),
              latin_name: String::from("Judica me, Deus"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Give judgment for me, O God,\nand defend my cause against an ungodly people; *"),
                      b: String::from("deliver me from the deceitful and the wicked.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("For you are the God of my strength;\nwhy have you put me from you? *"),
                      b: String::from("and why do I go so heavily while the enemy oppresses me?")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Send out your light and your truth, that they may lead me, *"),
                      b: String::from("and bring me to your holy hill\n and to your dwelling;")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("That I may go to the altar of God,\nto the God of my joy and gladness; *"),
                      b: String::from("and on the harp I will give thanks to you, O God my God.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Why are you so full of heaviness, O my soul? *"),
                      b: String::from("and why are you so disquieted within me?")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Put your trust in God; *"),
                      b: String::from("for I will yet give thanks to him,\n who is the help of my countenance, and my God.")
                  },
              ]
            }
        ]
    };
}
