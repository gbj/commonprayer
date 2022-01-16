use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_24: Psalm = Psalm {
        number: 24,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 613
              },
              local_name: String::from("Psalm 24"),
              latin_name: String::from("Domini est terra"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("The earth is the LORD’S and all that is in it, *"),
                      b: String::from("the world and all who dwell therein.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("For it is he who founded it upon the seas *"),
                      b: String::from("and made it firm upon the rivers of the deep.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("“Who can ascend the hill of the LORD? *"),
                      b: String::from("and who can stand in his holy place?”")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("“Those who have clean hands and a pure heart, *"),
                      b: String::from("who have not pledged themselves to falsehood,\n nor sworn by what is a fraud.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("They shall receive a blessing from the LORD *"),
                      b: String::from("and a just reward from the God of their salvation.”")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Such is the generation of those who seek him, *"),
                      b: String::from("of those who seek your face, O God of Jacob.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Lift up your heads, O gates;\nlift them high, O everlasting doors; *"),
                      b: String::from("and the King of glory shall come in.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("“Who is this King of glory?” *"),
                      b: String::from("“The LORD, strong and mighty,\n the LORD, mighty in battle.”")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Lift up your heads, O gates;\nlift them high, O everlasting doors; *"),
                      b: String::from("and the King of glory shall come in.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("“Who is he, this King of glory?” *"),
                      b: String::from("“The LORD of hosts,\n he is the King of glory.”")
                  },
              ]
            }
        ]
    };
}
