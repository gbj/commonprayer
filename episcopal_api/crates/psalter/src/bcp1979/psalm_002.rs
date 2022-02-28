use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_2: Psalm = Psalm {
        number: 2,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 586
            },
            local_name: String::from(""),
            latin_name: String::from("Quare fremuerunt gentes?"),
            verses: vec![
              PsalmVerse {
        number: 1,
        a: String::from("Why are the nations in an uproar? *"),
        b: String::from("Why do the peoples mutter empty threats?")
    },
  PsalmVerse {
        number: 2,
        a: String::from("Why do the kings of the earth rise up in revolt,\nand the princes plot together, *"),
        b: String::from("against the LORD and against his Anointed?")
    },
  PsalmVerse {
        number: 3,
        a: String::from("“Let us break their yoke,” they say; *"),
        b: String::from("“let us cast off their bonds from us.”")
    },
  PsalmVerse {
        number: 4,
        a: String::from("He whose throne is in heaven is laughing; *"),
        b: String::from("the Lord has them in derision.")
    },
  PsalmVerse {
        number: 5,
        a: String::from("Then he speaks to them in his wrath, *"),
        b: String::from("and his rage fills them with terror.")
    },
  PsalmVerse {
        number: 6,
        a: String::from("“I myself have set my king *"),
        b: String::from("upon my holy hill of Zion.”")
    },
  PsalmVerse {
        number: 7,
        a: String::from("Let me announce the decree of the LORD: *"),
        b: String::from("he said to me, “You are my Son;\nthis day have I begotten you.")
    },
  PsalmVerse {
        number: 8,
        a: String::from("Ask of me, and I will give you the nations for your inheritance *"),
        b: String::from("and the ends of the earth for your possession.")
    },
  PsalmVerse {
        number: 9,
        a: String::from("You shall crush them with an iron rod *"),
        b: String::from("and shatter them like a piece of pottery.”")
    },
  PsalmVerse {
        number: 10,
        a: String::from("And now, you kings, be wise; *"),
        b: String::from("be warned, you rulers of the earth.")
    },
  PsalmVerse {
        number: 11,
        a: String::from("Submit to the LORD with fear, *"),
        b: String::from("and with trembling bow before him;")
    },
  PsalmVerse {
        number: 12,
        a: String::from("Lest he be angry and you perish; *"),
        b: String::from("for his wrath is quickly kindled.")
    },
  PsalmVerse {
        number: 13,
        a: String::from("Happy are they all *"),
        b: String::from("who take refuge in him!")
    },
]
        }]
    };
}
