use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_27: Psalm = Psalm {
        number: 27,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 617
              },
              local_name: String::from(""),
              latin_name: String::from("Dominus illuminatio"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("The LORD is my light and my salvation;\nwhom then shall I fear? *"),
                      b: String::from("the LORD is the strength of my life;\nof whom then shall I be afraid?")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("When evildoers came upon me to eat up my flesh, *"),
                      b: String::from("it was they, my foes and my adversaries, who stumbled and fell.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Though an army should encamp against me, *"),
                      b: String::from("yet my heart shall not be afraid;")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("And though war should rise up against me, *"),
                      b: String::from("yet will I put my trust in him.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("One thing have I asked of the LORD;\none thing I seek; *"),
                      b: String::from("that I may dwell in the house of the LORD all the days of my life;")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("To behold the fair beauty of the LORD *"),
                      b: String::from("and to seek him in his temple.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("For in the day of trouble he shall keep me safe in his shelter; *"),
                      b: String::from("he shall hide me in the secrecy of his dwelling\nand set me high upon a rock.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Even now he lifts up my head *"),
                      b: String::from("above my enemies round about me.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Therefore I will offer in his dwelling an oblation\nwith sounds of great gladness; *"),
                      b: String::from("I will sing and make music to the LORD.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("Hearken to my voice, O LORD, when I call; *"),
                      b: String::from("have mercy on me and answer me.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("You speak in my heart and say, “Seek my face.” *"),
                      b: String::from("Your face, LORD, will I seek.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Hide not your face from me, *"),
                      b: String::from("nor turn away your servant in displeasure.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("You have been my helper;\ncast me not away; *"),
                      b: String::from("do not forsake me, O God of my salvation.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("Though my father and my mother forsake me, *"),
                      b: String::from("the LORD will sustain me.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("Show me your way, O LORD; *"),
                      b: String::from("lead me on a level path, because of my enemies.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("Deliver me not into the hand of my adversaries, *"),
                      b: String::from("for false witnesses have risen up against me,\nand also those who speak malice.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("What if I had not believed\nthat I should see the goodness of the LORD *"),
                      b: String::from("in the land of the living!")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("O tarry and await the LORD’s pleasure;\nbe strong, and he shall comfort your heart; *"),
                      b: String::from("wait patiently for the LORD.")
                  },
              ]
            }
        ]
    };
}
