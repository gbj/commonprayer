use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_86: Psalm = Psalm {
        number: 86,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 709
              },
              local_name: String::from("Psalm 86"),
              latin_name: String::from("Inclina, Domine"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Bow down your ear, O LORD, and answer me, *"),
                      b: String::from("for I am poor and in misery.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Keep watch over my life, for I am faithful; *"),
                      b: String::from("save your servant who puts his trust in you.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Be merciful to me, O LORD, for you are my God; *"),
                      b: String::from("I call upon you all the day long.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Gladden the soul of your servant, *"),
                      b: String::from("for to you, O LORD, I lift up my soul.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("For you, O LORD, are good and forgiving, *"),
                      b: String::from("and great is your love toward all who call upon you.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Give ear, O LORD, to my prayer, *"),
                      b: String::from("and attend to the voice of my supplications.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("In the time of my trouble I will call upon you, *"),
                      b: String::from("for you will answer me.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Among the gods there is none like you, O LORD, *"),
                      b: String::from("nor anything like your works.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("All the nations you have made will come and worship you, O LORD, *"),
                      b: String::from("and glorify your Name.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("For you are great;\nyou do wondrous things; *"),
                      b: String::from("and you alone are God.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Teach me your way, O LORD,\nand I will walk in your truth; *"),
                      b: String::from("knit my heart to you that I may fear your Name.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("I will thank you, O LORD my God, with all my heart, *"),
                      b: String::from("and glorify your Name for evermore.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("For great is your love toward me; *"),
                      b: String::from("you have delivered me from the nethermost Pit.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("The arrogant rise up against me, O God,\nand a band of violent men seeks my life; *"),
                      b: String::from("they have not set you before their eyes.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("But you, O LORD, are gracious and full of compassion, *"),
                      b: String::from("slow to anger, and full of kindness and truth.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("Turn to me and have mercy upon me; *"),
                      b: String::from("give your strength to your servant;\n and save the child of your handmaid.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("Show me a sign of your favor,\nso that those who hate me may see it and be ashamed; *"),
                      b: String::from("because you, O LORD, have helped me and comforted me.")
                  },
              ]
            }
        ]
    };
}
