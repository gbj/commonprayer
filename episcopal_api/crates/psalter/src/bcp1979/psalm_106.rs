use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_106: Psalm = Psalm {
        number: 106,
        citation: None,
        sections: vec![
          PsalmSection {
            reference: Reference {
              source: Source::BCP1979,
              page: 741
            },
            local_name: String::from("Part I"),
            latin_name: String::from("Confitemini Domino"),
            verses: vec![
                          PsalmVerse {
                    number: 1,
                    a: String::from("Hallelujah!\nGive thanks to the LORD, for he is good, *"),
                    b: String::from("for his mercy endures for ever.")
                },
              PsalmVerse {
                    number: 2,
                    a: String::from("Who can declare the mighty acts of the LORD *"),
                    b: String::from("or show forth all his praise?")
                },
              PsalmVerse {
                    number: 3,
                    a: String::from("Happy are those who act with justice *"),
                    b: String::from("and always do what is right!")
                },
              PsalmVerse {
                    number: 4,
                    a: String::from("Remember me, O LORD, with the favor you have for your people, *"),
                    b: String::from("and visit me with your saving help;")
                },
              PsalmVerse {
                    number: 5,
                    a: String::from("That I may see the prosperity of your elect\nand be glad with the gladness of your people, *"),
                    b: String::from("that I may glory with your inheritance.")
                },
              PsalmVerse {
                    number: 6,
                    a: String::from("We have sinned as our forebears did; *"),
                    b: String::from("we have done wrong and dealt wickedly.")
                },
              PsalmVerse {
                    number: 7,
                    a: String::from("In Egypt they did not consider your marvelous works,\nnor remember the abundance of your love; *"),
                    b: String::from("they defied the Most High at the Red Sea.")
                },
              PsalmVerse {
                    number: 8,
                    a: String::from("But he saved them for his Name’s sake, *"),
                    b: String::from("to make his power known.")
                },
              PsalmVerse {
                    number: 9,
                    a: String::from("He rebuked the Red Sea, and it dried up, *"),
                    b: String::from("and he led them through the deep as through a desert.")
                },
              PsalmVerse {
                    number: 10,
                    a: String::from("He saved them from the hand of those who hated them *"),
                    b: String::from("and redeemed them from the hand of the enemy.")
                },
              PsalmVerse {
                    number: 11,
                    a: String::from("The waters covered their oppressors; *"),
                    b: String::from("not one of them was left.")
                },
              PsalmVerse {
                    number: 12,
                    a: String::from("Then they believed his words *"),
                    b: String::from("and sang him songs of praise.")
                },
              PsalmVerse {
                    number: 13,
                    a: String::from("But they soon forgot his deeds *"),
                    b: String::from("and did not wait for his counsel.")
                },
              PsalmVerse {
                    number: 14,
                    a: String::from("A craving seized them in the wilderness, *"),
                    b: String::from("and they put God to the test in the desert.")
                },
              PsalmVerse {
                    number: 15,
                    a: String::from("He gave them what they asked, *"),
                    b: String::from("but sent leanness into their soul.")
                },
              PsalmVerse {
                    number: 16,
                    a: String::from("They envied Moses in the camp, *"),
                    b: String::from("and Aaron, the holy one of the LORD.")
                },
              PsalmVerse {
                    number: 17,
                    a: String::from("The earth opened and swallowed Dathan *"),
                    b: String::from("and covered the company of Abiram.")
                },
              PsalmVerse {
                    number: 18,
                    a: String::from("Fire blazed up against their company, *"),
                    b: String::from("and flames devoured the wicked.")
                },
            ]
          },
      PsalmSection {
        reference: Reference {
          source: Source::BCP1979,
          page: 743
        },
        local_name: String::from("Part II"),
        latin_name: String::from("Et fecerunt vitulum"),
        verses: vec![
                      PsalmVerse {
                number: 19,
                a: String::from("Israel made a bull-calf at Horeb *"),
                b: String::from("and worshiped a molten image;")
            },
          PsalmVerse {
                number: 20,
                a: String::from("And so they exchanged their Glory *"),
                b: String::from("for the image of an ox that feeds on grass.")
            },
          PsalmVerse {
                number: 21,
                a: String::from("They forgot God their Savior, *"),
                b: String::from("who had done great things in Egypt,")
            },
          PsalmVerse {
                number: 22,
                a: String::from("Wonderful deeds in the land of Ham, *"),
                b: String::from("and fearful things at the Red Sea.")
            },
          PsalmVerse {
                number: 23,
                a: String::from("So he would have destroyed them,\nhad not Moses his chosen stood before him in the breach, *"),
                b: String::from("to turn away his wrath from consuming them.")
            },
          PsalmVerse {
                number: 24,
                a: String::from("They refused the pleasant land *"),
                b: String::from("and would not believe his promise.")
            },
          PsalmVerse {
                number: 25,
                a: String::from("They grumbled in their tents *"),
                b: String::from("and would not listen to the voice of the LORD.")
            },
          PsalmVerse {
                number: 26,
                a: String::from("So he lifted his hand against them, *"),
                b: String::from("to overthrow them in the wilderness,")
            },
          PsalmVerse {
                number: 27,
                a: String::from("To cast out their seed among the nations, *"),
                b: String::from("and to scatter them throughout the lands.")
            },
          PsalmVerse {
                number: 28,
                a: String::from("They joined themselves to Baal-Peor *"),
                b: String::from("and ate sacrifices offered to the dead.")
            },
          PsalmVerse {
                number: 29,
                a: String::from("They provoked him to anger with their actions, *"),
                b: String::from("and a plague broke out among them.")
            },
          PsalmVerse {
                number: 30,
                a: String::from("Then Phinehas stood up and interceded, *"),
                b: String::from("and the plague came to an end.")
            },
          PsalmVerse {
                number: 31,
                a: String::from("This was reckoned to him as righteousness *"),
                b: String::from("throughout all generations for ever.")
            },
          PsalmVerse {
                number: 32,
                a: String::from("Again they provoked his anger at the waters of Meribah, *"),
                b: String::from("so that he punished Moses because of them;")
            },
          PsalmVerse {
                number: 33,
                a: String::from("For they so embittered his spirit *"),
                b: String::from("that he spoke rash words with his lips.")
            },
          PsalmVerse {
                number: 34,
                a: String::from("They did not destroy the peoples *"),
                b: String::from("as the LORD had commanded them.")
            },
          PsalmVerse {
                number: 35,
                a: String::from("They intermingled with the heathen *"),
                b: String::from("and learned their pagan ways,")
            },
          PsalmVerse {
                number: 36,
                a: String::from("So that they worshiped their idols, *"),
                b: String::from("which became a snare to them.")
            },
          PsalmVerse {
                number: 37,
                a: String::from("They sacrificed their sons *"),
                b: String::from("and their daughters to evil spirits.")
            },
          PsalmVerse {
                number: 38,
                a: String::from("They shed innocent blood,\nthe blood of their sons and daughters, *"),
                b: String::from("which they offered to the idols of Canaan,\nand the land was defiled with blood.")
            },
          PsalmVerse {
                number: 39,
                a: String::from("Thus they were polluted by their actions *"),
                b: String::from("and went whoring in their evil deeds.")
            },
          PsalmVerse {
                number: 40,
                a: String::from("Therefore the wrath of the LORD was kindled against his people *"),
                b: String::from("and he abhorred his inheritance.")
            },
          PsalmVerse {
                number: 41,
                a: String::from("He gave them over to the hand of the heathen, *"),
                b: String::from("and those who hated them ruled over them.")
            },
          PsalmVerse {
                number: 42,
                a: String::from("Their enemies oppressed them, *"),
                b: String::from("and they were humbled under their hand.")
            },
          PsalmVerse {
                number: 43,
                a: String::from("Many a time did he deliver them,\nbut they rebelled through their own devices, *"),
                b: String::from("and were brought down in their iniquity.")
            },
          PsalmVerse {
                number: 44,
                a: String::from("Nevertheless, he saw their distress, *"),
                b: String::from("when he heard their lamentation.")
            },
          PsalmVerse {
                number: 45,
                a: String::from("He remembered his covenant with them *"),
                b: String::from("and relented in accordance with his great mercy.")
            },
          PsalmVerse {
                number: 46,
                a: String::from("He caused them to be pitied *"),
                b: String::from("by those who held them captive.")
            },
          PsalmVerse {
                number: 47,
                a: String::from("Save us, O LORD our God,\nand gather us from among the nations, *"),
                b: String::from("that we may give thanks to your holy Name\nand glory in your praise.")
            },
          PsalmVerse {
                number: 48,
                a: String::from("Blessed be the LORD, the God of Israel,\nfrom everlasting to everlasting; *"),
                b: String::from("and let all the people say, “Amen!”\nHallelujah!")
            },
        ]
      }
        ]
    };
}
