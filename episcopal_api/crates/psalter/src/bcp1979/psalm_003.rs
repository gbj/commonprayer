use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_3: Psalm = Psalm {
        number: 3,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 587
            },
            local_name: String::from("Psalm 3"),
            latin_name: String::from("Domine, quid multiplicati"),
            verses: vec![
              PsalmVerse {
        number: 1,
        a: String::from("LORD, how many adversaries I have! *"),
        b: String::from("how many there are who rise up against me!")
    },
  PsalmVerse {
        number: 2,
        a: String::from("How many there are who say of me, *"),
        b: String::from("“There is no help for him in his God.”")
    },
  PsalmVerse {
        number: 3,
        a: String::from("But you, O LORD, are a shield about me; *"),
        b: String::from("you are my glory, the one who lifts up my head.")
    },
  PsalmVerse {
        number: 4,
        a: String::from("I call aloud upon the LORD, *"),
        b: String::from("and he answers me from his holy hill;")
    },
  PsalmVerse {
        number: 5,
        a: String::from("I lie down and go to sleep; *"),
        b: String::from("I wake again, because the LORD sustains me.")
    },
  PsalmVerse {
        number: 6,
        a: String::from("I do not fear the multitudes of people *"),
        b: String::from("who set themselves against me all around.")
    },
  PsalmVerse {
        number: 7,
        a: String::from("Rise up, O LORD; set me free, O my God; *"),
        b: String::from("surely, you will strike all my enemies across the face,\n you will break the teeth of the wicked.")
    },
  PsalmVerse {
        number: 8,
        a: String::from("Deliverance belongs to the LORD. *"),
        b: String::from("Your blessing be upon your people!")
    },
]
        }]
    };
}
