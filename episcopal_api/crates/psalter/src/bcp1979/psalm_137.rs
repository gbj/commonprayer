use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_137: Psalm = Psalm {
        number: 137,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 792
              },
              local_name: String::from(""),
              latin_name: String::from("Super flumina"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("By the waters of Babylon we sat down and wept, *"),
                      b: String::from("when we remembered you, O Zion.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("As for our harps, we hung them up *"),
                      b: String::from("on the trees in the midst of that land.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("For those who led us away captive asked us for a song,\nand our oppressors called for mirth: *"),
                      b: String::from("“Sing us one of the songs of Zion.”")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("How shall we sing the LORD’s song *"),
                      b: String::from("upon an alien soil?")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("If I forget you, O Jerusalem, *"),
                      b: String::from("let my right hand forget its skill.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Let my tongue cleave to the roof of my mouth\nif I do not remember you, *"),
                      b: String::from("if I do not set Jerusalem above my highest joy.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Remember the day of Jerusalem, O LORD,\nagainst the people of Edom, *"),
                      b: String::from("who said, “Down with it! down with it!\neven to the ground!”")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("O Daughter of Babylon, doomed to destruction, *"),
                      b: String::from("happy the one who pays you back\nfor what you have done to us!")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Happy shall he be who takes your little ones, *"),
                      b: String::from("and dashes them against the rock!")
                  },
              ]
            }
        ]
    };
}
