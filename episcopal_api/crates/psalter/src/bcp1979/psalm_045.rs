use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_45: Psalm = Psalm {
        number: 45,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 647
              },
              local_name: String::from(""),
              latin_name: String::from("Eructavit cor meum"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("My heart is stirring with a noble song;\nlet me recite what I have fashioned for the king; *"),
                      b: String::from("my tongue shall be the pen of a skilled writer.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("You are the fairest of men; *"),
                      b: String::from("grace flows from your lips,\nbecause God has blessed you for ever.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Strap your sword upon your thigh, O mighty warrior, *"),
                      b: String::from("in your pride and in your majesty.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Ride out and conquer in the cause of truth *"),
                      b: String::from("and for the sake of justice.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Your right hand will show you marvelous things; *"),
                      b: String::from("your arrows are very sharp, O mighty warrior.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("The peoples are falling at your feet, *"),
                      b: String::from("and the king’s enemies are losing heart.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Your throne, O God, endures for ever and ever, *"),
                      b: String::from("a scepter of righteousness is the scepter of your kingdom;\nyou love righteousness and hate iniquity.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Therefore God, your God, has anointed you *"),
                      b: String::from("with the oil of gladness above your fellows.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("All your garments are fragrant with myrrh, aloes, and cassia, *"),
                      b: String::from("and the music of strings from ivory palaces makes you glad.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("Kings’ daughters stand among the ladies of the court; *"),
                      b: String::from("on your right hand is the queen,\nadorned with the gold of Ophir.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("“Hear, O daughter; consider and listen closely; *"),
                      b: String::from("forget your people and your father’s house.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("The king will have pleasure in your beauty; *"),
                      b: String::from("he is your master; therefore do him honor.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("The people of Tyre are here with a gift; *"),
                      b: String::from("the rich among the people seek your favor.”")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("All glorious is the princess as she enters; *"),
                      b: String::from("her gown is cloth-of-gold.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("In embroidered apparel she is brought to the king; *"),
                      b: String::from("after her the bridesmaids follow in procession.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("With joy and gladness they are brought, *"),
                      b: String::from("and enter into the palace of the king.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("“In place of fathers, O king, you shall have sons; *"),
                      b: String::from("you shall make them princes over all the earth.")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("I will make your name to be remembered\nfrom one generation to another; *"),
                      b: String::from("therefore nations will praise you for ever and ever.”")
                  },
              ]
            }
        ]
    };
}
