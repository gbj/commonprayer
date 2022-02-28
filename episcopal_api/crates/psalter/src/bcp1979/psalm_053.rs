use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_53: Psalm = Psalm {
        number: 53,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 658
              },
              local_name: String::from(""),
              latin_name: String::from("Dixit insipiens"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("The fool has said in his heart, “There is no God.” *"),
                      b: String::from("All are corrupt and commit abominable acts;\nthere is none who does any good.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("God looks down from heaven upon us all, *"),
                      b: String::from("to see if there is any who is wise,\nif there is one who seeks after God.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Every one has proved faithless;\nall alike have turned bad; *"),
                      b: String::from("there is none who does good; no, not one.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Have they no knowledge, those evildoers *"),
                      b: String::from("who eat up my people like bread\nand do not call upon God?")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("See how greatly they tremble,\nsuch trembling as never was; *"),
                      b: String::from("for God has scattered the bones of the enemy;\nthey are put to shame, because God has rejected them.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Oh, that Israel’s deliverance would come out of Zion! *"),
                      b: String::from("when God restores the fortunes of his people\nJacob will rejoice and Israel be glad.")
                  },
              ]
            }
        ]
    };
}
