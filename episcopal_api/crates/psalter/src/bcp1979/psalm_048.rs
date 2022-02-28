use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_48: Psalm = Psalm {
        number: 48,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 651
              },
              local_name: String::from(""),
              latin_name: String::from("Magnus Dominus"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Great is the LORD, and highly to be praised; *"),
                      b: String::from("in the city of our God is his holy hill.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Beautiful and lofty, the joy of all the earth, is the hill of Zion, *"),
                      b: String::from("the very center of the world and the city of the great King.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("God is in her citadels; *"),
                      b: String::from("he is known to be her sure refuge.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Behold, the kings of the earth assembled *"),
                      b: String::from("and marched forward together.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("They looked and were astounded; *"),
                      b: String::from("they retreated and fled in terror.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Trembling seized them there; *"),
                      b: String::from("they writhed like a woman in childbirth,\nlike ships of the sea when the east wind shatters them.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("As we have heard, so have we seen,\nin the city of the LORD of hosts, in the city of our God; *"),
                      b: String::from("God has established her for ever.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("We have waited in silence on your loving-kindness, O God, *"),
                      b: String::from("in the midst of your temple.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Your praise, like your Name, O God, reaches to the world’s end; *"),
                      b: String::from("your right hand is full of justice.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("Let Mount Zion be glad\nand the cities of Judah rejoice, *"),
                      b: String::from("because of your judgments.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Make the circuit of Zion;\nwalk round about her; *"),
                      b: String::from("count the number of her towers.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Consider well her bulwarks;\nexamine her strongholds; *"),
                      b: String::from("that you may tell those who come after.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("This God is our God for ever and ever; *"),
                      b: String::from("he shall be our guide for evermore.")
                  },
              ]
            }
        ]
    };
}
