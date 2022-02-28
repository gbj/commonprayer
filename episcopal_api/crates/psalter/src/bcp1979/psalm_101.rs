use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_101: Psalm = Psalm {
        number: 101,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 730
              },
              local_name: String::from(""),
              latin_name: String::from("Misericordiam ed judicium"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("I will sing of mercy and justice; *"),
                      b: String::from("to you, O LORD, will I sing praises.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("I will strive to follow a blameless course;\noh, when will you come to me? *"),
                      b: String::from("I will walk with sincerity of heart within my house.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("I will set no worthless thing before my eyes; *"),
                      b: String::from("I hate the doers of evil deeds;\nthey shall not remain with me.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("A crooked heart shall be far from me; *"),
                      b: String::from("I will not know evil.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Those who in secret slander their neighbors I will destroy; *"),
                      b: String::from("those who have a haughty look and a proud heart I cannot abide.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("My eyes are upon the faithful in the land, that they may dwell with me, *"),
                      b: String::from("and only those who lead a blameless life shall be my servants.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Those who act deceitfully shall not dwell in my house, *"),
                      b: String::from("and those who tell lies shall not continue in my sight.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("I will soon destroy all the wicked in the land, *"),
                      b: String::from("that I may root out all evildoers from the city of the LORD.")
                  },
              ]
            }
        ]
    };
}
