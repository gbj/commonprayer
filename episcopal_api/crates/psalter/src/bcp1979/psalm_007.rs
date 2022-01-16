use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_7: Psalm = Psalm {
        number: 7,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 590
            },
            local_name: String::from("Psalm 7"),
            latin_name: String::from("Domine, Deus meus"),
            verses: vec![
              PsalmVerse {
        number: 1,
        a: String::from("O LORD my God, I take refuge in you; *"),
        b: String::from("save and deliver me from all who pursue me;")
    },
  PsalmVerse {
        number: 2,
        a: String::from("Lest like a lion they tear me in pieces *"),
        b: String::from("and snatch me away with none to deliver me.")
    },
  PsalmVerse {
        number: 3,
        a: String::from("O LORD my God, if I have done these things: *"),
        b: String::from("if there is any wickedness in my hands,")
    },
  PsalmVerse {
        number: 4,
        a: String::from("If I have repaid my friend with evil, *"),
        b: String::from("or plundered him who without cause is my enemy;")
    },
  PsalmVerse {
        number: 5,
        a: String::from("Then let my enemy pursue and overtake me, *"),
        b: String::from("trample my life into the ground,\n and lay my honor in the dust.")
    },
  PsalmVerse {
        number: 6,
        a: String::from("Stand up, O LORD, in your wrath; *"),
        b: String::from("rise up against the fury of my enemies.")
    },
  PsalmVerse {
        number: 7,
        a: String::from("Awake, O my God, decree justice; *"),
        b: String::from("let the assembly of the peoples gather round you.")
    },
  PsalmVerse {
        number: 8,
        a: String::from("Be seated on your lofty throne, O Most High; *"),
        b: String::from("O LORD, judge the nations.")
    },
  PsalmVerse {
        number: 9,
        a: String::from("Give judgment for me according to my righteousness, O LORD, *"),
        b: String::from("and according to my innocence, O Most High.")
    },
  PsalmVerse {
        number: 10,
        a: String::from("Let the malice of the wicked come to an end,\nbut establish the righteous; *"),
        b: String::from("for you test the mind and heart, O righteous God.")
    },
  PsalmVerse {
        number: 11,
        a: String::from("God is my shield and defense; *"),
        b: String::from("he is the savior of the true in heart.")
    },
  PsalmVerse {
        number: 12,
        a: String::from("God is a righteous judge; *"),
        b: String::from("God sits in judgment every day.")
    },
  PsalmVerse {
        number: 13,
        a: String::from("If they will not repent, God will whet his sword; *"),
        b: String::from("he will bend his bow and make it ready.")
    },
  PsalmVerse {
        number: 14,
        a: String::from("He has prepared his weapons of death; *"),
        b: String::from("he makes his arrows shafts of fire.")
    },
  PsalmVerse {
        number: 15,
        a: String::from("Look at those who are in labor with wickedness, *"),
        b: String::from("who conceive evil, and give birth to a lie.")
    },
  PsalmVerse {
        number: 16,
        a: String::from("They dig a pit and make it deep *"),
        b: String::from("and fall into the hole that they have made.")
    },
  PsalmVerse {
        number: 17,
        a: String::from("Their malice turns back upon their own head; *"),
        b: String::from("their violence falls on their own scalp.")
    },
  PsalmVerse {
        number: 18,
        a: String::from("I will bear witness that the LORD is righteous; *"),
        b: String::from("I will praise the Name of the LORD Most High.")
    },
]
        }]
    };
}
