use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_51: Psalm = Psalm {
        number: 51,
        citation: None,
        sections: vec![
          PsalmSection {
            reference: Reference {
              source: Source::BCP1979,
              page: 656
            },
            local_name: String::from(""),
            latin_name: String::from("Miserere mei, Deus"),
            verses: vec![
                          PsalmVerse {
                    number: 1,
                    a: String::from("Have mercy on me, O God, according to your loving-kindness; *"),
                    b: String::from("in your great compassion blot out my offenses.")
                },
              PsalmVerse {
                    number: 2,
                    a: String::from("Wash me through and through from my wickedness *"),
                    b: String::from("and cleanse me from my sin.")
                },
              PsalmVerse {
                    number: 3,
                    a: String::from("For I know my transgressions, *"),
                    b: String::from("and my sin is ever before me.")
                },
              PsalmVerse {
                    number: 4,
                    a: String::from("Against you only have I sinned *"),
                    b: String::from("and done what is evil in your sight.")
                },
              PsalmVerse {
                    number: 5,
                    a: String::from("And so you are justified when you speak *"),
                    b: String::from("and upright in your judgment.")
                },
              PsalmVerse {
                    number: 6,
                    a: String::from("Indeed, I have been wicked from my birth, *"),
                    b: String::from("a sinner from my mother’s womb.")
                },
              PsalmVerse {
                    number: 7,
                    a: String::from("For behold, you look for truth deep within me, *"),
                    b: String::from("and will make me understand wisdom secretly.")
                },
              PsalmVerse {
                    number: 8,
                    a: String::from("Purge me from my sin, and I shall be pure; *"),
                    b: String::from("wash me, and I shall be clean indeed.")
                },
              PsalmVerse {
                    number: 9,
                    a: String::from("Make me hear of joy and gladness, *"),
                    b: String::from("that the body you have broken may rejoice.")
                },
              PsalmVerse {
                    number: 10,
                    a: String::from("Hide your face from my sins *"),
                    b: String::from("and blot out all my iniquities.")
                },
              PsalmVerse {
                    number: 11,
                    a: String::from("Create in me a clean heart, O God, *"),
                    b: String::from("and renew a right spirit within me.")
                },
              PsalmVerse {
                    number: 12,
                    a: String::from("Cast me not away from your presence *"),
                    b: String::from("and take not your holy Spirit from me.")
                },
              PsalmVerse {
                    number: 13,
                    a: String::from("Give me the joy of your saving help again *"),
                    b: String::from("and sustain me with your bountiful Spirit.")
                },
              PsalmVerse {
                    number: 14,
                    a: String::from("I shall teach your ways to the wicked, *"),
                    b: String::from("and sinners shall return to you.")
                },
              PsalmVerse {
                    number: 15,
                    a: String::from("Deliver me from death, O God, *"),
                    b: String::from("and my tongue shall sing of your righteousness,\nO God of my salvation.")
                },
              PsalmVerse {
                    number: 16,
                    a: String::from("Open my lips, O Lord, *"),
                    b: String::from("and my mouth shall proclaim your praise.")
                },
              PsalmVerse {
                    number: 17,
                    a: String::from("Had you desired it, I would have offered sacrifice, *"),
                    b: String::from("but you take no delight in burnt-offerings.")
                },
              PsalmVerse {
                    number: 18,
                    a: String::from("The sacrifice of God is a troubled spirit; *"),
                    b: String::from("a broken and contrite heart, O God, you will not despise.")
                },
              PsalmVerse {
                    number: 19,
                    a: String::from("Be favorable and gracious to Zion, *"),
                    b: String::from("and rebuild the walls of Jerusalem.")
                },
              PsalmVerse {
                    number: 20,
                    a: String::from("Then you will be pleased with the appointed sacrifices,\nwith burnt-offerings and oblations; *"),
                    b: String::from("then shall they offer young bullocks upon your altar.")
                },
            ]
          }
        ]
    };
}
