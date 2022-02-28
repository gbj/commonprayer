use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_41: Psalm = Psalm {
        number: 41,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 641
              },
              local_name: String::from(""),
              latin_name: String::from("Beatus qui intelligit"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Happy are they who consider the poor and needy! *"),
                      b: String::from("the LORD will deliver them in the time of trouble.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("The LORD preserves them and keeps them alive,\nso that they may be happy in the land; *"),
                      b: String::from("he does not hand them over to the will of their enemies.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("The LORD sustains them on their sickbed *"),
                      b: String::from("and ministers to them in their illness.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("I said, “LORD, be merciful to me; *"),
                      b: String::from("heal me, for I have sinned against you.”")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("My enemies are saying wicked things about me: *"),
                      b: String::from("“When will he die, and his name perish?”")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Even if they come to see me, they speak empty words; *"),
                      b: String::from("their heart collects false rumors;\nthey go outside and spread them.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("All my enemies whisper together about me *"),
                      b: String::from("and devise evil against me.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("“A deadly thing,” they say, “has fastened on him; *"),
                      b: String::from("he has taken to his bed and will never get up again.”")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Even my best friend, whom I trusted,\nwho broke bread with me, *"),
                      b: String::from("has lifted up his heel and turned against me.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("But you, O LORD, be merciful to me and raise me up, *"),
                      b: String::from("and I shall repay them.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("By this I know you are pleased with me, *"),
                      b: String::from("that my enemy does not triumph over me.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("In my integrity you hold me fast, *"),
                      b: String::from("and shall set me before your face for ever.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("Blessed be the LORD God of Israel, *"),
                      b: String::from("from age to age. Amen. Amen.")
                  },
              ]
            }
        ]
    };
}
