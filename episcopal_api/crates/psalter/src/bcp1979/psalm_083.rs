use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_83: Psalm = Psalm {
        number: 83,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 706
            },
            local_name: String::from("Psalm 83"),
            latin_name: String::from("Deus, quis similis"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("O God, do not be silent; *"),
                    b: String::from("do not keep still nor hold your peace, O God;")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("For your enemies are in tumult, *"),
                    b: String::from("and those who hate you have lifted up their heads.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("They take secret counsel against your people *"),
                    b: String::from("and plot against those whom you protect.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from(
                        "They have said, “Come, let us wipe them out from among the nations; *"
                    ),
                    b: String::from("let the name of Israel be remembered no more.”")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("They have conspired together; *"),
                    b: String::from("they have made an alliance against you:")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("The tents of Edom and the Ishmaelites; *"),
                    b: String::from("the Moabites and the Hagarenes;")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("Gebal, and Ammon, and Amalek; *"),
                    b: String::from("the Philistines and those who dwell in Tyre.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("The Assyrians also have joined them, *"),
                    b: String::from("and have come to help the people of Lot.")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("Do to them as you did to Midian, *"),
                    b: String::from("to Sisera, and to Jabin at the river of Kishon:")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("They were destroyed at Endor; *"),
                    b: String::from("they became like dung upon the ground.")
                },
                PsalmVerse {
                    number: 11,
                    a: String::from("Make their leaders like Oreb and Zeëb, *"),
                    b: String::from("and all their commanders like Zebah and Zalmunna,")
                },
                PsalmVerse {
                    number: 12,
                    a: String::from("Who said, “Let us take for ourselves *"),
                    b: String::from("the fields of God as our possession.”")
                },
                PsalmVerse {
                    number: 13,
                    a: String::from("O my God, make them like whirling dust *"),
                    b: String::from("and like chaff before the wind;")
                },
                PsalmVerse {
                    number: 14,
                    a: String::from("Like fire that burns down a forest, *"),
                    b: String::from("like the flame that sets mountains ablaze.")
                },
                PsalmVerse {
                    number: 15,
                    a: String::from("Drive them with your tempest *"),
                    b: String::from("and terrify them with your storm;")
                },
                PsalmVerse {
                    number: 16,
                    a: String::from("Cover their faces with shame, O LORD, *"),
                    b: String::from("that they may seek your Name.")
                },
                PsalmVerse {
                    number: 17,
                    a: String::from("Let them be disgraced and terrified for ever; *"),
                    b: String::from("let them be put to confusion and perish.")
                },
                PsalmVerse {
                    number: 18,
                    a: String::from("Let them know that you, whose Name is YAHWEH, *"),
                    b: String::from("you alone are the Most High over all the earth.")
                },
            ]
        }]
    };
}
