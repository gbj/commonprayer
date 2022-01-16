use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};

lazy_static! {
    pub static ref PSALM_89: Psalm = Psalm {
        number: 89,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 713
            },
            local_name: String::from("Psalm 89: Part I"),
            latin_name: String::from("Misericordias Domini"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Your love, O LORD, for ever will I sing; *"),
                    b: String::from("from age to age my mouth will proclaim your faithfulness.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("For I am persuaded that your love is established for ever; *"),
                    b: String::from("you have set your faithfulness firmly in the heavens.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("“I have made a covenant with my chosen one; *"),
                    b: String::from("I have sworn an oath to David my servant:")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("‘I will establish your line for ever, *"),
                    b: String::from("and preserve your throne for all generations.’”")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("The heavens bear witness to your wonders, O LORD, *"),
                    b: String::from("and to your faithfulness in the assembly of the holy ones;")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("For who in the skies can be compared to the LORD? *"),
                    b: String::from("who is like the LORD among the gods?")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("God is much to be feared in the council of the holy ones, *"),
                    b: String::from("great and terrible to all those round about him.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("Who is like you, LORD God of hosts? *"),
                    b: String::from("O mighty LORD, your faithfulness is all around you.")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("You rule the raging of the sea *"),
                    b: String::from("and still the surging of its waves.")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("You have crushed Rahab of the deep with a deadly wound; *"),
                    b: String::from("you have scattered your enemies with your mighty arm.")
                },
                PsalmVerse {
                    number: 11,
                    a: String::from("Yours are the heavens; the earth also is yours; *"),
                    b: String::from("you laid the foundations of the world and all that is in it.")
                },
                PsalmVerse {
                    number: 12,
                    a: String::from("You have made the north and the south; *"),
                    b: String::from("Tabor and Hermon rejoice in your Name.")
                },
                PsalmVerse {
                    number: 13,
                    a: String::from("You have a mighty arm; *"),
                    b: String::from("strong is your hand and high is your right hand.")
                },
                PsalmVerse {
                    number: 14,
                    a: String::from(
                        "Righteousness and justice are the foundations of your throne; *"
                    ),
                    b: String::from("love and truth go before your face.")
                },
                PsalmVerse {
                    number: 15,
                    a: String::from("Happy are the people who know the festal shout! *"),
                    b: String::from("they walk, O LORD, in the light of your presence.")
                },
                PsalmVerse {
                    number: 16,
                    a: String::from("They rejoice daily in your Name; *"),
                    b: String::from("they are jubilant in your righteousness.")
                },
                PsalmVerse {
                    number: 17,
                    a: String::from("For you are the glory of their strength, *"),
                    b: String::from("and by your favor our might is exalted.")
                },
                PsalmVerse {
                    number: 18,
                    a: String::from("Truly, the LORD is our ruler; *"),
                    b: String::from("The Holy One of Israel is our King.")
                },
            ]
        }, PsalmSection {
          reference: Reference {
            source: Source::BCP1979,
            page: 715
          },
          local_name: String::from("Psalm 89: Part II"),
          latin_name: String::from("Tunc locutus es"),
          verses: vec![
                        PsalmVerse {
                  number: 19,
                  a: String::from("You spoke once in vision and said to your faithful people: *"),
                  b: String::from("“I have set the crown upon a warrior\n and have exalted one chosen out of the people.")
              },
            PsalmVerse {
                  number: 20,
                  a: String::from("I have found David my servant; *"),
                  b: String::from("with my holy oil have I anointed him.")
              },
            PsalmVerse {
                  number: 21,
                  a: String::from("My hand will hold him fast *"),
                  b: String::from("and my arm will make him strong.")
              },
            PsalmVerse {
                  number: 22,
                  a: String::from("No enemy shall deceive him, *"),
                  b: String::from("nor any wicked man bring him down.")
              },
            PsalmVerse {
                  number: 23,
                  a: String::from("I will crush his foes before him *"),
                  b: String::from("and strike down those who hate him.")
              },
            PsalmVerse {
                  number: 24,
                  a: String::from("My faithfulness and love shall be with him, *"),
                  b: String::from("and he shall be victorious through my Name.")
              },
            PsalmVerse {
                  number: 25,
                  a: String::from("I shall make his dominion extend *"),
                  b: String::from("from the Great Sea to the River.")
              },
            PsalmVerse {
                  number: 26,
                  a: String::from("He will say to me, ‘You are my Father, *"),
                  b: String::from("my God, and the rock of my salvation.’")
              },
            PsalmVerse {
                  number: 27,
                  a: String::from("I will make him my firstborn *"),
                  b: String::from("and higher than the kings of the earth.")
              },
            PsalmVerse {
                  number: 28,
                  a: String::from("I will keep my love for him for ever, *"),
                  b: String::from("and my covenant will stand firm for him.")
              },
            PsalmVerse {
                  number: 29,
                  a: String::from("I will establish his line for ever *"),
                  b: String::from("and his throne as the days of heaven.”")
              },
            PsalmVerse {
                  number: 30,
                  a: String::from("“If his children forsake my law *"),
                  b: String::from("and do not walk according to my judgments;")
              },
            PsalmVerse {
                  number: 31,
                  a: String::from("If they break my statutes *"),
                  b: String::from("and do not keep my commandments;")
              },
            PsalmVerse {
                  number: 32,
                  a: String::from("I will punish their transgressions with a rod *"),
                  b: String::from("and their iniquities with the lash;")
              },
            PsalmVerse {
                  number: 33,
                  a: String::from("But I will not take my love from him, *"),
                  b: String::from("nor let my faithfulness prove false.")
              },
            PsalmVerse {
                  number: 34,
                  a: String::from("I will not break my covenant, *"),
                  b: String::from("nor change what has gone out of my lips.")
              },
            PsalmVerse {
                  number: 35,
                  a: String::from("Once for all I have sworn by my holiness: *"),
                  b: String::from("‘I will not lie to David.")
              },
            PsalmVerse {
                  number: 36,
                  a: String::from("His line shall endure for ever *"),
                  b: String::from("and his throne as the sun before me;")
              },
            PsalmVerse {
                  number: 37,
                  a: String::from("It shall stand fast for evermore like the moon, *"),
                  b: String::from("the abiding witness in the sky.’”")
              },
            PsalmVerse {
                  number: 38,
                  a: String::from("But you have cast off and rejected your anointed; *"),
                  b: String::from("you have become enraged at him.")
              },
            PsalmVerse {
                  number: 39,
                  a: String::from("You have broken your covenant with your servant, *"),
                  b: String::from("defiled his crown, and hurled it to the ground.")
              },
            PsalmVerse {
                  number: 40,
                  a: String::from("You have breached all his walls *"),
                  b: String::from("and laid his strongholds in ruins.")
              },
            PsalmVerse {
                  number: 41,
                  a: String::from("All who pass by despoil him; *"),
                  b: String::from("he has become the scorn of his neighbors.")
              },
            PsalmVerse {
                  number: 42,
                  a: String::from("You have exalted the right hand of his foes *"),
                  b: String::from("and made all his enemies rejoice.")
              },
            PsalmVerse {
                  number: 43,
                  a: String::from("You have turned back the edge of his sword *"),
                  b: String::from("and have not sustained him in battle.")
              },
            PsalmVerse {
                  number: 44,
                  a: String::from("You have put an end to his splendor *"),
                  b: String::from("and cast his throne to the ground.")
              },
            PsalmVerse {
                  number: 45,
                  a: String::from("You have cut short the days of his youth *"),
                  b: String::from("and have covered him with shame.")
              },
            PsalmVerse {
                  number: 46,
                  a: String::from("How long will you hide yourself, O LORD?\nwill you hide yourself for ever? *"),
                  b: String::from("how long will your anger burn like fire?")
              },
            PsalmVerse {
                  number: 47,
                  a: String::from("Remember, LORD, how short life is, *"),
                  b: String::from("how frail you have made all flesh.")
              },
            PsalmVerse {
                  number: 48,
                  a: String::from("Who can live and not see death? *"),
                  b: String::from("who can save himself from the power of the grave?")
              },
            PsalmVerse {
                  number: 49,
                  a: String::from("Where, Lord, are your loving-kindnesses of old, *"),
                  b: String::from("which you promised David in your faithfulness?")
              },
            PsalmVerse {
                  number: 50,
                  a: String::from("Remember, Lord, how your servant is mocked, *"),
                  b: String::from("how I carry in my bosom the taunts of many peoples,")
              },
            PsalmVerse {
                  number: 51,
                  a: String::from("The taunts your enemies have hurled, O LORD, *"),
                  b: String::from("which they hurled at the heels of your anointed.")
              },
            PsalmVerse {
                  number: 52,
                  a: String::from("Blessed be the LORD for evermore! *"),
                  b: String::from("Amen, I say, Amen.")
              },
          ]
        }

    ]
    };
}
