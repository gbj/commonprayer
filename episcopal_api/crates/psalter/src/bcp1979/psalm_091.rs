use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_91: Psalm = Psalm {
        number: 91,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 719
              },
              local_name: String::from(""),
              latin_name: String::from("Qui habitat"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("He who dwells in the shelter of the Most High, *"),
                      b: String::from("abides under the shadow of the Almighty.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("He shall say to the LORD,\n“You are my refuge and my stronghold, *"),
                      b: String::from("my God in whom I put my trust.”")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("He shall deliver you from the snare of the hunter *"),
                      b: String::from("and from the deadly pestilence.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("He shall cover you with his pinions,\nand you shall find refuge under his wings; *"),
                      b: String::from("his faithfulness shall be a shield and buckler.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("You shall not be afraid of any terror by night, *"),
                      b: String::from("nor of the arrow that flies by day;")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Of the plague that stalks in the darkness, *"),
                      b: String::from("nor of the sickness that lays waste at mid-day.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("A thousand shall fall at your side\nand ten thousand at your right hand, *"),
                      b: String::from("but it shall not come near you.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Your eyes have only to behold *"),
                      b: String::from("to see the reward of the wicked.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Because you have made the LORD your refuge, *"),
                      b: String::from("and the Most High your habitation,")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("There shall no evil happen to you, *"),
                      b: String::from("neither shall any plague come near your dwelling.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("For he shall give his angels charge over you, *"),
                      b: String::from("to keep you in all your ways.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("They shall bear you in their hands, *"),
                      b: String::from("lest you dash your foot against a stone.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("You shall tread upon the lion and the adder; *"),
                      b: String::from("you shall trample the young lion and the serpent under your feet.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("Because he is bound to me in love,\ntherefore will I deliver him; *"),
                      b: String::from("I will protect him, because he knows my Name.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("He shall call upon me, and I will answer him; *"),
                      b: String::from("I am with him in trouble;\nI will rescue him and bring him to honor.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("With long life will I satisfy him, *"),
                      b: String::from("and show him my salvation.")
                  },
              ]
            }
        ]
    };
}
