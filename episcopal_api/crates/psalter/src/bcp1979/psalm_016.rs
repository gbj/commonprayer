use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_16: Psalm = Psalm {
        number: 16,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 599
              },
              local_name: String::from(""),
              latin_name: String::from("Conserva me, Domine"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Protect me, O God, for I take refuge in you; *"),
                      b: String::from("I have said to the LORD, “You are my Lord,\nmy good above all other.”")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("All my delight is upon the godly that are in the land, *"),
                      b: String::from("upon those who are noble among the people.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("But those who run after other gods *"),
                      b: String::from("shall have their troubles multiplied.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Their libations of blood I will not offer, *"),
                      b: String::from("nor take the names of their gods upon my lips.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("O LORD, you are my portion and my cup; *"),
                      b: String::from("it is you who uphold my lot.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("My boundaries enclose a pleasant land; *"),
                      b: String::from("indeed, I have a goodly heritage.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("I will bless the LORD who gives me counsel; *"),
                      b: String::from("my heart teaches me, night after night.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("I have set the LORD always before me; *"),
                      b: String::from("because he is at my right hand I shall not fall.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("My heart, therefore, is glad, and my spirit rejoices; *"),
                      b: String::from("my body also shall rest in hope.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("For you will not abandon me to the grave, *"),
                      b: String::from("nor let your holy one see the Pit.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("You will show me the path of life; *"),
                      b: String::from("in your presence there is fullness of joy,\nand in your right hand are pleasures for evermore.")
                  },
              ]
            }
        ]
    };
}
