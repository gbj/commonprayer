use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_57: Psalm = Psalm {
        number: 57,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 663
              },
              local_name: String::from(""),
              latin_name: String::from("Miserere mei, Deus"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Be merciful to me, O God, be merciful,\nfor I have taken refuge in you; *"),
                      b: String::from("in the shadow of your wings will I take refuge\nuntil this time of trouble has gone by.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("I will call upon the Most High God, *"),
                      b: String::from("the God who maintains my cause.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("He will send from heaven and save me;\nhe will confound those who trample upon me; *"),
                      b: String::from("God will send forth his love and his faithfulness.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("I lie in the midst of lions that devour the people; *"),
                      b: String::from("their teeth are spears and arrows,\ntheir tongue a sharp sword.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("They have laid a net for my feet,\nand I am bowed low; *"),
                      b: String::from("they have dug a pit before me,\nbut have fallen into it themselves.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Exalt yourself above the heavens, O God, *"),
                      b: String::from("and your glory over all the earth.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("My heart is firmly fixed, O God, my heart is fixed; *"),
                      b: String::from("I will sing and make melody.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Wake up, my spirit;\nawake, lute and harp; *"),
                      b: String::from("I myself will waken the dawn.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("I will confess you among the peoples, O LORD; *"),
                      b: String::from("I will sing praise to you among the nations.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("For your loving-kindness is greater than the heavens, *"),
                      b: String::from("and your faithfulness reaches to the clouds.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Exalt yourself above the heavens, O God, *"),
                      b: String::from("and your glory over all the earth.")
                  },
              ]
            }
        ]
    };
}
