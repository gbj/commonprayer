use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};

lazy_static! {
    pub static ref PSALM_50: Psalm = Psalm {
        number: 50,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 654
              },
              local_name: String::from(""),
              latin_name: String::from("Deus deorum"),
              verses: vec![
                            PsalmVerse {
                      number: 1,
                      a: String::from("The LORD, the God of gods, has spoken; *"),
                      b: String::from("he has called the earth from the rising of the sun to its setting.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Out of Zion, perfect in its beauty, *"),
                      b: String::from("God reveals himself in glory.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Our God will come and will not keep silence; *"),
                      b: String::from("before him there is a consuming flame,\nand round about him a raging storm.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("He calls the heavens and the earth from above *"),
                      b: String::from("to witness the judgment of his people.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("“Gather before me my loyal followers, *"),
                      b: String::from("those who have made a covenant with me\nand sealed it with sacrifice.”")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Let the heavens declare the rightness of his cause; *"),
                      b: String::from("for God himself is judge.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Hear, O my people, and I will speak:\n“O Israel, I will bear witness against you; *"),
                      b: String::from("for I am God, your God.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("I do not accuse you because of your sacrifices; *"),
                      b: String::from("your offerings are always before me.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("I will take no bull-calf from your stalls, *"),
                      b: String::from("nor he-goats out of your pens;")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("For all the beasts of the forest are mine, *"),
                      b: String::from("the herds in their thousands upon the hills.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("I know every bird in the sky, *"),
                      b: String::from("and the creatures of the fields are in my sight.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("If I were hungry, I would not tell you, *"),
                      b: String::from("for the whole world is mine and all that is in it.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("Do you think I eat the flesh of bulls, *"),
                      b: String::from("or drink the blood of goats?")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("Offer to God a sacrifice of thanksgiving *"),
                      b: String::from("and make good your vows to the Most High.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("Call upon me in the day of trouble; *"),
                      b: String::from("I will deliver you, and you shall honor me.”")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("But to the wicked God says: *"),
                      b: String::from("“Why do you recite my statutes,\nand take my covenant upon your lips;")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("Since you refuse discipline, *"),
                      b: String::from("and toss my words behind your back?")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("When you see a thief, you make him your friend, *"),
                      b: String::from("and you cast in your lot with adulterers.")
                  },
                PsalmVerse {
                      number: 19,
                      a: String::from("You have loosed your lips for evil, *"),
                      b: String::from("and harnessed your tongue to a lie.")
                  },
                PsalmVerse {
                      number: 20,
                      a: String::from("You are always speaking evil of your brother *"),
                      b: String::from("and slandering your own mother’s son.")
                  },
                PsalmVerse {
                      number: 21,
                      a: String::from("These things you have done, and I kept still, *"),
                      b: String::from("and you thought that I am like you.”")
                  },
                PsalmVerse {
                      number: 22,
                      a: String::from("“I have made my accusation; *"),
                      b: String::from("I have put my case in order before your eyes.")
                  },
                PsalmVerse {
                      number: 23,
                      a: String::from("Consider this well, you who forget God, *"),
                      b: String::from("lest I rend you and there be none to deliver you.")
                  },
                PsalmVerse {
                      number: 24,
                      a: String::from("Whoever offers me the sacrifice of thanksgiving honors me; *"),
                      b: String::from("but to those who keep in my way will I show the salvation of God.”")
                  },
              ]
            }

        ]

    };
}
