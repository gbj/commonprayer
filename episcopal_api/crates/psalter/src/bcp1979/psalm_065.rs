use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_65: Psalm = Psalm {
        number: 65,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 672
              },
              local_name: String::from(""),
              latin_name: String::from("Te decet hymnus"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("You are to be praised, O God, in Zion; *"),
                      b: String::from("to you shall vows be performed in Jerusalem.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("To you that hear prayer shall all flesh come, *"),
                      b: String::from("because of their transgressions.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Our sins are stronger than we are, *"),
                      b: String::from("but you will blot them out.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Happy are they whom you choose\nand draw to your courts to dwell there! *"),
                      b: String::from("they will be satisfied by the beauty of your house,\nby the holiness of your temple.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Awesome things will you show us in your righteousness,\nO God of our salvation, *"),
                      b: String::from("O Hope of all the ends of the earth\nand of the seas that are far away.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("You make fast the mountains by your power; *"),
                      b: String::from("they are girded about with might.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("You still the roaring of the seas, *"),
                      b: String::from("the roaring of their waves,\nand the clamor of the peoples.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Those who dwell at the ends of the earth will tremble at your marvelous signs; *"),
                      b: String::from("you make the dawn and the dusk to sing for joy.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("You visit the earth and water it abundantly;\nyou make it very plenteous; *"),
                      b: String::from("the river of God is full of water.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("You prepare the grain, *"),
                      b: String::from("for so you provide for the earth.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("You drench the furrows and smooth out the ridges; *"),
                      b: String::from("with heavy rain you soften the ground and bless its increase.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("You crown the year with your goodness, *"),
                      b: String::from("and your paths overflow with plenty.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("May the fields of the wilderness be rich for grazing, *"),
                      b: String::from("and the hills be clothed with joy.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("May the meadows cover themselves with flocks,\nand the valleys cloak themselves with grain; *"),
                      b: String::from("let them shout for joy and sing.")
                  },
              ]
            }
        ]
    };
}
