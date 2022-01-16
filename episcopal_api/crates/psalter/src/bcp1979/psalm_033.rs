use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_33: Psalm = Psalm {
        number: 33,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 626
            },
            local_name: String::from("Psalm 33"),
            latin_name: String::from("Exultate, justi"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Rejoice in the LORD, you righteous; *"),
                    b: String::from("it is good for the just to sing praises.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Praise the LORD with the harp; *"),
                    b: String::from("play to him upon the psaltery and lyre.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("Sing for him a new song; *"),
                    b: String::from("sound a fanfare with all your skill upon the trumpet.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("For the word of the LORD is right, *"),
                    b: String::from("and all his works are sure.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("He loves righteousness and justice; *"),
                    b: String::from("the loving-kindness of the LORD fills the whole earth.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("By the word of the LORD were the heavens made, *"),
                    b: String::from("by the breath of his mouth all the heavenly hosts.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("He gathers up the waters of the ocean as in a water-skin *"),
                    b: String::from("and stores up the depths of the sea.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("Let all the earth fear the LORD; *"),
                    b: String::from("let all who dwell in the world stand in awe of him.")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("For he spoke, and it came to pass; *"),
                    b: String::from("he commanded, and it stood fast.")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("The LORD brings the will of the nations to naught; *"),
                    b: String::from("he thwarts the designs of the peoples.")
                },
                PsalmVerse {
                    number: 11,
                    a: String::from("But the LORD’S will stands fast for ever, *"),
                    b: String::from("and the designs of his heart from age to age.")
                },
                PsalmVerse {
                    number: 12,
                    a: String::from("Happy is the nation whose God is the LORD! *"),
                    b: String::from("happy the people he has chosen to be his own!")
                },
                PsalmVerse {
                    number: 13,
                    a: String::from("The LORD looks down from heaven, *"),
                    b: String::from("and beholds all the people in the world.")
                },
                PsalmVerse {
                    number: 14,
                    a: String::from("From where he sits enthroned he turns his gaze *"),
                    b: String::from("on all who dwell on the earth.")
                },
                PsalmVerse {
                    number: 15,
                    a: String::from("He fashions all the hearts of them *"),
                    b: String::from("and understands all their works.")
                },
                PsalmVerse {
                    number: 16,
                    a: String::from("There is no king that can be saved by a mighty army; *"),
                    b: String::from("a strong man is not delivered by his great strength.")
                },
                PsalmVerse {
                    number: 17,
                    a: String::from("The horse is a vain hope for deliverance; *"),
                    b: String::from("for all its strength it cannot save.")
                },
                PsalmVerse {
                    number: 18,
                    a: String::from("Behold, the eye of the LORD is upon those who fear him, *"),
                    b: String::from("on those who wait upon his love,")
                },
                PsalmVerse {
                    number: 19,
                    a: String::from("To pluck their lives from death, *"),
                    b: String::from("and to feed them in time of famine.")
                },
                PsalmVerse {
                    number: 20,
                    a: String::from("Our soul waits for the LORD; *"),
                    b: String::from("he is our help and our shield.")
                },
                PsalmVerse {
                    number: 21,
                    a: String::from("Indeed, our heart rejoices in him, *"),
                    b: String::from("for in his holy Name we put our trust.")
                },
                PsalmVerse {
                    number: 22,
                    a: String::from("Let your loving-kindness, O LORD, be upon us, *"),
                    b: String::from("as we have put our trust in you.")
                },
            ]
        }]
    };
}
