use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_56: Psalm = Psalm {
        number: 56,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 662
              },
              local_name: String::from(""),
              latin_name: String::from("Miserere mei, Deus"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Have mercy on me, O God,\nfor my enemies are hounding me; *"),
                      b: String::from("all day long they assault and oppress me.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("They hound me all the day long; *"),
                      b: String::from("truly there are many who fight against me, O Most High.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Whenever I am afraid, *"),
                      b: String::from("I will put my trust in you.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("In God, whose word I praise,\nin God I trust and will not be afraid, *"),
                      b: String::from("for what can flesh do to me?")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("All day long they damage my cause; *"),
                      b: String::from("their only thought is to do me evil.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("They band together; they lie in wait; *"),
                      b: String::from("they spy upon my footsteps;\nbecause they seek my life.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Shall they escape despite their wickedness? *"),
                      b: String::from("O God, in your anger, cast down the peoples.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("You have noted my lamentation;\nput my tears into your bottle; *"),
                      b: String::from("are they not recorded in your book?")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Whenever I call upon you, my enemies will be put to flight; *"),
                      b: String::from("this I know, for God is on my side.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("In God the LORD, whose word I praise,\nin God I trust and will not be afraid, *"),
                      b: String::from("for what can mortals do to me?")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("I am bound by the vow I made to you, O God; *"),
                      b: String::from("I will present to you thank-offerings;")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("For you have rescued my soul from death and my feet from stumbling, *"),
                      b: String::from("that I may walk before God in the light of the living.")
                  },
              ]
            }
        ]
    };
}
