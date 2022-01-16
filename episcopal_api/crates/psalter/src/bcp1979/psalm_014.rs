use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_14: Psalm = Psalm {
        number: 14,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 598
              },
              local_name: String::from("Psalm 14"),
              latin_name: String::from("Dixit insipiens"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("The fool has said in his heart, “There is no God.” *"),
                      b: String::from("All are corrupt and commit abominable acts;\n there is none who does any good.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("The LORD looks down from heaven upon us all, *"),
                      b: String::from("to see if there is any who is wise,\n if there is one who seeks after God.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Every one has proved faithless;\nall alike have turned bad; *"),
                      b: String::from("there is none who does good; no, not one.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Have they no knowledge, all those evildoers *"),
                      b: String::from("who eat up my people like bread\n and do not call upon the LORD?")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("See how they tremble with fear, *"),
                      b: String::from("because God is in the company of the righteous.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Their aim is to confound the plans of the afflicted, *"),
                      b: String::from("but the LORD is their refuge.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Oh, that Israel’s deliverance would come out of Zion! *"),
                      b: String::from("when the LORD restores the fortunes of his people,\n Jacob will rejoice and Israel be glad.")
                  },
              ]
            }
        ]
    };
}
