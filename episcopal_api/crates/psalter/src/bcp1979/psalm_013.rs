use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_13: Psalm = Psalm {
        number: 13,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 597
              },
              local_name: String::from(""),
              latin_name: String::from("Usquequo, Domine?"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("How long, O LORD?\nwill you forget me for ever? *"),
                      b: String::from("how long will you hide your face from me?")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("How long shall I have perplexity in my mind,\nand grief in my heart, day after day? *"),
                      b: String::from("how long shall my enemy triumph over me?")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Look upon me and answer me, O LORD my God; *"),
                      b: String::from("give light to my eyes, lest I sleep in death;")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Lest my enemy say, “I have prevailed over him,” *"),
                      b: String::from("and my foes rejoice that I have fallen.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("But I put my trust in your mercy; *"),
                      b: String::from("my heart is joyful because of your saving help.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("I will sing to the LORD, for he has dealt with me richly; *"),
                      b: String::from("I will praise the Name of the Lord Most High.")
                  },
              ]
            }
        ]
    };
}
