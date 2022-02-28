use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_25: Psalm = Psalm {
        number: 25,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 614
              },
              local_name: String::from(""),
              latin_name: String::from("Ad te, Domine, levavi"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("To you, O LORD, I lift up my soul;\nmy God, I put my trust in you; *"),
                      b: String::from("let me not be humiliated,\nnor let my enemies triumph over me.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Let none who look to you be put to shame; *"),
                      b: String::from("let the treacherous be disappointed in their schemes.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Show me your ways, O LORD, *"),
                      b: String::from("and teach me your paths.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Lead me in your truth and teach me, *"),
                      b: String::from("for you are the God of my salvation;\nin you have I trusted all the day long.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Remember, O LORD, your compassion and love, *"),
                      b: String::from("for they are from everlasting.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Remember not the sins of my youth and my transgressions; *"),
                      b: String::from("remember me according to your love\nand for the sake of your goodness, O LORD.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Gracious and upright is the LORD; *"),
                      b: String::from("therefore he teaches sinners in his way.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("He guides the humble in doing right *"),
                      b: String::from("and teaches his way to the lowly.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("All the paths of the LORD are love and faithfulness *"),
                      b: String::from("to those who keep his covenant and his testimonies.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("For your Name’s sake, O LORD, *"),
                      b: String::from("forgive my sin, for it is great.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Who are they who fear the LORD? *"),
                      b: String::from("he will teach them the way that they should choose.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("They shall dwell in prosperity, *"),
                      b: String::from("and their offspring shall inherit the land.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("The LORD is a friend to those who fear him *"),
                      b: String::from("and will show them his covenant.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("My eyes are ever looking to the LORD, *"),
                      b: String::from("for he shall pluck my feet out of the net.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("Turn to me and have pity on me, *"),
                      b: String::from("for I am left alone and in misery.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("The sorrows of my heart have increased; *"),
                      b: String::from("bring me out of my troubles.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("Look upon my adversity and misery *"),
                      b: String::from("and forgive me all my sin.")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("Look upon my enemies, for they are many, *"),
                      b: String::from("and they bear a violent hatred against me.")
                  },
                PsalmVerse {
                      number: 19,
                      a: String::from("Protect my life and deliver me; *"),
                      b: String::from("let me not be put to shame, for I have trusted in you.")
                  },
                PsalmVerse {
                      number: 20,
                      a: String::from("Let integrity and uprightness preserve me, *"),
                      b: String::from("for my hope has been in you.")
                  },
                PsalmVerse {
                      number: 21,
                      a: String::from("Deliver Israel, O God, *"),
                      b: String::from("out of all his troubles.")
                  },
              ]
            }
        ]
    };
}
