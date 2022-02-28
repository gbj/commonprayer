use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_77: Psalm = Psalm {
        number: 77,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 693
              },
              local_name: String::from(""),
              latin_name: String::from("Voce mea ad Dominum"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("I will cry aloud to God; *"),
                      b: String::from("I will cry aloud, and he will hear me.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("In the day of my trouble I sought the Lord; *"),
                      b: String::from("my hands were stretched out by night and did not tire;\nI refused to be comforted.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("I think of God, I am restless, *"),
                      b: String::from("I ponder, and my spirit faints.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("You will not let my eyelids close; *"),
                      b: String::from("I am troubled and I cannot speak.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("I consider the days of old; *"),
                      b: String::from("I remember the years long past;")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("I commune with my heart in the night; *"),
                      b: String::from("I ponder and search my mind.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Will the Lord cast me off for ever? *"),
                      b: String::from("will he no more show his favor?")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Has his loving-kindness come to an end for ever? *"),
                      b: String::from("has his promise failed for evermore?")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Has God forgotten to be gracious? *"),
                      b: String::from("has he, in his anger, withheld his compassion?")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("And I said, “My grief is this: *"),
                      b: String::from("the right hand of the Most High has lost its power.”")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("I will remember the works of the LORD, *"),
                      b: String::from("and call to mind your wonders of old time.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("I will meditate on all your acts *"),
                      b: String::from("and ponder your mighty deeds.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("Your way, O God, is holy; *"),
                      b: String::from("who is so great a god as our God?")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("You are the God who works wonders *"),
                      b: String::from("and have declared your power among the peoples.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("By your strength you have redeemed your people, *"),
                      b: String::from("the children of Jacob and Joseph.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("The waters saw you, O God;\nthe waters saw you and trembled; *"),
                      b: String::from("the very depths were shaken.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("The clouds poured out water;\nthe skies thundered; *"),
                      b: String::from("your arrows flashed to and fro;")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("The sound of your thunder was in the whirlwind;\nyour lightnings lit up the world; *"),
                      b: String::from("the earth trembled and shook.")
                  },
                PsalmVerse {
                      number: 19,
                      a: String::from("Your way was in the sea,\nand your paths in the great waters, *"),
                      b: String::from("yet your footsteps were not seen.")
                  },
                PsalmVerse {
                      number: 20,
                      a: String::from("You led your people like a flock *"),
                      b: String::from("by the hand of Moses and Aaron.")
                  },
              ]
            }
        ]
    };
}
