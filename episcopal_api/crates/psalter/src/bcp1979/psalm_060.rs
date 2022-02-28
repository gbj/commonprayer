use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_60: Psalm = Psalm {
        number: 60,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 667
              },
              local_name: String::from(""),
              latin_name: String::from("Deus, repulisti nos"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("O God, you have cast us off and broken us; *"),
                      b: String::from("you have been angry;\noh, take us back to you again.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("You have shaken the earth and split it open; *"),
                      b: String::from("repair the cracks in it, for it totters.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("You have made your people know hardship; *"),
                      b: String::from("you have given us wine that makes us stagger.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("You have set up a banner for those who fear you, *"),
                      b: String::from("to be a refuge from the power of the bow.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Save us by your right hand and answer us, *"),
                      b: String::from("that those who are dear to you may be delivered.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("God spoke from his holy place and said: *"),
                      b: String::from("“I will exult and parcel out Shechem;\n I will divide the valley of Succoth.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Gilead is mine and Manasseh is mine; *"),
                      b: String::from("Ephraim is my helmet and Judah my scepter.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Moab is my wash-basin,\non Edom I throw down my sandal to claim it, *"),
                      b: String::from("and over Philistia will I shout in triumph.”")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Who will lead me into the strong city? *"),
                      b: String::from("who will bring me to Edom?")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("Have you not cast us off, O God? *"),
                      b: String::from("you no longer go out, O God, with our armies.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Grant us your help against the enemy, *"),
                      b: String::from("for vain is the help of man.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("With God we will do valiant deeds, *"),
                      b: String::from("and he shall tread our enemies under foot.")
                  },
              ]
            }
        ]
    };
}
