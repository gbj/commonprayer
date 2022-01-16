use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_80: Psalm = Psalm {
        number: 80,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 702
              },
              local_name: String::from("Psalm 80"),
              latin_name: String::from("Qui regis Israel"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Hear, O Shepherd of Israel, leading Joseph like a flock; *"),
                      b: String::from("shine forth, you that are enthroned upon the cherubim.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("In the presence of Ephraim, Benjamin, and Manasseh, *"),
                      b: String::from("stir up your strength and come to help us.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Restore us, O God of hosts; *"),
                      b: String::from("show the light of your countenance, and we shall be saved.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("O LORD God of hosts, *"),
                      b: String::from("how long will you be angered\n despite the prayers of your people?")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("You have fed them with the bread of tears; *"),
                      b: String::from("you have given them bowls of tears to drink.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("You have made us the derision of our neighbors, *"),
                      b: String::from("and our enemies laugh us to scorn.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Restore us, O God of hosts; *"),
                      b: String::from("show the light of your countenance, and we shall be saved.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("You have brought a vine out of Egypt; *"),
                      b: String::from("you cast out the nations and planted it.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("You prepared the ground for it; *"),
                      b: String::from("it took root and filled the land.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("The mountains were covered by its shadow *"),
                      b: String::from("and the towering cedar trees by its boughs.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("You stretched out its tendrils to the Sea *"),
                      b: String::from("and its branches to the River.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Why have you broken down its wall, *"),
                      b: String::from("so that all who pass by pluck off its grapes?")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("The wild boar of the forest has ravaged it, *"),
                      b: String::from("and the beasts of the field have grazed upon it.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("Turn now, O God of hosts, look down from heaven;\nbehold and tend this vine; *"),
                      b: String::from("preserve what your right hand has planted.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("They burn it with fire like rubbish; *"),
                      b: String::from("at the rebuke of your countenance let them perish.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("Let your hand be upon the man of your right hand, *"),
                      b: String::from("and son of man you have made so strong for yourself.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("And so will we never turn away from you; *"),
                      b: String::from("give us life, that we may call upon your Name.")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("Restore us, O LORD God of hosts; *"),
                      b: String::from("show the light of your countenance, and we shall be saved.")
                  },
              ]
            }
        ]
    };
}
