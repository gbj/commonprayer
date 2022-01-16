use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_88: Psalm = Psalm {
        number: 88,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 712
              },
              local_name: String::from("Psalm 88"),
              latin_name: String::from("Domine, Deus"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("O LORD, my God, my Savior, *"),
                      b: String::from("by day and night I cry to you.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Let my prayer enter into your presence; *"),
                      b: String::from("incline your ear to my lamentation.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("For I am full of trouble; *"),
                      b: String::from("my life is at the brink of the grave.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("I am counted among those who go down to the Pit; *"),
                      b: String::from("I have become like one who has no strength;")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Lost among the dead, *"),
                      b: String::from("like the slain who lie in the grave,")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Whom you remember no more, *"),
                      b: String::from("for they are cut off from your hand.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("You have laid me in the depths of the Pit, *"),
                      b: String::from("in dark places, and in the abyss.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Your anger weighs upon me heavily, *"),
                      b: String::from("and all your great waves overwhelm me.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("You have put my friends far from me;\nyou have made me to be abhorred by them; *"),
                      b: String::from("I am in prison and cannot get free.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("My sight has failed me because of trouble; *"),
                      b: String::from("LORD, I have called upon you daily;\n I have stretched out my hands to you.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Do you work wonders for the dead? *"),
                      b: String::from("will those who have died stand up and give you thanks?")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Will your loving-kindness be declared in the grave? *"),
                      b: String::from("your faithfulness in the land of destruction?")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("Will your wonders be known in the dark? *"),
                      b: String::from("or your righteousness in the country where all is forgotten?")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("But as for me, O LORD, I cry to you for help; *"),
                      b: String::from("in the morning my prayer comes before you.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("LORD, why have you rejected me? *"),
                      b: String::from("why have you hidden your face from me?")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("Ever since my youth, I have been wretched and at the point of death; *"),
                      b: String::from("I have borne your terrors with a troubled mind.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("Your blazing anger has swept over me; *"),
                      b: String::from("your terrors have destroyed me;")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("They surround me all day long like a flood; *"),
                      b: String::from("they encompass me on every side.")
                  },
                PsalmVerse {
                      number: 19,
                      a: String::from("My friend and my neighbor you have put away from me, *"),
                      b: String::from("and darkness is my only companion.")
                  },
              ]
            }
        ]
    };
}
