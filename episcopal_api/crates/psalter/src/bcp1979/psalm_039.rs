use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_39: Psalm = Psalm {
        number: 39,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 638
              },
              local_name: String::from(""),
              latin_name: String::from("Dixi, Custodiam"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("I said, “I will keep watch upon my ways, *"),
                      b: String::from("so that I do not offend with my tongue.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("I will put a muzzle on my mouth *"),
                      b: String::from("while the wicked are in my presence.”")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("So I held my tongue and said nothing; *"),
                      b: String::from("I refrained from rash words;\nbut my pain became unbearable.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("My heart was hot within me;\nwhile I pondered, the fire burst into flame; *"),
                      b: String::from("I spoke out with my tongue:")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("LORD, let me know my end and the number of my days, *"),
                      b: String::from("so that I may know how short my life is.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("You have given me a mere handful of days,\nand my lifetime is as nothing in your sight; *"),
                      b: String::from("truly, even those who stand erect are but a puff of wind.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("We walk about like a shadow,\nand in vain we are in turmoil; *"),
                      b: String::from("we heap up riches and cannot tell who will gather them.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("And now, what is my hope? *"),
                      b: String::from("O Lord, my hope is in you.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Deliver me from all my transgressions *"),
                      b: String::from("and do not make me the taunt of the fool.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("I fell silent and did not open my mouth, *"),
                      b: String::from("for surely it was you that did it.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Take your affliction from me; *"),
                      b: String::from("I am worn down by the blows of your hand.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("With rebukes for sin you punish us;\nlike a moth you eat away all that is dear to us; *"),
                      b: String::from("truly, everyone is but a puff of wind.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("Hear my prayer, O LORD,\nand give ear to my cry; *"),
                      b: String::from("hold not your peace at my tears.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("For I am but a sojourner with you, *"),
                      b: String::from("a wayfarer, as all my forebears were.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("Turn your gaze from me, that I may be glad again, *"),
                      b: String::from("before I go my way and am no more.")
                  },
              ]
            }
        ]
    };
}
