use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_85: Psalm = Psalm {
        number: 85,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 708
              },
              local_name: String::from("Psalm 85"),
              latin_name: String::from("Benedixisti, Domine"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("You have been gracious to your land, O LORD, *"),
                      b: String::from("you have restored the good fortune of Jacob.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("You have forgiven the iniquity of your people *"),
                      b: String::from("and blotted out all their sins.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("You have withdrawn all your fury *"),
                      b: String::from("and turned yourself from your wrathful indignation.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Restore us then, O God our Savior; *"),
                      b: String::from("let your anger depart from us.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Will you be displeased with us for ever? *"),
                      b: String::from("will you prolong your anger from age to age?")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("Will you not give us life again, *"),
                      b: String::from("that your people may rejoice in you?")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Show us your mercy, O LORD, *"),
                      b: String::from("and grant us your salvation.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("I will listen to what the LORD God is saying, *"),
                      b: String::from("for he is speaking peace to his faithful people\n and to those who turn their hearts to him.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Truly, his salvation is very near to those who fear him, *"),
                      b: String::from("that his glory may dwell in our land.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("Mercy and truth have met together; *"),
                      b: String::from("righteousness and peace have kissed each other.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Truth shall spring up from the earth, *"),
                      b: String::from("and righteousness shall look down from heaven.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("The LORD will indeed grant prosperity, *"),
                      b: String::from("and our land will yield its increase.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("Righteousness shall go before him, *"),
                      b: String::from("and peace shall be a pathway for his feet.")
                  },
              ]
            }
        ]
    };
}
