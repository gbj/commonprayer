use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_127: Psalm = Psalm {
        number: 127,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 782
              },
              local_name: String::from(""),
              latin_name: String::from("Nisi Dominus"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Unless the LORD builds the house, *"),
                      b: String::from("their labor is in vain who build it.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Unless the LORD watches over the city, *"),
                      b: String::from("in vain the watchman keeps his vigil.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("It is in vain that you rise so early and go to bed so late; *"),
                      b: String::from("vain, too, to eat the bread of toil,\nfor he gives to his beloved sleep.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Children are a heritage from the LORD, *"),
                      b: String::from("and the fruit of the womb is a gift.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Like arrows in the hand of a warrior *"),
                      b: String::from("are the children of one’s youth.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Happy is the man who has his quiver full of them! *"),
                      b: String::from("he shall not be put to shame\nwhen he contends with his enemies in the gate.")
                  },
              ]
            }
        ]
    };
}
