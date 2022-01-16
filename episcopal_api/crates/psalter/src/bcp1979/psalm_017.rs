use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_17: Psalm = Psalm {
        number: 17,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 600
              },
              local_name: String::from("Psalm 17"),
              latin_name: String::from("Exaudi, Domine"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Hear my plea of innocence, O LORD;\ngive heed to my cry; *"),
                      b: String::from("listen to my prayer, which does not come from lying lips.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Let my vindication come forth from your presence; *"),
                      b: String::from("let your eyes be fixed on justice.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Weigh my heart, summon me by night, *"),
                      b: String::from("melt me down; you will find no impurity in me.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("I give no offense with my mouth as others do; *"),
                      b: String::from("I have heeded the words of your lips.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("My footsteps hold fast to the ways of your law; *"),
                      b: String::from("in your paths my feet shall not stumble.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("I call upon you, O God, for you will answer me; *"),
                      b: String::from("incline your ear to me and hear my words.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Show me your marvelous loving-kindness, *"),
                      b: String::from("O Savior of those who take refuge at your right hand\n from those who rise up against them.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Keep me as the apple of your eye; *"),
                      b: String::from("hide me under the shadow of your wings,")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("From the wicked who assault me, *"),
                      b: String::from("from my deadly enemies who surround me.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("They have closed their heart to pity, *"),
                      b: String::from("and their mouth speaks proud things.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("They press me hard,\nnow they surround me, *"),
                      b: String::from("watching how they may cast me to the ground,")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Like a lion, greedy for its prey, *"),
                      b: String::from("and like a young lion lurking in secret places.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("Arise, O LORD; confront them and bring them down; *"),
                      b: String::from("deliver me from the wicked by your sword.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("Deliver me, O LORD, by your hand *"),
                      b: String::from("from those whose portion in life is this world;")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("Whose bellies you fill with your treasure, *"),
                      b: String::from("who are well supplied with children\n and leave their wealth to their little ones.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("But at my vindication I shall see your face; *"),
                      b: String::from("when I awake, I shall be satisfied, beholding your likeness.")
                  },
              ]
            }
        ]
    };
}
