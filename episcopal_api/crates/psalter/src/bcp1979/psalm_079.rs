use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_79: Psalm = Psalm {
        number: 79,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 701
              },
              local_name: String::from(""),
              latin_name: String::from("Deus, venerunt"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("O God, the heathen have come into your inheritance;\nthey have profaned your holy temple; *"),
                      b: String::from("they have made Jerusalem a heap of rubble.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("They have given the bodies of your servants as food for the birds of the air, *"),
                      b: String::from("and the flesh of your faithful ones to the beasts of the field.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("They have shed their blood like water on every side of Jerusalem, *"),
                      b: String::from("and there was no one to bury them.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("We have become a reproach to our neighbors, *"),
                      b: String::from("an object of scorn and derision to those around us.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("How long will you be angry, O LORD? *"),
                      b: String::from("will your fury blaze like fire for ever?")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Pour out your wrath upon the heathen who have not known you *"),
                      b: String::from("and upon the kingdoms that have not called upon your Name.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("For they have devoured Jacob *"),
                      b: String::from("and made his dwelling a ruin.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Remember not our past sins;\nlet your compassion be swift to meet us; *"),
                      b: String::from("for we have been brought very low.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Help us, O God our Savior, for the glory of your Name; *"),
                      b: String::from("deliver us and forgive us our sins, for your Name’s sake.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("Why should the heathen say, “Where is their God?” *"),
                      b: String::from("Let it be known among the heathen and in our sight\nthat you avenge the shedding of your servants’ blood.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Let the sorrowful sighing of the prisoners come before you, *"),
                      b: String::from("and by your great might spare those who are condemned to die.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("May the revilings with which they reviled you, O Lord, *"),
                      b: String::from("return seven-fold into their bosoms.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("For we are your people and the sheep of your pasture; *"),
                      b: String::from("we will give you thanks for ever\nand show forth your praise from age to age.")
                  },
              ]
            }
        ]
    };
}
