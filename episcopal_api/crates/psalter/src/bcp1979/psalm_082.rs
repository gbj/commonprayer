use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_82: Psalm = Psalm {
        number: 82,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 705
              },
              local_name: String::from(""),
              latin_name: String::from("Deus stetit"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("God takes his stand in the council of heaven; *"),
                      b: String::from("he gives judgment in the midst of the gods:")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("“How long will you judge unjustly, *"),
                      b: String::from("and show favor to the wicked?")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Save the weak and the orphan; *"),
                      b: String::from("defend the humble and needy;")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Rescue the weak and the poor; *"),
                      b: String::from("deliver them from the power of the wicked.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("They do not know, neither do they understand;\nthey go about in darkness; *"),
                      b: String::from("all the foundations of the earth are shaken.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Now I say to you, ‘You are gods, *"),
                      b: String::from("and all of you children of the Most High;")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Nevertheless, you shall die like mortals, *"),
                      b: String::from("and fall like any prince.’”")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Arise, O God, and rule the earth, *"),
                      b: String::from("for you shall take all nations for your own.")
                  },
              ]
            }
        ]
    };
}
