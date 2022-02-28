use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};

lazy_static! {
    pub static ref PSALM_98: Psalm = Psalm {
        number: 98,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 727
              },
              local_name: String::from(""),
              latin_name: String::from("Cantate Domino"),
              verses: vec![
                            PsalmVerse {
                      number: 1,
                      a: String::from("Sing to the LORD a new song, *"),
                      b: String::from("for he has done marvelous things.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("With his right hand and his holy arm *"),
                      b: String::from("has he won for himself the victory.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("The LORD has made known his victory; *"),
                      b: String::from("his righteousness has he openly shown in the sight of the nations.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("He remembers his mercy and faithfulness to the house of Israel, *"),
                      b: String::from("and all the ends of the earth have seen the victory of our God.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Shout with joy to the LORD, all you lands; *"),
                      b: String::from("lift up your voice, rejoice, and sing.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Sing to the LORD with the harp, *"),
                      b: String::from("with the harp and the voice of song.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("With trumpets and the sound of the horn *"),
                      b: String::from("shout with joy before the King, the LORD.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Let the sea make a noise and all that is in it, *"),
                      b: String::from("the lands and those who dwell therein.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Let the rivers clap their hands, *"),
                      b: String::from("and let the hills ring out with joy before the LORD,\nwhen he comes to judge the earth.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("In righteousness shall he judge the world *"),
                      b: String::from("and the peoples with equity.")
                  },
              ]
            }

        ]

    };
}
