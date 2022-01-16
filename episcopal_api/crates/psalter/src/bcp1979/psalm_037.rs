use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_37: Psalm = Psalm {
        number: 37,
        citation: None,
        sections: vec![
          PsalmSection {
            reference: Reference {
              source: Source::BCP1979,
              page: 633
            },
            local_name: String::from("Psalm 37: Part I"),
            latin_name: String::from("Noli aemulari"),
            verses: vec![
                          PsalmVerse {
                    number: 1,
                    a: String::from("Do not fret yourself because of evildoers; *"),
                    b: String::from("do not be jealous of those who do wrong.")
                },
              PsalmVerse {
                    number: 2,
                    a: String::from("For they shall soon wither like the grass, *"),
                    b: String::from("and like the green grass fade away.")
                },
              PsalmVerse {
                    number: 3,
                    a: String::from("Put your trust in the LORD and do good; *"),
                    b: String::from("dwell in the land and feed on its riches.")
                },
              PsalmVerse {
                    number: 4,
                    a: String::from("Take delight in the LORD, *"),
                    b: String::from("and he shall give you your heart’s desire.")
                },
              PsalmVerse {
                    number: 5,
                    a: String::from("Commit your way to the LORD and put your trust in him, *"),
                    b: String::from("and he will bring it to pass.")
                },
              PsalmVerse {
                    number: 6,
                    a: String::from("He will make your righteousness as clear as the light *"),
                    b: String::from("and your just dealing as the noonday.")
                },
              PsalmVerse {
                    number: 7,
                    a: String::from("Be still before the LORD *"),
                    b: String::from("and wait patiently for him.")
                },
              PsalmVerse {
                    number: 8,
                    a: String::from("Do not fret yourself over the one who prospers, *"),
                    b: String::from("the one who succeeds in evil schemes.")
                },
              PsalmVerse {
                    number: 9,
                    a: String::from("Refrain from anger, leave rage alone; *"),
                    b: String::from("do not fret yourself; it leads only to evil.")
                },
              PsalmVerse {
                    number: 10,
                    a: String::from("For evildoers shall be cut off, *"),
                    b: String::from("but those who wait upon the LORD shall possess the land.")
                },
              PsalmVerse {
                    number: 11,
                    a: String::from("In a little while the wicked shall be no more; *"),
                    b: String::from("you shall search out their place, but they will not be there.")
                },
              PsalmVerse {
                    number: 12,
                    a: String::from("But the lowly shall possess the land; *"),
                    b: String::from("they will delight in abundance of peace.")
                },
              PsalmVerse {
                    number: 13,
                    a: String::from("The wicked plot against the righteous *"),
                    b: String::from("and gnash at them with their teeth.")
                },
              PsalmVerse {
                    number: 14,
                    a: String::from("The Lord laughs at the wicked, *"),
                    b: String::from("because he sees that their day will come.")
                },
              PsalmVerse {
                    number: 15,
                    a: String::from("The wicked draw their sword and bend their bow\nto strike down the poor and needy, *"),
                    b: String::from("to slaughter those who are upright in their ways.")
                },
              PsalmVerse {
                    number: 16,
                    a: String::from("Their sword shall go through their own heart, *"),
                    b: String::from("and their bow shall be broken.")
                },
              PsalmVerse {
                    number: 17,
                    a: String::from("The little that the righteous has *"),
                    b: String::from("is better than great riches of the wicked.")
                },
              PsalmVerse {
                    number: 18,
                    a: String::from("For the power of the wicked shall be broken, *"),
                    b: String::from("but the LORD upholds the righteous.")
                },
            ]
          },
          PsalmSection {
            reference: Reference {
              source: Source::BCP1979,
              page: 634
            },
            local_name: String::from("Psalm 37: Part II"),
            latin_name: String::from("Novit Dominus"),
            verses: vec![
              PsalmVerse {
                    number: 19,
                    a: String::from("The LORD cares for the lives of the godly, *"),
                    b: String::from("and their inheritance shall last for ever.")
                },
              PsalmVerse {
                    number: 20,
                    a: String::from("They shall not be ashamed in bad times, *"),
                    b: String::from("and in days of famine they shall have enough.")
                },
              PsalmVerse {
                    number: 21,
                    a: String::from("As for the wicked, they shall perish, *"),
                    b: String::from("and the enemies of the LORD, like the glory of the meadows, shall vanish;\n they shall vanish like smoke.")
                },
              PsalmVerse {
                    number: 22,
                    a: String::from("The wicked borrow and do not repay, *"),
                    b: String::from("but the righteous are generous in giving.")
                },
              PsalmVerse {
                    number: 23,
                    a: String::from("Those who are blessed by God shall possess the land, *"),
                    b: String::from("but those who are cursed by him shall be destroyed.")
                },
              PsalmVerse {
                    number: 24,
                    a: String::from("Our steps are directed by the LORD; *"),
                    b: String::from("he strengthens those in whose way he delights.")
                },
              PsalmVerse {
                    number: 25,
                    a: String::from("If they stumble, they shall not fall headlong, *"),
                    b: String::from("for the LORD holds them by the hand.")
                },
              PsalmVerse {
                    number: 26,
                    a: String::from("I have been young and now I am old, *"),
                    b: String::from("but never have I seen the righteous forsaken,\n or their children begging bread.")
                },
              PsalmVerse {
                    number: 27,
                    a: String::from("The righteous are always generous in their lending, *"),
                    b: String::from("and their children shall be a blessing.")
                },
              PsalmVerse {
                    number: 28,
                    a: String::from("Turn from evil, and do good, *"),
                    b: String::from("and dwell in the land for ever.")
                },
              PsalmVerse {
                    number: 29,
                    a: String::from("For the LORD loves justice; *"),
                    b: String::from("he does not forsake his faithful ones.")
                },
              PsalmVerse {
                    number: 30,
                    a: String::from("They shall be kept safe for ever, *"),
                    b: String::from("but the offspring of the wicked shall be destroyed.")
                },
              PsalmVerse {
                    number: 31,
                    a: String::from("The righteous shall possess the land *"),
                    b: String::from("and dwell in it for ever.")
                },
              PsalmVerse {
                    number: 32,
                    a: String::from("The mouth of the righteous utters wisdom, *"),
                    b: String::from("and their tongue speaks what is right.")
                },
              PsalmVerse {
                    number: 33,
                    a: String::from("The law of their God is in their heart, *"),
                    b: String::from("and their footsteps shall not falter.")
                },
              PsalmVerse {
                    number: 34,
                    a: String::from("The wicked spy on the righteous *"),
                    b: String::from("and seek occasion to kill them.")
                },
              PsalmVerse {
                    number: 35,
                    a: String::from("The LORD will not abandon them to their hand, *"),
                    b: String::from("nor let them be found guilty when brought to trial.")
                },
              PsalmVerse {
                    number: 36,
                    a: String::from("Wait upon the LORD and keep his way; *"),
                    b: String::from("he will raise you up to possess the land,\n and when the wicked are cut off, you will see it.")
                },
              PsalmVerse {
                    number: 37,
                    a: String::from("I have seen the wicked in their arrogance, *"),
                    b: String::from("flourishing like a tree in full leaf.")
                },
              PsalmVerse {
                    number: 38,
                    a: String::from("I went by, and behold, they were not there; *"),
                    b: String::from("I searched for them, but they could not be found.")
                },
              PsalmVerse {
                    number: 39,
                    a: String::from("Mark those who are honest;\nobserve the upright; *"),
                    b: String::from("for there is a future for the peaceable.")
                },
              PsalmVerse {
                    number: 40,
                    a: String::from("Transgressors shall be destroyed, one and all; *"),
                    b: String::from("the future of the wicked is cut off.")
                },
              PsalmVerse {
                    number: 41,
                    a: String::from("But the deliverance of the righteous comes from the LORD; *"),
                    b: String::from("he is their stronghold in time of trouble.")
                },
              PsalmVerse {
                    number: 42,
                    a: String::from("The LORD will help them and rescue them; *"),
                    b: String::from("he will rescue them from the wicked and deliver them,\n because they seek refuge in him.")
                },
            ]
          }
        ]
    };
}
