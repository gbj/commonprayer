use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_32: Psalm = Psalm {
        number: 32,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 624
              },
              local_name: String::from("Psalm 32"),
              latin_name: String::from("Beati quorum"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Happy are they whose transgressions are forgiven, *"),
                      b: String::from("and whose sin is put away!")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Happy are they to whom the LORD imputes no guilt, *"),
                      b: String::from("and in whose spirit there is no guile!")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("While I held my tongue, my bones withered away, *"),
                      b: String::from("because of my groaning all day long.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("For your hand was heavy upon me day and night; *"),
                      b: String::from("my moisture was dried up as in the heat of summer.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Then I acknowledged my sin to you, *"),
                      b: String::from("and did not conceal my guilt.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("I said, “I will confess my transgressions to the LORD.” *"),
                      b: String::from("Then you forgave me the guilt of my sin.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Therefore all the faithful will make their prayers to you in time of trouble; *"),
                      b: String::from("when the great waters overflow, they shall not reach them.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("You are my hiding-place;\nyou preserve me from trouble; *"),
                      b: String::from("you surround me with shouts of deliverance.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("“I will instruct you and teach you in the way that you should go; *"),
                      b: String::from("I will guide you with my eye.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("Do not be like horse or mule, which have no understanding; *"),
                      b: String::from("who must be fitted with bit and bridle,\n or else they will not stay near you.”")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Great are the tribulations of the wicked; *"),
                      b: String::from("but mercy embraces those who trust in the LORD.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Be glad, you righteous, and rejoice in the LORD; *"),
                      b: String::from("shout for joy, all who are true of heart.")
                  },
              ]
            }
        ]
    };
}
