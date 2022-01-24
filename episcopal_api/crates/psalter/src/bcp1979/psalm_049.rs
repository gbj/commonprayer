use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_49: Psalm = Psalm {
        number: 49,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 652
              },
              local_name: String::from(""),
              latin_name: String::from("Audite haec, omnes"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Hear this, all you peoples;\nhearken, all you who dwell in the world, *"),
                      b: String::from("you of high degree and low, rich and poor together.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("My mouth shall speak of wisdom, *"),
                      b: String::from("and my heart shall meditate on understanding.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("I will incline my ear to a proverb *"),
                      b: String::from("and set forth my riddle upon the harp.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Why should I be afraid in evil days, *"),
                      b: String::from("when the wickedness of those at my heels surrounds me,")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("The wickedness of those who put their trust in their goods, *"),
                      b: String::from("and boast of their great riches?")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("We can never ransom ourselves, *"),
                      b: String::from("or deliver to God the price of our life;")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("For the ransom of our life is so great, *"),
                      b: String::from("that we should never have enough to pay it,")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("In order to live for ever and ever, *"),
                      b: String::from("and never see the grave.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("For we see that the wise die also;\nlike the dull and stupid they perish *"),
                      b: String::from("and leave their wealth to those who come after them.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("Their graves shall be their homes for ever,\ntheir dwelling places from generation to generation, *"),
                      b: String::from("though they call the lands after their own names.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Even though honored, they cannot live for ever; *"),
                      b: String::from("they are like the beasts that perish.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Such is the way of those who foolishly trust in themselves, *"),
                      b: String::from("and the end of those who delight in their own words.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("Like a flock of sheep they are destined to die;\nDeath is their shepherd; *"),
                      b: String::from("they go down straightway to the grave.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("Their form shall waste away, *"),
                      b: String::from("and the land of the dead shall be their home.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("But God will ransom my life; *"),
                      b: String::from("he will snatch me from the grasp of death.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("Do not be envious when some become rich, *"),
                      b: String::from("or when the grandeur of their house increases;")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("For they will carry nothing away at their death, *"),
                      b: String::from("nor will their grandeur follow them.")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("Though they thought highly of themselves while they lived, *"),
                      b: String::from("and were praised for their success,")
                  },
                PsalmVerse {
                      number: 19,
                      a: String::from("They shall join the company of their forebears, *"),
                      b: String::from("who will never see the light again.")
                  },
                PsalmVerse {
                      number: 20,
                      a: String::from("Those who are honored, but have no understanding, *"),
                      b: String::from("are like the beasts that perish.")
                  },
              ]
            }
        ]
    };
}
