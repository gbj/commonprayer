use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_59: Psalm = Psalm {
        number: 59,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 665
              },
              local_name: String::from("Psalm 59"),
              latin_name: String::from("Eripe me de inimicis"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Rescue me from my enemies, O God; *"),
                      b: String::from("protect me from those who rise up against me.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Rescue me from evildoers *"),
                      b: String::from("and save me from those who thirst for my blood.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("See how they lie in wait for my life,\nhow the mighty gather together against me; *"),
                      b: String::from("not for any offense or fault of mine, O LORD.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Not because of any guilt of mine *"),
                      b: String::from("they run and prepare themselves for battle.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Rouse yourself, come to my side, and see; *"),
                      b: String::from("for you, LORD God of hosts, are Israel’s God.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Awake, and punish all the ungodly; *"),
                      b: String::from("show no mercy to those who are faithless and evil.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("They go to and fro in the evening; *"),
                      b: String::from("they snarl like dogs and run about the city.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Behold, they boast with their mouths,\nand taunts are on their lips; *"),
                      b: String::from("“For who,” they say, “will hear us?”")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("But you, O LORD, you laugh at them; *"),
                      b: String::from("you laugh all the ungodly to scorn.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("My eyes are fixed on you, O my Strength; *"),
                      b: String::from("for you, O God, are my stronghold.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("My merciful God comes to meet me; *"),
                      b: String::from("God will let me look in triumph on my enemies.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Slay them, O God, lest my people forget; *"),
                      b: String::from("send them reeling by your might\n and put them down, O Lord our shield.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("For the sins of their mouths, for the words of their lips,\nfor the cursing and lies that they utter, *"),
                      b: String::from("let them be caught in their pride.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("Make an end of them in your wrath; *"),
                      b: String::from("make an end of them, and they shall be no more.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("Let everyone know that God rules in Jacob, *"),
                      b: String::from("and to the ends of the earth.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("They go to and fro in the evening; *"),
                      b: String::from("they snarl like dogs and run about the city.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("They forage for food, *"),
                      b: String::from("and if they are not filled, they howl.")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("For my part, I will sing of your strength; *"),
                      b: String::from("I will celebrate your love in the morning;")
                  },
                PsalmVerse {
                      number: 19,
                      a: String::from("For you have become my stronghold, *"),
                      b: String::from("a refuge in the day of my trouble.")
                  },
                PsalmVerse {
                      number: 20,
                      a: String::from("To you, O my Strength, will I sing; *"),
                      b: String::from("for you, O God, are my stronghold and my merciful God.")
                  },
              ]
            }
        ]
    };
}
