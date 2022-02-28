use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_112: Psalm = Psalm {
        number: 112,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 755
              },
              local_name: String::from(""),
              latin_name: String::from("Beatus vir"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Hallelujah!\nHappy are they who fear the Lord *"),
                      b: String::from("and have great delight in his commandments!")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Their descendants will be mighty in the land; *"),
                      b: String::from("the generation of the upright will be blessed.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Wealth and riches will be in their house, *"),
                      b: String::from("and their righteousness will last for ever.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Light shines in the darkness for the upright; *"),
                      b: String::from("the righteous are merciful and full of compassion.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("It is good for them to be generous in lending *"),
                      b: String::from("and to manage their affairs with justice.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("For they will never be shaken; *"),
                      b: String::from("the righteous will be kept in everlasting remembrance.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("They will not be afraid of any evil rumors; *"),
                      b: String::from("their heart is right;\nthey put their trust in the Lord.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Their heart is established and will not shrink, *"),
                      b: String::from("until they see their desire upon their enemies.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("They have given freely to the poor, *"),
                      b: String::from("and their righteousness stands fast for ever;\nthey will hold up their head with honor.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("The wicked will see it and be angry;\nthey will gnash their teeth and pine away; *"),
                      b: String::from("the desires of the wicked will perish.")
                  },
              ]
            }
        ]
    };
}
