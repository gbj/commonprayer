use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_110: Psalm = Psalm {
        number: 110,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 753
              },
              local_name: String::from(""),
              latin_name: String::from("Dixit Dominus"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("The LORD said to my Lord, “Sit at my right hand, *"),
                      b: String::from("until I make your enemies your footstool.”")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("The LORD will send the scepter of your power out of Zion, *"),
                      b: String::from("saying, “Rule over your enemies round about you.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Princely state has been yours from the day of your birth; *"),
                      b: String::from("in the beauty of holiness have I begotten you,\nlike dew from the womb of the morning.”")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("The LORD has sworn and he will not recant: *"),
                      b: String::from("“You are a priest for ever after the order of Melchizedek.”")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("The Lord who is at your right hand\nwill smite kings in the day of his wrath; *"),
                      b: String::from("he will rule over the nations.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("He will heap high the corpses; *"),
                      b: String::from("he will smash heads over the wide earth.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("He will drink from the brook beside the road; *"),
                      b: String::from("therefore he will lift high his head.")
                  },
              ]
            }
        ]
    };
}
