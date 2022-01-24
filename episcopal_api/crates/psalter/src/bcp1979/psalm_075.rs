use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_75: Psalm = Psalm {
        number: 75,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 691
              },
              local_name: String::from(""),
              latin_name: String::from("Confitebimur tibi"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("We give you thanks, O God, we give you thanks, *"),
                      b: String::from("calling upon your Name and declaring all your wonderful deeds.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("“I will appoint a time,” says God; *"),
                      b: String::from("“I will judge with equity.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Though the earth and all its inhabitants are quaking, *"),
                      b: String::from("I will make its pillars fast.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("I will say to the boasters, ‘Boast no more,’ *"),
                      b: String::from("and to the wicked, ‘Do not toss your horns;")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Do not toss your horns so high, *"),
                      b: String::from("nor speak with a proud neck.’”")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("For judgment is neither from the east nor from the west, *"),
                      b: String::from("nor yet from the wilderness or the mountains.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("It is God who judges; *"),
                      b: String::from("he puts down one and lifts up another.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("For in the LORD’s hand there is a cup,\nfull of spiced and foaming wine, which he pours out, *"),
                      b: String::from("and all the wicked of the earth shall drink and drain the dregs.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("But I will rejoice for ever; *"),
                      b: String::from("I will sing praises to the God of Jacob.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("He shall break off all the horns of the wicked; *"),
                      b: String::from("but the horns of the righteous shall be exalted.")
                  },
              ]
            }
        ]
    };
}
