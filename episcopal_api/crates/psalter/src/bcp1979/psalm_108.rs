use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_108: Psalm = Psalm {
        number: 108,
        citation: None,
        sections: vec![
          PsalmSection {
            reference: Reference {
              source: Source::BCP1979,
              page: 749
            },
            local_name: String::from("Psalm 108"),
            latin_name: String::from("Paratum cor meum"),
            verses: vec![
                          PsalmVerse {
                    number: 1,
                    a: String::from("My heart is firmly fixed, O God, my heart is fixed; *"),
                    b: String::from("I will sing and make melody.")
                },
              PsalmVerse {
                    number: 2,
                    a: String::from("Wake up, my spirit;\nawake, lute and harp; *"),
                    b: String::from("I myself will waken the dawn.")
                },
              PsalmVerse {
                    number: 3,
                    a: String::from("I will confess you among the peoples, O LORD; *"),
                    b: String::from("I will sing praises to you among the nations.")
                },
              PsalmVerse {
                    number: 4,
                    a: String::from("For your loving-kindness is greater than the heavens, *"),
                    b: String::from("and your faithfulness reaches to the clouds.")
                },
              PsalmVerse {
                    number: 5,
                    a: String::from("Exalt yourself above the heavens, O God, *"),
                    b: String::from("and your glory over all the earth.")
                },
              PsalmVerse {
                    number: 6,
                    a: String::from("So that those who are dear to you may be delivered, *"),
                    b: String::from("save with your right hand and answer me.")
                },
              PsalmVerse {
                    number: 7,
                    a: String::from("God spoke from his holy place and said, *"),
                    b: String::from("“I will exult and parcel out Shechem;\n I will divide the valley of Succoth.")
                },
              PsalmVerse {
                    number: 8,
                    a: String::from("Gilead is mine and Manasseh is mine; *"),
                    b: String::from("Ephraim is my helmet and Judah my scepter.")
                },
              PsalmVerse {
                    number: 9,
                    a: String::from("Moab is my washbasin,\non Edom I throw down my sandal to claim it, *"),
                    b: String::from("and over Philistia will I shout in triumph.”")
                },
              PsalmVerse {
                    number: 10,
                    a: String::from("Who will lead me into the strong city? *"),
                    b: String::from("who will bring me into Edom?")
                },
              PsalmVerse {
                    number: 11,
                    a: String::from("Have you not cast us off, O God? *"),
                    b: String::from("you no longer go out, O God, with our armies.")
                },
              PsalmVerse {
                    number: 12,
                    a: String::from("Grant us your help against the enemy, *"),
                    b: String::from("for vain is the help of man.")
                },
              PsalmVerse {
                    number: 13,
                    a: String::from("With God we will do valiant deeds, *"),
                    b: String::from("and he shall tread our enemies under foot.")
                },
            ]
          }
        ]
    };
}
