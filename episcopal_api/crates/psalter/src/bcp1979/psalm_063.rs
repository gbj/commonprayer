use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_63: Psalm = Psalm {
        number: 63,
        citation: None,
        sections: vec![
          PsalmSection {
            reference: Reference {
              source: Source::BCP1979,
              page: 670
            },
            local_name: String::from(""),
            latin_name: String::from("Deus, Deus meus"),
            verses: vec![
                          PsalmVerse {
                    number: 1,
                    a: String::from("O God, you are my God; eagerly I seek you; *"),
                    b: String::from("my soul thirsts for you, my flesh faints for you,\nas in a barren and dry land where there is no water.")
                },
              PsalmVerse {
                    number: 2,
                    a: String::from("Therefore I have gazed upon you in your holy place, *"),
                    b: String::from("that I might behold your power and your glory.")
                },
              PsalmVerse {
                    number: 3,
                    a: String::from("For your loving-kindness is better than life itself; *"),
                    b: String::from("my lips shall give you praise.")
                },
              PsalmVerse {
                    number: 4,
                    a: String::from("So will I bless you as long as I live *"),
                    b: String::from("and lift up my hands in your Name.")
                },
              PsalmVerse {
                    number: 5,
                    a: String::from("My soul is content, as with marrow and fatness, *"),
                    b: String::from("and my mouth praises you with joyful lips,")
                },
              PsalmVerse {
                    number: 6,
                    a: String::from("When I remember you upon my bed, *"),
                    b: String::from("and meditate on you in the night watches.")
                },
              PsalmVerse {
                    number: 7,
                    a: String::from("For you have been my helper, *"),
                    b: String::from("and under the shadow of your wings I will rejoice.")
                },
              PsalmVerse {
                    number: 8,
                    a: String::from("My soul clings to you; *"),
                    b: String::from("your right hand holds me fast.")
                },
              PsalmVerse {
                    number: 9,
                    a: String::from("May those who seek my life to destroy it *"),
                    b: String::from("go down into the depths of the earth;")
                },
              PsalmVerse {
                    number: 10,
                    a: String::from("Let them fall upon the edge of the sword, *"),
                    b: String::from("and let them be food for jackals.")
                },
              PsalmVerse {
                    number: 11,
                    a: String::from("But the king will rejoice in God;\nall those who swear by him will be glad; *"),
                    b: String::from("for the mouth of those who speak lies shall be stopped.")
                },
            ]
          }
        ]
    };
}
