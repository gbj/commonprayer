use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_42: Psalm = Psalm {
        number: 42,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 643
              },
              local_name: String::from(""),
              latin_name: String::from("Quemadmodum"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("As the deer longs for the water-brooks, *"),
                      b: String::from("so longs my soul for you, O God.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("My soul is athirst for God, athirst for the living God; *"),
                      b: String::from("when shall I come to appear before the presence of God?")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("My tears have been my food day and night, *"),
                      b: String::from("while all day long they say to me,\n“Where now is your God?”")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("I pour out my soul when I think on these things; *"),
                      b: String::from("how I went with the multitude and led them into the house of God,")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("With the voice of praise and thanksgiving, *"),
                      b: String::from("among those who keep holy-day.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Why are you so full of heaviness, O my soul? *"),
                      b: String::from("and why are you so disquieted within me?")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Put your trust in God; *"),
                      b: String::from("for I will yet give thanks to him,\nwho is the help of my countenance, and my God.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("My soul is heavy within me; *"),
                      b: String::from("therefore I will remember you from the land of Jordan,\nand from the peak of Mizar among the heights of Hermon.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("One deep calls to another in the noise of your cataracts; *"),
                      b: String::from("all your rapids and floods have gone over me.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("The LORD grants his loving-kindness in the daytime; *"),
                      b: String::from("in the night season his song is with me,\na prayer to the God of my life.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("I will say to the God of my strength,\n“Why have you forgotten me? *"),
                      b: String::from("and why do I go so heavily while the enemy oppresses me?”")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("While my bones are being broken, *"),
                      b: String::from("my enemies mock me to my face;")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("All day long they mock me *"),
                      b: String::from("and say to me, “Where now is your God?”")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("Why are you so full of heaviness, O my soul? *"),
                      b: String::from("and why are you so disquieted within me?")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("Put your trust in God; *"),
                      b: String::from("for I will yet give thanks to him,\nwho is the help of my countenance, and my God.")
                  },
              ]
            }
        ]
    };
}
