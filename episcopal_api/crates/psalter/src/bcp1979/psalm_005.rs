use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};

lazy_static! {
    pub static ref PSALM_5: Psalm = Psalm {
        number: 5,
        citation: None,
        sections: vec! [
        PsalmSection {
            reference: Reference {
              source: Source::BCP1979,
              page: 588
            },
            local_name: String::from("Psalm 5"),
            latin_name: String::from("Verba mea auribus"),
            verses: vec![
                          PsalmVerse {
                    number: 1,
                    a: String::from("Give ear to my words, O LORD; *"),
                    b: String::from("consider my meditation.")
                },
              PsalmVerse {
                    number: 2,
                    a: String::from("Hearken to my cry for help, my King and my God, *"),
                    b: String::from("for I make my prayer to you.")
                },
              PsalmVerse {
                    number: 3,
                    a: String::from("In the morning, LORD, you hear my voice; *"),
                    b: String::from("early in the morning I make my appeal and watch for you.")
                },
              PsalmVerse {
                    number: 4,
                    a: String::from("For you are not a God who takes pleasure in wickedness, *"),
                    b: String::from("and evil cannot dwell with you.")
                },
              PsalmVerse {
                    number: 5,
                    a: String::from("Braggarts cannot stand in your sight; *"),
                    b: String::from("you hate all those who work wickedness.")
                },
              PsalmVerse {
                    number: 6,
                    a: String::from("You destroy those who speak lies; *"),
                    b: String::from("the bloodthirsty and deceitful, O LORD, you abhor.")
                },
              PsalmVerse {
                    number: 7,
                    a: String::from("But as for me, through the greatness of your mercy I will go into your house; *"),
                    b: String::from("I will bow down toward your holy temple in awe of you.")
                },
              PsalmVerse {
                    number: 8,
                    a: String::from("Lead me, O LORD, in your righteousness,\nbecause of those who lie in wait for me; *"),
                    b: String::from("make your way straight before me.")
                },
              PsalmVerse {
                    number: 9,
                    a: String::from("For there is no truth in their mouth; *"),
                    b: String::from("there is destruction in their heart;")
                },
              PsalmVerse {
                    number: 10,
                    a: String::from("Their throat is an open grave; *"),
                    b: String::from("they flatter with their tongue.")
                },
              PsalmVerse {
                    number: 11,
                    a: String::from("Declare them guilty, O God; *"),
                    b: String::from("let them fall, because of their schemes.")
                },
              PsalmVerse {
                    number: 12,
                    a: String::from("Because of their many transgressions cast them out, *"),
                    b: String::from("for they have rebelled against you.")
                },
              PsalmVerse {
                    number: 13,
                    a: String::from("But all who take refuge in you will be glad; *"),
                    b: String::from("they will sing out their joy for ever.")
                },
              PsalmVerse {
                    number: 14,
                    a: String::from("You will shelter them, *"),
                    b: String::from("so that those who love your Name may exult in you.")
                },
              PsalmVerse {
                    number: 15,
                    a: String::from("For you, O LORD, will bless the righteous; *"),
                    b: String::from("you will defend them with your favor as with a shield.")
                },
            ]
          }
      ]
    };
}
