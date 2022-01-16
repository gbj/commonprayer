use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_142: Psalm = Psalm {
        number: 142,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 748
              },
              local_name: String::from("Psalm 142"),
              latin_name: String::from("Voce mea ad Dominus"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("I cry to the LORD with my voice; *"),
                      b: String::from("to the LORD I make loud supplication.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("I pour out my complaint before him *"),
                      b: String::from("and tell him all my trouble.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("When my spirit languishes within me, you know my path; *"),
                      b: String::from("in the way wherein I walk they have hidden a trap for me.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("I look to my right hand and find no one who knows me; *"),
                      b: String::from("I have no place to flee to, and no one cares for me.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("I cry out to you, O LORD; *"),
                      b: String::from("I say, “You are my refuge,\n my portion in the land of the living.”")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Listen to my cry for help, for I have been brought very low; *"),
                      b: String::from("save me from those who pursue me,\n for they are too strong for me.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Bring me out of prison, that I may give thanks to your Name; *"),
                      b: String::from("when you have dealt bountifully with me,\n the righteous will gather around me.")
                  },
              ]
            }
        ]
    };
}
