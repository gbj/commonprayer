use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_11: Psalm = Psalm {
        number: 11,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 596
            },
            local_name: String::from("Psalm 11"),
            latin_name: String::from("In Domino confido"),
            verses: vec![
              PsalmVerse {
        number: 1,
        a: String::from("In the LORD have I taken refuge; *"),
        b: String::from("how then can you say to me,\n “Fly away like a bird to the hilltop;")
    },
  PsalmVerse {
        number: 2,
        a: String::from("For see how the wicked bend the bow\nand fit their arrows to the string, *"),
        b: String::from("to shoot from ambush at the true of heart.")
    },
  PsalmVerse {
        number: 3,
        a: String::from("When the foundations are being destroyed, *"),
        b: String::from("what can the righteous do?”")
    },
  PsalmVerse {
        number: 4,
        a: String::from("The LORD is in his holy temple; *"),
        b: String::from("the LORD’S throne is in heaven.")
    },
  PsalmVerse {
        number: 5,
        a: String::from("His eyes behold the inhabited world; *"),
        b: String::from("his piercing eye weighs our worth.")
    },
  PsalmVerse {
        number: 6,
        a: String::from("The LORD weighs the righteous as well as the wicked, *"),
        b: String::from("but those who delight in violence he abhors.")
    },
  PsalmVerse {
        number: 7,
        a: String::from("Upon the wicked he shall rain coals of fire and burning sulphur; *"),
        b: String::from("a scorching wind shall be their lot.")
    },
  PsalmVerse {
        number: 8,
        a: String::from("For the LORD is righteous;\nhe delights in righteous deeds; *"),
        b: String::from("and the just shall see his face.")
    },
]
        }]
    };
}
