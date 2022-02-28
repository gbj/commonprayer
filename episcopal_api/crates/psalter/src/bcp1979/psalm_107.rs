use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_107: Psalm = Psalm {
        number: 107,
        citation: None,
        sections: vec![
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 746
                },
                local_name: String::from("Part I"),
                latin_name: String::from("Confitemini Domino"),
                verses: vec![
                    PsalmVerse {
                        number: 1,
                        a: String::from("Give thanks to the LORD, for he is good, *"),
                        b: String::from("and his mercy endures for ever.")
                    },
                    PsalmVerse {
                        number: 2,
                        a: String::from("Let all those whom the LORD has redeemed proclaim *"),
                        b: String::from("that he redeemed them from the hand of the foe.")
                    },
                    PsalmVerse {
                        number: 3,
                        a: String::from("He gathered them out of the lands; *"),
                        b: String::from(
                            "from the east and from the west,\nfrom the north and from the south."
                        )
                    },
                    PsalmVerse {
                        number: 4,
                        a: String::from("Some wandered in desert wastes; *"),
                        b: String::from("they found no way to a city where they might dwell.")
                    },
                    PsalmVerse {
                        number: 5,
                        a: String::from("They were hungry and thirsty; *"),
                        b: String::from("their spirits languished within them.")
                    },
                    PsalmVerse {
                        number: 6,
                        a: String::from("Then they cried to the LORD in their trouble, *"),
                        b: String::from("and he delivered them from their distress.")
                    },
                    PsalmVerse {
                        number: 7,
                        a: String::from("He put their feet on a straight path *"),
                        b: String::from("to go to a city where they might dwell.")
                    },
                    PsalmVerse {
                        number: 8,
                        a: String::from("Let them give thanks to the LORD for his mercy *"),
                        b: String::from("and the wonders he does for his children.")
                    },
                    PsalmVerse {
                        number: 9,
                        a: String::from("For he satisfies the thirsty *"),
                        b: String::from("and fills the hungry with good things.")
                    },
                    PsalmVerse {
                        number: 10,
                        a: String::from("Some sat in darkness and deep gloom, *"),
                        b: String::from("bound fast in misery and iron;")
                    },
                    PsalmVerse {
                        number: 11,
                        a: String::from("Because they rebelled against the words of God *"),
                        b: String::from("and despised the counsel of the Most High.")
                    },
                    PsalmVerse {
                        number: 12,
                        a: String::from("So he humbled their spirits with hard labor; *"),
                        b: String::from("they stumbled, and there was none to help.")
                    },
                    PsalmVerse {
                        number: 13,
                        a: String::from("Then they cried to the LORD in their trouble, *"),
                        b: String::from("and he delivered them from their distress.")
                    },
                    PsalmVerse {
                        number: 14,
                        a: String::from("He led them out of darkness and deep gloom *"),
                        b: String::from("and broke their bonds asunder.")
                    },
                    PsalmVerse {
                        number: 15,
                        a: String::from("Let them give thanks to the LORD for his mercy *"),
                        b: String::from("and the wonders he does for his children.")
                    },
                    PsalmVerse {
                        number: 16,
                        a: String::from("For he shatters the doors of bronze *"),
                        b: String::from("and breaks in two the iron bars.")
                    },
                    PsalmVerse {
                        number: 17,
                        a: String::from("Some were fools and took to rebellious ways; *"),
                        b: String::from("they were afflicted because of their sins.")
                    },
                    PsalmVerse {
                        number: 18,
                        a: String::from("They abhorred all manner of food *"),
                        b: String::from("and drew near to death’s door.")
                    },
                    PsalmVerse {
                        number: 19,
                        a: String::from("Then they cried to the LORD in their trouble, *"),
                        b: String::from("and he delivered them from their distress.")
                    },
                    PsalmVerse {
                        number: 20,
                        a: String::from("He sent forth his word and healed them *"),
                        b: String::from("and saved them from the grave.")
                    },
                    PsalmVerse {
                        number: 21,
                        a: String::from("Let them give thanks to the LORD for his mercy *"),
                        b: String::from("and the wonders he does for his children.")
                    },
                    PsalmVerse {
                        number: 22,
                        a: String::from("Let them offer a sacrifice of thanksgiving *"),
                        b: String::from("and tell of his acts with shouts of joy.")
                    },
                    PsalmVerse {
                        number: 23,
                        a: String::from("Some went down to the sea in ships *"),
                        b: String::from("and plied their trade in deep waters;")
                    },
                    PsalmVerse {
                        number: 24,
                        a: String::from("They beheld the works of the LORD *"),
                        b: String::from("and his wonders in the deep.")
                    },
                    PsalmVerse {
                        number: 25,
                        a: String::from("Then he spoke, and a stormy wind arose, *"),
                        b: String::from("which tossed high the waves of the sea.")
                    },
                    PsalmVerse {
                        number: 26,
                        a: String::from(
                            "They mounted up to the heavens and fell back to the depths; *"
                        ),
                        b: String::from("their hearts melted because of their peril.")
                    },
                    PsalmVerse {
                        number: 27,
                        a: String::from("They reeled and staggered like drunkards *"),
                        b: String::from("and were at their wits’ end.")
                    },
                    PsalmVerse {
                        number: 28,
                        a: String::from("Then they cried to the LORD in their trouble, *"),
                        b: String::from("and he delivered them from their distress.")
                    },
                    PsalmVerse {
                        number: 29,
                        a: String::from("He stilled the storm to a whisper *"),
                        b: String::from("and quieted the waves of the sea.")
                    },
                    PsalmVerse {
                        number: 30,
                        a: String::from("Then were they glad because of the calm, *"),
                        b: String::from("and he brought them to the harbor they were bound for.")
                    },
                    PsalmVerse {
                        number: 31,
                        a: String::from("Let them give thanks to the LORD for his mercy *"),
                        b: String::from("and the wonders he does for his children.")
                    },
                    PsalmVerse {
                        number: 32,
                        a: String::from("Let them exalt him in the congregation of the people *"),
                        b: String::from("and praise him in the council of the elders.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 746
                },
                local_name: String::from("Part II"),
                latin_name: String::from("Confitemini Domino"),
                verses: vec![
                    PsalmVerse {
                        number: 33,
                        a: String::from("The LORD changed rivers into deserts, *"),
                        b: String::from("and water-springs into thirsty ground,")
                    },
                    PsalmVerse {
                        number: 34,
                        a: String::from("A fruitful land into salt flats, *"),
                        b: String::from("because of the wickedness of those who dwell there.")
                    },
                    PsalmVerse {
                        number: 35,
                        a: String::from("He changed deserts into pools of water *"),
                        b: String::from("and dry land into water-springs.")
                    },
                    PsalmVerse {
                        number: 36,
                        a: String::from("He settled the hungry there, *"),
                        b: String::from("and they founded a city to dwell in.")
                    },
                    PsalmVerse {
                        number: 37,
                        a: String::from("They sowed fields, and planted vineyards, *"),
                        b: String::from("and brought in a fruitful harvest.")
                    },
                    PsalmVerse {
                        number: 38,
                        a: String::from("He blessed them, so that they increased greatly; *"),
                        b: String::from("he did not let their herds decrease.")
                    },
                    PsalmVerse {
                        number: 39,
                        a: String::from("Yet when they were diminished and brought low, *"),
                        b: String::from("through stress of adversity and sorrow,")
                    },
                    PsalmVerse {
                        number: 40,
                        a: String::from("(He pours contempt on princes *"),
                        b: String::from("and makes them wander in trackless wastes)")
                    },
                    PsalmVerse {
                        number: 41,
                        a: String::from("He lifted up the poor out of misery *"),
                        b: String::from("and multiplied their families like flocks of sheep.")
                    },
                    PsalmVerse {
                        number: 42,
                        a: String::from("The upright will see this and rejoice, *"),
                        b: String::from("but all wickedness will shut its mouth.")
                    },
                    PsalmVerse {
                        number: 43,
                        a: String::from("Whoever is wise will ponder these things, *"),
                        b: String::from("and consider well the mercies of the LORD.")
                    },
                ]
            }
        ]
    };
}
