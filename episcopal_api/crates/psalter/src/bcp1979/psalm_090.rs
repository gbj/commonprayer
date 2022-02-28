use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_90: Psalm = Psalm {
        number: 90,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 717
              },
              local_name: String::from(""),
              latin_name: String::from("Domine, refugium"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Lord, you have been our refuge *"),
                      b: String::from("from one generation to another.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Before the mountains were brought forth,\nor the land and the earth were born, *"),
                      b: String::from("from age to age you are God.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("You turn us back to the dust and say, *"),
                      b: String::from("“Go back, O child of earth.”")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("For a thousand years in your sight are like yesterday when it is past *"),
                      b: String::from("and like a watch in the night.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("You sweep us away like a dream; *"),
                      b: String::from("we fade away suddenly like the grass.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("In the morning it is green and flourishes; *"),
                      b: String::from("in the evening it is dried up and withered.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("For we consume away in your displeasure; *"),
                      b: String::from("we are afraid because of your wrathful indignation.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Our iniquities you have set before you, *"),
                      b: String::from("and our secret sins in the light of your countenance.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("When you are angry, all our days are gone; *"),
                      b: String::from("we bring our years to an end like a sigh.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("The span of our life is seventy years,\nperhaps in strength even eighty; *"),
                      b: String::from("yet the sum of them is but labor and sorrow,\nfor they pass away quickly and we are gone.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Who regards the power of your wrath? *"),
                      b: String::from("who rightly fears your indignation?")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("So teach us to number our days *"),
                      b: String::from("that we may apply our hearts to wisdom.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("Return, O LORD; how long will you tarry? *"),
                      b: String::from("be gracious to your servants.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("Satisfy us by your loving-kindness in the morning; *"),
                      b: String::from("so shall we rejoice and be glad all the days of our life.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("Make us glad by the measure of the days that you afflicted us *"),
                      b: String::from("and the years in which we suffered adversity.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("Show your servants your works *"),
                      b: String::from("and your splendor to their children.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("May the graciousness of the LORD our God be upon us; *"),
                      b: String::from("prosper the work of our hands;\nprosper our handiwork.")
                  },
              ]
            }
        ]
    };
}
