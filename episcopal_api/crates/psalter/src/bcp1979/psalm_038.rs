use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_38: Psalm = Psalm {
        number: 38,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 636
            },
            local_name: String::from("Psalm 38"),
            latin_name: String::from("Domine, ne in furore"),
            verses: vec![
              PsalmVerse {
                number: 1,
                a: String::from("O LORD, do not rebuke me in your anger; *"),
                b: String::from("do not punish me in your wrath.")
            },
          PsalmVerse {
                number: 2,
                a: String::from("For your arrows have already pierced me, *"),
                b: String::from("and your hand presses hard upon me.")
            },
          PsalmVerse {
                number: 3,
                a: String::from("There is no health in my flesh,\nbecause of your indignation; *"),
                b: String::from("there is no soundness in my body, because of my sin.")
            },
          PsalmVerse {
                number: 4,
                a: String::from("For my iniquities overwhelm me; *"),
                b: String::from("like a heavy burden they are too much for me to bear.")
            },
          PsalmVerse {
                number: 5,
                a: String::from("My wounds stink and fester *"),
                b: String::from("by reason of my foolishness.")
            },
          PsalmVerse {
                number: 6,
                a: String::from("I am utterly bowed down and prostrate; *"),
                b: String::from("I go about in mourning all the day long.")
            },
          PsalmVerse {
                number: 7,
                a: String::from("My loins are filled with searing pain; *"),
                b: String::from("there is no health in my body.")
            },
          PsalmVerse {
                number: 8,
                a: String::from("I am utterly numb and crushed; *"),
                b: String::from("I wail, because of the groaning of my heart.")
            },
          PsalmVerse {
                number: 9,
                a: String::from("O Lord, you know all my desires, *"),
                b: String::from("and my sighing is not hidden from you.")
            },
          PsalmVerse {
                number: 10,
                a: String::from("My heart is pounding, my strength has failed me, *"),
                b: String::from("and the brightness of my eyes is gone from me.")
            },
          PsalmVerse {
                number: 11,
                a: String::from("My friends and companions draw back from my affliction; *"),
                b: String::from("my neighbors stand afar off.")
            },
          PsalmVerse {
                number: 12,
                a: String::from("Those who seek after my life lay snares for me; *"),
                b: String::from("those who strive to hurt me speak of my ruin\n and plot treachery all the day long.")
            },
          PsalmVerse {
                number: 13,
                a: String::from("But I am like the deaf who do not hear, *"),
                b: String::from("like those who are mute and who do not open their mouth.")
            },
          PsalmVerse {
                number: 14,
                a: String::from("I have become like one who does not hear *"),
                b: String::from("and from whose mouth comes no defense.")
            },
          PsalmVerse {
                number: 15,
                a: String::from("For in you, O LORD, have I fixed my hope; *"),
                b: String::from("you will answer me, O Lord my God.")
            },
          PsalmVerse {
                number: 16,
                a: String::from("For I said, “Do not let them rejoice at my expense, *"),
                b: String::from("those who gloat over me when my foot slips.”")
            },
          PsalmVerse {
                number: 17,
                a: String::from("Truly, I am on the verge of falling, *"),
                b: String::from("and my pain is always with me.")
            },
          PsalmVerse {
                number: 18,
                a: String::from("I will confess my iniquity *"),
                b: String::from("and be sorry for my sin.")
            },
          PsalmVerse {
                number: 19,
                a: String::from("Those who are my enemies without cause are mighty, *"),
                b: String::from("and many in number are those who wrongfully hate me.")
            },
          PsalmVerse {
                number: 20,
                a: String::from("Those who repay evil for good slander me, *"),
                b: String::from("because I follow the course that is right.")
            },
          PsalmVerse {
                number: 21,
                a: String::from("O LORD, do not forsake me; *"),
                b: String::from("be not far from me, O my God.")
            },
          PsalmVerse {
                number: 22,
                a: String::from("Make haste to help me, *"),
                b: String::from("O Lord of my salvation.")
            },
            ]
        }]
    };
}
