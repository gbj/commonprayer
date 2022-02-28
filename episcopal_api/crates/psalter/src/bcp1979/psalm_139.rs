use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};

lazy_static! {
    pub static ref PSALM_139: Psalm = Psalm {
        number: 139,
        citation: None,
        sections: vec![
        PsalmSection {
            reference: Reference {
              source: Source::BCP1979,
              page: 794
            },
            local_name: String::from(""),
            latin_name: String::from("Domine, probasti"),
            verses: vec![
                          PsalmVerse {
                    number: 1,
                    a: String::from("LORD, you have searched me out and known me; *"),
                    b: String::from("you know my sitting down and my rising up;\nyou discern my thoughts from afar.")
                },
              PsalmVerse {
                    number: 2,
                    a: String::from("You trace my journeys and my resting-places *"),
                    b: String::from("and are acquainted with all my ways.")
                },
              PsalmVerse {
                    number: 3,
                    a: String::from("Indeed, there is not a word on my lips, *"),
                    b: String::from("but you, O LORD, know it altogether.")
                },
              PsalmVerse {
                    number: 4,
                    a: String::from("You press upon me behind and before *"),
                    b: String::from("and lay your hand upon me.")
                },
              PsalmVerse {
                    number: 5,
                    a: String::from("Such knowledge is too wonderful for me; *"),
                    b: String::from("it is so high that I cannot attain to it.")
                },
              PsalmVerse {
                    number: 6,
                    a: String::from("Where can I go then from your Spirit? *"),
                    b: String::from("where can I flee from your presence?")
                },
              PsalmVerse {
                    number: 7,
                    a: String::from("If I climb up to heaven, you are there; *"),
                    b: String::from("if I make the grave my bed, you are there also.")
                },
              PsalmVerse {
                    number: 8,
                    a: String::from("If I take the wings of the morning *"),
                    b: String::from("and dwell in the uttermost parts of the sea,")
                },
              PsalmVerse {
                    number: 9,
                    a: String::from("Even there your hand will lead me *"),
                    b: String::from("and your right hand hold me fast.")
                },
              PsalmVerse {
                    number: 10,
                    a: String::from("If I say, “Surely the darkness will cover me, *"),
                    b: String::from("and the light around me turn to night,”")
                },
              PsalmVerse {
                    number: 11,
                    a: String::from("Darkness is not dark to you;\nthe night is as bright as the day; *"),
                    b: String::from("darkness and light to you are both alike.")
                },
              PsalmVerse {
                    number: 12,
                    a: String::from("For you yourself created my inmost parts; *"),
                    b: String::from("you knit me together in my mother’s womb.")
                },
              PsalmVerse {
                    number: 13,
                    a: String::from("I will thank you because I am marvelously made; *"),
                    b: String::from("your works are wonderful, and I know it well.")
                },
              PsalmVerse {
                    number: 14,
                    a: String::from("My body was not hidden from you, *"),
                    b: String::from("while I was being made in secret\nand woven in the depths of the earth.")
                },
              PsalmVerse {
                    number: 15,
                    a: String::from("Your eyes beheld my limbs, yet unfinished in the womb;\nall of them were written in your book; *"),
                    b: String::from("they were fashioned day by day,\nwhen as yet there was none of them.")
                },
              PsalmVerse {
                    number: 16,
                    a: String::from("How deep I find your thoughts, O God! *"),
                    b: String::from("how great is the sum of them!")
                },
              PsalmVerse {
                    number: 17,
                    a: String::from("If I were to count them, they would be more in number than the sand; *"),
                    b: String::from("to count them all, my life span would need to be like yours.")
                },
              PsalmVerse {
                    number: 18,
                    a: String::from("Oh, that you would slay the wicked, O God! *"),
                    b: String::from("You that thirst for blood, depart from me.")
                },
              PsalmVerse {
                    number: 19,
                    a: String::from("They speak despitefully against you; *"),
                    b: String::from("your enemies take your Name in vain.")
                },
              PsalmVerse {
                    number: 20,
                    a: String::from("Do I not hate those, O LORD, who hate you? *"),
                    b: String::from("and do I not loathe those who rise up against you?")
                },
              PsalmVerse {
                    number: 21,
                    a: String::from("I hate them with a perfect hatred; *"),
                    b: String::from("they have become my own enemies.")
                },
              PsalmVerse {
                    number: 22,
                    a: String::from("Search me out, O God, and know my heart; *"),
                    b: String::from("try me and know my restless thoughts.")
                },
              PsalmVerse {
                    number: 23,
                    a: String::from("Look well whether there be any wickedness in me *"),
                    b: String::from("and lead me in the way that is everlasting.")
                },
            ]
          }

      ]

    };
}
