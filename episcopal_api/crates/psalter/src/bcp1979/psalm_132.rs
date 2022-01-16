use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_132: Psalm = Psalm {
        number: 132,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 785
              },
              local_name: String::from("Psalm 132"),
              latin_name: String::from("Memento, Domine"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("LORD, remember David, *"),
                      b: String::from("and all the hardships he endured;")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("How he swore an oath to the LORD *"),
                      b: String::from("and vowed a vow to the Mighty One of Jacob:")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("“I will not come under the roof of my house, *"),
                      b: String::from("nor climb up into my bed;")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("I will not allow my eyes to sleep, *"),
                      b: String::from("nor let my eyelids slumber;")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Until I find a place for the LORD, *"),
                      b: String::from("a dwelling for the Mighty One of Jacob.”")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("“The ark! We heard it was in Ephratah; *"),
                      b: String::from("we found it in the fields of Jearim.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Let us go to God’s dwelling place; *"),
                      b: String::from("let us fall upon our knees before his footstool.”")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Arise, O LORD, into your resting-place, *"),
                      b: String::from("you and the ark of your strength.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Let your priests be clothed with righteousness; *"),
                      b: String::from("let your faithful people sing with joy.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("For your servant David’s sake, *"),
                      b: String::from("do not turn away the face of your Anointed.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("The LORD has sworn an oath to David; *"),
                      b: String::from("in truth, he will not break it:")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("“A son, the fruit of your body *"),
                      b: String::from("will I set upon your throne.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("If your children keep my covenant\nand my testimonies that I shall teach them, *"),
                      b: String::from("their children will sit upon your throne for evermore.”")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("For the LORD has chosen Zion; *"),
                      b: String::from("he has desired her for his habitation:")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("“This shall be my resting-place for ever; *"),
                      b: String::from("here will I dwell, for I delight in her.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("I will surely bless her provisions, *"),
                      b: String::from("and satisfy her poor with bread.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("I will clothe her priests with salvation, *"),
                      b: String::from("and her faithful people will rejoice and sing.")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("There will I make the horn of David flourish; *"),
                      b: String::from("I have prepared a lamp for my Anointed.")
                  },
                PsalmVerse {
                      number: 19,
                      a: String::from("As for his enemies, I will clothe them with shame; *"),
                      b: String::from("but as for him, his crown will shine.”")
                  },
              ]
            }
        ]
    };
}
