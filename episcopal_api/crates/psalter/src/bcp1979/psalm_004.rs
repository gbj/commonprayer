use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_4: Psalm = Psalm {
        number: 4,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 587
            },
            local_name: String::from("Psalm 4"),
            latin_name: String::from("Cum invocarem"),
            verses: vec![
              PsalmVerse {
        number: 1,
        a: String::from("Answer me when I call, O God, defender of my cause; *"),
        b: String::from("you set me free when I am hard-pressed;\n have mercy on me and hear my prayer.")
    },
  PsalmVerse {
        number: 2,
        a: String::from("“You mortals, how long will you dishonor my glory; *"),
        b: String::from("how long will you worship dumb idols\n and run after false gods?”")
    },
  PsalmVerse {
        number: 3,
        a: String::from("Know that the LORD does wonders for the faithful; *"),
        b: String::from("when I call upon the LORD, he will hear me.")
    },
  PsalmVerse {
        number: 4,
        a: String::from("Tremble, then, and do not sin; *"),
        b: String::from("speak to your heart in silence upon your bed.")
    },
  PsalmVerse {
        number: 5,
        a: String::from("Offer the appointed sacrifices *"),
        b: String::from("and put your trust in the LORD.")
    },
  PsalmVerse {
        number: 6,
        a: String::from("Many are saying,\n“Oh, that we might see better times!” *"),
        b: String::from("Lift up the light of your countenance upon us, O LORD.")
    },
  PsalmVerse {
        number: 7,
        a: String::from("You have put gladness in my heart, *"),
        b: String::from("more than when grain and wine and oil increase.")
    },
  PsalmVerse {
        number: 8,
        a: String::from("I lie down in peace; at once I fall asleep; *"),
        b: String::from("for only you, LORD, make me dwell in safety.")
    },
]
        }]
    };
}
