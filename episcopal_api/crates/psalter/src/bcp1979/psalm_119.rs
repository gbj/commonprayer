use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_119: Psalm = Psalm {
        number: 119,
        citation: None,
        sections: vec![
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 763
                },
                local_name: String::from("Aleph"),
                latin_name: String::from("Beati immaculati"),
                verses: vec![
                    PsalmVerse {
                        number: 1,
                        a: String::from("Happy are they whose way is blameless, *"),
                        b: String::from("who walk in the law of the LORD!")
                    },
                    PsalmVerse {
                        number: 2,
                        a: String::from("Happy are they who observe his decrees *"),
                        b: String::from("and seek him with all their hearts!")
                    },
                    PsalmVerse {
                        number: 3,
                        a: String::from("Who never do any wrong, *"),
                        b: String::from("but always walk in his ways.")
                    },
                    PsalmVerse {
                        number: 4,
                        a: String::from("You laid down your commandments, *"),
                        b: String::from("that we should fully keep them.")
                    },
                    PsalmVerse {
                        number: 5,
                        a: String::from("Oh, that my ways were made so direct *"),
                        b: String::from("that I might keep your statutes!")
                    },
                    PsalmVerse {
                        number: 6,
                        a: String::from("Then I should not be put to shame, *"),
                        b: String::from("when I regard all your commandments.")
                    },
                    PsalmVerse {
                        number: 7,
                        a: String::from("I will thank you with an unfeigned heart, *"),
                        b: String::from("when I have learned your righteous judgments.")
                    },
                    PsalmVerse {
                        number: 8,
                        a: String::from("I will keep your statutes; *"),
                        b: String::from("do not utterly forsake me.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 764
                },
                local_name: String::from("Beth"),
                latin_name: String::from("In quo corrigit?"),
                verses: vec![
                    PsalmVerse {
                        number: 9,
                        a: String::from("How shall a young man cleanse his way? *"),
                        b: String::from("By keeping to your words.")
                    },
                    PsalmVerse {
                        number: 10,
                        a: String::from("With my whole heart I seek you; *"),
                        b: String::from("let me not stray from your commandments.")
                    },
                    PsalmVerse {
                        number: 11,
                        a: String::from("I treasure your promise in my heart, *"),
                        b: String::from("that I may not sin against you.")
                    },
                    PsalmVerse {
                        number: 12,
                        a: String::from("Blessed are you, O LORD; *"),
                        b: String::from("instruct me in your statutes.")
                    },
                    PsalmVerse {
                        number: 13,
                        a: String::from("With my lips will I recite *"),
                        b: String::from("all the judgments of your mouth.")
                    },
                    PsalmVerse {
                        number: 14,
                        a: String::from(
                            "I have taken greater delight in the way of your decrees *"
                        ),
                        b: String::from("than in all manner of riches.")
                    },
                    PsalmVerse {
                        number: 15,
                        a: String::from("I will meditate on your commandments *"),
                        b: String::from("and give attention to your ways.")
                    },
                    PsalmVerse {
                        number: 16,
                        a: String::from("My delight is in your statutes; *"),
                        b: String::from("I will not forget your word.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 764
                },
                local_name: String::from("Gimel"),
                latin_name: String::from("Retribue servo tuo"),
                verses: vec![
                    PsalmVerse {
                        number: 17,
                        a: String::from("Deal bountifully with your servant, *"),
                        b: String::from("that I may live and keep your word.")
                    },
                    PsalmVerse {
                        number: 18,
                        a: String::from("Open my eyes, that I may see *"),
                        b: String::from("the wonders of your law.")
                    },
                    PsalmVerse {
                        number: 19,
                        a: String::from("I am a stranger here on earth; *"),
                        b: String::from("do not hide your commandments from me.")
                    },
                    PsalmVerse {
                        number: 20,
                        a: String::from("My soul is consumed at all times *"),
                        b: String::from("with longing for your judgments.")
                    },
                    PsalmVerse {
                        number: 21,
                        a: String::from("You have rebuked the insolent; *"),
                        b: String::from("cursed are they who stray from your commandments!")
                    },
                    PsalmVerse {
                        number: 22,
                        a: String::from("Turn from me shame and rebuke, *"),
                        b: String::from("for I have kept your decrees.")
                    },
                    PsalmVerse {
                        number: 23,
                        a: String::from("Even though rulers sit and plot against me, *"),
                        b: String::from("I will meditate on your statutes.")
                    },
                    PsalmVerse {
                        number: 24,
                        a: String::from("For your decrees are my delight, *"),
                        b: String::from("and they are my counselors.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 765
                },
                local_name: String::from("Daleth"),
                latin_name: String::from("Adhaesit pavimento"),
                verses: vec![
                    PsalmVerse {
                        number: 25,
                        a: String::from("My soul cleaves to the dust; *"),
                        b: String::from("give me life according to your word.")
                    },
                    PsalmVerse {
                        number: 26,
                        a: String::from("I have confessed my ways, and you answered me; *"),
                        b: String::from("instruct me in your statutes.")
                    },
                    PsalmVerse {
                        number: 27,
                        a: String::from("Make me understand the way of your commandments, *"),
                        b: String::from("that I may meditate on your marvelous works.")
                    },
                    PsalmVerse {
                        number: 28,
                        a: String::from("My soul melts away for sorrow; *"),
                        b: String::from("strengthen me according to your word.")
                    },
                    PsalmVerse {
                        number: 29,
                        a: String::from("Take from me the way of lying; *"),
                        b: String::from("let me find grace through your law.")
                    },
                    PsalmVerse {
                        number: 30,
                        a: String::from("I have chosen the way of faithfulness; *"),
                        b: String::from("I have set your judgments before me.")
                    },
                    PsalmVerse {
                        number: 31,
                        a: String::from("I hold fast to your decrees; *"),
                        b: String::from("O LORD, let me not be put to shame.")
                    },
                    PsalmVerse {
                        number: 32,
                        a: String::from("I will run the way of your commandments, *"),
                        b: String::from("for you have set my heart at liberty.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 766
                },
                local_name: String::from("He"),
                latin_name: String::from("Legem pone"),
                verses: vec![
                    PsalmVerse {
                        number: 33,
                        a: String::from("Teach me, O LORD, the way of your statutes, *"),
                        b: String::from("and I shall keep it to the end.")
                    },
                    PsalmVerse {
                        number: 34,
                        a: String::from("Give me understanding, and I shall keep your law; *"),
                        b: String::from("I shall keep it with all my heart.")
                    },
                    PsalmVerse {
                        number: 35,
                        a: String::from("Make me go in the path of your commandments, *"),
                        b: String::from("for that is my desire.")
                    },
                    PsalmVerse {
                        number: 36,
                        a: String::from("Incline my heart to your decrees *"),
                        b: String::from("and not to unjust gain.")
                    },
                    PsalmVerse {
                        number: 37,
                        a: String::from("Turn my eyes from watching what is worthless; *"),
                        b: String::from("give me life in your ways.")
                    },
                    PsalmVerse {
                        number: 38,
                        a: String::from("Fulfill your promise to your servant, *"),
                        b: String::from("which you make to those who fear you.")
                    },
                    PsalmVerse {
                        number: 39,
                        a: String::from("Turn away the reproach which I dread, *"),
                        b: String::from("because your judgments are good.")
                    },
                    PsalmVerse {
                        number: 40,
                        a: String::from("Behold, I long for your commandments; *"),
                        b: String::from("in your righteousness preserve my life.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 766
                },
                local_name: String::from("Waw"),
                latin_name: String::from("Et veniat super me"),
                verses: vec![
                    PsalmVerse {
                        number: 41,
                        a: String::from("Let your loving-kindness come to me, O LORD, *"),
                        b: String::from("and your salvation, according to your promise.")
                    },
                    PsalmVerse {
                        number: 42,
                        a: String::from("Then shall I have a word for those who taunt me, *"),
                        b: String::from("because I trust in your words.")
                    },
                    PsalmVerse {
                        number: 43,
                        a: String::from("Do not take the word of truth out of my mouth, *"),
                        b: String::from("for my hope is in your judgments.")
                    },
                    PsalmVerse {
                        number: 44,
                        a: String::from("I shall continue to keep your law; *"),
                        b: String::from("I shall keep it for ever and ever.")
                    },
                    PsalmVerse {
                        number: 45,
                        a: String::from("I will walk at liberty, *"),
                        b: String::from("because I study your commandments.")
                    },
                    PsalmVerse {
                        number: 46,
                        a: String::from("I will tell of your decrees before kings *"),
                        b: String::from("and will not be ashamed.")
                    },
                    PsalmVerse {
                        number: 47,
                        a: String::from("I delight in your commandments, *"),
                        b: String::from("which I have always loved.")
                    },
                    PsalmVerse {
                        number: 48,
                        a: String::from("I will lift up my hands to your commandments, *"),
                        b: String::from("and I will meditate on your statutes.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 767
                },
                local_name: String::from("Zayin"),
                latin_name: String::from("Memor esto verbi tui"),
                verses: vec![
                    PsalmVerse {
                        number: 49,
                        a: String::from("Remember your word to your servant, *"),
                        b: String::from("because you have given me hope.")
                    },
                    PsalmVerse {
                        number: 50,
                        a: String::from("This is my comfort in my trouble, *"),
                        b: String::from("that your promise gives me life.")
                    },
                    PsalmVerse {
                        number: 51,
                        a: String::from("The proud have derided me cruelly, *"),
                        b: String::from("but I have not turned from your law.")
                    },
                    PsalmVerse {
                        number: 52,
                        a: String::from("When I remember your judgments of old, *"),
                        b: String::from("O LORD, I take great comfort.")
                    },
                    PsalmVerse {
                        number: 53,
                        a: String::from("I am filled with a burning rage, *"),
                        b: String::from("because of the wicked who forsake your law.")
                    },
                    PsalmVerse {
                        number: 54,
                        a: String::from("Your statutes have been like songs to me *"),
                        b: String::from("wherever I have lived as a stranger.")
                    },
                    PsalmVerse {
                        number: 55,
                        a: String::from("I remember your Name in the night, O LORD, *"),
                        b: String::from("and dwell upon your law.")
                    },
                    PsalmVerse {
                        number: 56,
                        a: String::from("This is how it has been with me, *"),
                        b: String::from("because I have kept your commandments.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 768
                },
                local_name: String::from("Heth"),
                latin_name: String::from("Portio mea, Domine"),
                verses: vec![
                    PsalmVerse {
                        number: 57,
                        a: String::from("You only are my portion, O LORD; *"),
                        b: String::from("I have promised to keep your words.")
                    },
                    PsalmVerse {
                        number: 58,
                        a: String::from("I entreat you with all my heart, *"),
                        b: String::from("be merciful to me according to your promise.")
                    },
                    PsalmVerse {
                        number: 59,
                        a: String::from("I have considered my ways *"),
                        b: String::from("and turned my feet toward your decrees.")
                    },
                    PsalmVerse {
                        number: 60,
                        a: String::from("I hasten and do not tarry *"),
                        b: String::from("to keep your commandments.")
                    },
                    PsalmVerse {
                        number: 61,
                        a: String::from("Though the cords of the wicked entangle me, *"),
                        b: String::from("I do not forget your law.")
                    },
                    PsalmVerse {
                        number: 62,
                        a: String::from("At midnight I will rise to give you thanks, *"),
                        b: String::from("because of your righteous judgments.")
                    },
                    PsalmVerse {
                        number: 63,
                        a: String::from("I am a companion of all who fear you *"),
                        b: String::from("and of those who keep your commandments.")
                    },
                    PsalmVerse {
                        number: 64,
                        a: String::from("The earth, O LORD, is full of your love; *"),
                        b: String::from("instruct me in your statutes.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 768
                },
                local_name: String::from("Teth"),
                latin_name: String::from("Bonitatem fecisti"),
                verses: vec![
                    PsalmVerse {
                        number: 65,
                        a: String::from("O LORD, you have dealt graciously with your servant, *"),
                        b: String::from("according to your word.")
                    },
                    PsalmVerse {
                        number: 66,
                        a: String::from("Teach me discernment and knowledge, *"),
                        b: String::from("for I have believed in your commandments.")
                    },
                    PsalmVerse {
                        number: 67,
                        a: String::from("Before I was afflicted I went astray, *"),
                        b: String::from("but now I keep your word.")
                    },
                    PsalmVerse {
                        number: 68,
                        a: String::from("You are good and you bring forth good; *"),
                        b: String::from("instruct me in your statutes.")
                    },
                    PsalmVerse {
                        number: 69,
                        a: String::from("The proud have smeared me with lies, *"),
                        b: String::from("but I will keep your commandments with my whole heart.")
                    },
                    PsalmVerse {
                        number: 70,
                        a: String::from("Their heart is gross and fat, *"),
                        b: String::from("but my delight is in your law.")
                    },
                    PsalmVerse {
                        number: 71,
                        a: String::from("It is good for me that I have been afflicted, *"),
                        b: String::from("that I might learn your statutes.")
                    },
                    PsalmVerse {
                        number: 72,
                        a: String::from("The law of your mouth is dearer to me *"),
                        b: String::from("than thousands in gold and silver.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 769
                },
                local_name: String::from("Yodh"),
                latin_name: String::from("Manus tuae fecerunt me"),
                verses: vec![
                    PsalmVerse {
                        number: 73,
                        a: String::from("Your hands have made me and fashioned me; *"),
                        b: String::from(
                            "give me understanding, that I may learn your commandments."
                        )
                    },
                    PsalmVerse {
                        number: 74,
                        a: String::from("Those who fear you will be glad when they see me, *"),
                        b: String::from("because I trust in your word.")
                    },
                    PsalmVerse {
                        number: 75,
                        a: String::from("I know, O LORD, that your judgments are right *"),
                        b: String::from("and that in faithfulness you have afflicted me.")
                    },
                    PsalmVerse {
                        number: 76,
                        a: String::from("Let your loving-kindness be my comfort, *"),
                        b: String::from("as you have promised to your servant.")
                    },
                    PsalmVerse {
                        number: 77,
                        a: String::from("Let your compassion come to me, that I may live, *"),
                        b: String::from("for your law is my delight.")
                    },
                    PsalmVerse {
                        number: 78,
                        a: String::from(
                            "Let the arrogant be put to shame, for they wrong me with lies; *"
                        ),
                        b: String::from("but I will meditate on your commandments.")
                    },
                    PsalmVerse {
                        number: 79,
                        a: String::from("Let those who fear you turn to me, *"),
                        b: String::from("and also those who know your decrees.")
                    },
                    PsalmVerse {
                        number: 80,
                        a: String::from("Let my heart be sound in your statutes, *"),
                        b: String::from("that I may not be put to shame.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 770
                },
                local_name: String::from("Kaph"),
                latin_name: String::from("Defecit in salutare"),
                verses: vec![
                    PsalmVerse {
                        number: 81,
                        a: String::from("My soul has longed for your salvation; *"),
                        b: String::from("I have put my hope in your word.")
                    },
                    PsalmVerse {
                        number: 82,
                        a: String::from("My eyes have failed from watching for your promise, *"),
                        b: String::from("and I say, “When will you comfort me?”")
                    },
                    PsalmVerse {
                        number: 83,
                        a: String::from("I have become like a leather flask in the smoke, *"),
                        b: String::from("but I have not forgotten your statutes.")
                    },
                    PsalmVerse {
                        number: 84,
                        a: String::from("How much longer must I wait? *"),
                        b: String::from(
                            "when will you give judgment against those who persecute me?"
                        )
                    },
                    PsalmVerse {
                        number: 85,
                        a: String::from("The proud have dug pits for me; *"),
                        b: String::from("they do not keep your law.")
                    },
                    PsalmVerse {
                        number: 86,
                        a: String::from("All your commandments are true; *"),
                        b: String::from("help me, for they persecute me with lies.")
                    },
                    PsalmVerse {
                        number: 87,
                        a: String::from("They had almost made an end of me on earth, *"),
                        b: String::from("but I have not forsaken your commandments.")
                    },
                    PsalmVerse {
                        number: 88,
                        a: String::from("In your loving-kindness, revive me, *"),
                        b: String::from("that I may keep the decrees of your mouth.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 770
                },
                local_name: String::from("Lamedh"),
                latin_name: String::from("In aeternum, Domine"),
                verses: vec![
                    PsalmVerse {
                        number: 89,
                        a: String::from("O LORD, your word is everlasting; *"),
                        b: String::from("it stands firm in the heavens.")
                    },
                    PsalmVerse {
                        number: 90,
                        a: String::from(
                            "Your faithfulness remains from one generation to another; *"
                        ),
                        b: String::from("you established the earth, and it abides.")
                    },
                    PsalmVerse {
                        number: 91,
                        a: String::from("By your decree these continue to this day, *"),
                        b: String::from("for all things are your servants.")
                    },
                    PsalmVerse {
                        number: 92,
                        a: String::from("If my delight had not been in your law, *"),
                        b: String::from("I should have perished in my affliction.")
                    },
                    PsalmVerse {
                        number: 93,
                        a: String::from("I will never forget your commandments, *"),
                        b: String::from("because by them you give me life.")
                    },
                    PsalmVerse {
                        number: 94,
                        a: String::from("I am yours; oh, that you would save me! *"),
                        b: String::from("for I study your commandments.")
                    },
                    PsalmVerse {
                        number: 95,
                        a: String::from("Though the wicked lie in wait for me to destroy me, *"),
                        b: String::from("I will apply my mind to your decrees.")
                    },
                    PsalmVerse {
                        number: 96,
                        a: String::from("I see that all things come to an end, *"),
                        b: String::from("but your commandment has no bounds.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 771
                },
                local_name: String::from("Mem"),
                latin_name: String::from("Quomodo dilexi!"),
                verses: vec![
                    PsalmVerse {
                        number: 97,
                        a: String::from("Oh, how I love your law! *"),
                        b: String::from("all the day long it is in my mind.")
                    },
                    PsalmVerse {
                        number: 98,
                        a: String::from("Your commandment has made me wiser than my enemies, *"),
                        b: String::from("and it is always with me.")
                    },
                    PsalmVerse {
                        number: 99,
                        a: String::from("I have more understanding than all my teachers, *"),
                        b: String::from("for your decrees are my study.")
                    },
                    PsalmVerse {
                        number: 100,
                        a: String::from("I am wiser than the elders, *"),
                        b: String::from("because I observe your commandments.")
                    },
                    PsalmVerse {
                        number: 101,
                        a: String::from("I restrain my feet from every evil way, *"),
                        b: String::from("that I may keep your word.")
                    },
                    PsalmVerse {
                        number: 102,
                        a: String::from("I do not shrink from your judgments, *"),
                        b: String::from("because you yourself have taught me.")
                    },
                    PsalmVerse {
                        number: 103,
                        a: String::from("How sweet are your words to my taste! *"),
                        b: String::from("they are sweeter than honey to my mouth.")
                    },
                    PsalmVerse {
                        number: 104,
                        a: String::from("Through your commandments I gain understanding; *"),
                        b: String::from("therefore I hate every lying way.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 772
                },
                local_name: String::from("Nun"),
                latin_name: String::from("Lucerna pedibus meis"),
                verses: vec![
                    PsalmVerse {
                        number: 105,
                        a: String::from("Your word is a lantern to my feet *"),
                        b: String::from("and a light upon my path.")
                    },
                    PsalmVerse {
                        number: 106,
                        a: String::from("I have sworn and am determined *"),
                        b: String::from("to keep your righteous judgments.")
                    },
                    PsalmVerse {
                        number: 107,
                        a: String::from("I am deeply troubled; *"),
                        b: String::from("preserve my life, O LORD, according to your word.")
                    },
                    PsalmVerse {
                        number: 108,
                        a: String::from("Accept, O LORD, the willing tribute of my lips, *"),
                        b: String::from("and teach me your judgments.")
                    },
                    PsalmVerse {
                        number: 109,
                        a: String::from("My life is always in my hand, *"),
                        b: String::from("yet I do not forget your law.")
                    },
                    PsalmVerse {
                        number: 110,
                        a: String::from("The wicked have set a trap for me, *"),
                        b: String::from("but I have not strayed from your commandments.")
                    },
                    PsalmVerse {
                        number: 111,
                        a: String::from("Your decrees are my inheritance for ever; *"),
                        b: String::from("truly, they are the joy of my heart.")
                    },
                    PsalmVerse {
                        number: 112,
                        a: String::from("I have applied my heart to fulfill your statutes *"),
                        b: String::from("for ever and to the end.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 772
                },
                local_name: String::from("Samekh"),
                latin_name: String::from("Iniquos odio habui"),
                verses: vec![
                    PsalmVerse {
                        number: 113,
                        a: String::from("I hate those who have a divided heart, *"),
                        b: String::from("but your law do I love.")
                    },
                    PsalmVerse {
                        number: 114,
                        a: String::from("You are my refuge and shield; *"),
                        b: String::from("my hope is in your word.")
                    },
                    PsalmVerse {
                        number: 115,
                        a: String::from("Away from me, you wicked! *"),
                        b: String::from("I will keep the commandments of my God.")
                    },
                    PsalmVerse {
                        number: 116,
                        a: String::from("Sustain me according to your promise, that I may live, *"),
                        b: String::from("and let me not be disappointed in my hope.")
                    },
                    PsalmVerse {
                        number: 117,
                        a: String::from("Hold me up, and I shall be safe, *"),
                        b: String::from("and my delight shall be ever in your statutes.")
                    },
                    PsalmVerse {
                        number: 118,
                        a: String::from("You spurn all who stray from your statutes; *"),
                        b: String::from("their deceitfulness is in vain.")
                    },
                    PsalmVerse {
                        number: 119,
                        a: String::from(
                            "In your sight all the wicked of the earth are but dross; *"
                        ),
                        b: String::from("therefore I love your decrees.")
                    },
                    PsalmVerse {
                        number: 120,
                        a: String::from("My flesh trembles with dread of you; *"),
                        b: String::from("I am afraid of your judgments.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 773
                },
                local_name: String::from("Ayin"),
                latin_name: String::from("Feci judicium"),
                verses: vec![
                    PsalmVerse {
                        number: 121,
                        a: String::from("I have done what is just and right; *"),
                        b: String::from("do not deliver me to my oppressors.")
                    },
                    PsalmVerse {
                        number: 122,
                        a: String::from("Be surety for your servant’s good; *"),
                        b: String::from("let not the proud oppress me.")
                    },
                    PsalmVerse {
                        number: 123,
                        a: String::from("My eyes have failed from watching for your salvation *"),
                        b: String::from("and for your righteous promise.")
                    },
                    PsalmVerse {
                        number: 124,
                        a: String::from(
                            "Deal with your servant according to your loving-kindness *"
                        ),
                        b: String::from("and teach me your statutes.")
                    },
                    PsalmVerse {
                        number: 125,
                        a: String::from("I am your servant; grant me understanding, *"),
                        b: String::from("that I may know your decrees.")
                    },
                    PsalmVerse {
                        number: 126,
                        a: String::from("It is time for you to act, O LORD, *"),
                        b: String::from("for they have broken your law.")
                    },
                    PsalmVerse {
                        number: 127,
                        a: String::from("Truly, I love your commandments *"),
                        b: String::from("more than gold and precious stones.")
                    },
                    PsalmVerse {
                        number: 128,
                        a: String::from("I hold all your commandments to be right for me; *"),
                        b: String::from("all paths of falsehood I abhor.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 774
                },
                local_name: String::from("Pe"),
                latin_name: String::from("Mirabilia"),
                verses: vec![
                    PsalmVerse {
                        number: 129,
                        a: String::from("Your decrees are wonderful; *"),
                        b: String::from("therefore I obey them with all my heart.")
                    },
                    PsalmVerse {
                        number: 130,
                        a: String::from("When your word goes forth it gives light; *"),
                        b: String::from("it gives understanding to the simple.")
                    },
                    PsalmVerse {
                        number: 131,
                        a: String::from("I open my mouth and pant; *"),
                        b: String::from("I long for your commandments.")
                    },
                    PsalmVerse {
                        number: 132,
                        a: String::from("Turn to me in mercy, *"),
                        b: String::from("as you always do to those who love your Name.")
                    },
                    PsalmVerse {
                        number: 133,
                        a: String::from("Steady my footsteps in your word; *"),
                        b: String::from("let no iniquity have dominion over me.")
                    },
                    PsalmVerse {
                        number: 134,
                        a: String::from("Rescue me from those who oppress me, *"),
                        b: String::from("and I will keep your commandments.")
                    },
                    PsalmVerse {
                        number: 135,
                        a: String::from("Let your countenance shine upon your servant *"),
                        b: String::from("and teach me your statutes.")
                    },
                    PsalmVerse {
                        number: 136,
                        a: String::from("My eyes shed streams of tears, *"),
                        b: String::from("because people do not keep your law.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 774
                },
                local_name: String::from("Sadhe"),
                latin_name: String::from("Justus es, Domine"),
                verses: vec![
                    PsalmVerse {
                        number: 137,
                        a: String::from("You are righteous, O LORD, *"),
                        b: String::from("and upright are your judgments.")
                    },
                    PsalmVerse {
                        number: 138,
                        a: String::from("You have issued your decrees *"),
                        b: String::from("with justice and in perfect faithfulness.")
                    },
                    PsalmVerse {
                        number: 139,
                        a: String::from("My indignation has consumed me, *"),
                        b: String::from("because my enemies forget your words.")
                    },
                    PsalmVerse {
                        number: 140,
                        a: String::from("Your word has been tested to the uttermost, *"),
                        b: String::from("and your servant holds it dear.")
                    },
                    PsalmVerse {
                        number: 141,
                        a: String::from("I am small and of little account, *"),
                        b: String::from("yet I do not forget your commandments.")
                    },
                    PsalmVerse {
                        number: 142,
                        a: String::from("Your justice is an everlasting justice *"),
                        b: String::from("and your law is the truth.")
                    },
                    PsalmVerse {
                        number: 143,
                        a: String::from("Trouble and distress have come upon me, *"),
                        b: String::from("yet your commandments are my delight.")
                    },
                    PsalmVerse {
                        number: 144,
                        a: String::from("The righteousness of your decrees is everlasting; *"),
                        b: String::from("grant me understanding, that I may live.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 775
                },
                local_name: String::from("Qoph"),
                latin_name: String::from("Clamavi in toto corde meo"),
                verses: vec![
                    PsalmVerse {
                        number: 145,
                        a: String::from("I call with my whole heart; *"),
                        b: String::from("answer me, O LORD, that I may keep your statutes.")
                    },
                    PsalmVerse {
                        number: 146,
                        a: String::from("I call to you;\noh, that you would save me! *"),
                        b: String::from("I will keep your decrees.")
                    },
                    PsalmVerse {
                        number: 147,
                        a: String::from("Early in the morning I cry out to you, *"),
                        b: String::from("for in your word is my trust.")
                    },
                    PsalmVerse {
                        number: 148,
                        a: String::from("My eyes are open in the night watches, *"),
                        b: String::from("that I may meditate upon your promise.")
                    },
                    PsalmVerse {
                        number: 149,
                        a: String::from(
                            "Hear my voice, O LORD, according to your loving-kindness; *"
                        ),
                        b: String::from("according to your judgments, give me life.")
                    },
                    PsalmVerse {
                        number: 150,
                        a: String::from("They draw near who in malice persecute me; *"),
                        b: String::from("they are very far from your law.")
                    },
                    PsalmVerse {
                        number: 151,
                        a: String::from("You, O LORD, are near at hand, *"),
                        b: String::from("and all your commandments are true.")
                    },
                    PsalmVerse {
                        number: 152,
                        a: String::from("Long have I known from your decrees *"),
                        b: String::from("that you have established them for ever.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 776
                },
                local_name: String::from("Resh"),
                latin_name: String::from("Vide humilitatem"),
                verses: vec![
                    PsalmVerse {
                        number: 153,
                        a: String::from("Behold my affliction and deliver me, *"),
                        b: String::from("for I do not forget your law.")
                    },
                    PsalmVerse {
                        number: 154,
                        a: String::from("Plead my cause and redeem me; *"),
                        b: String::from("according to your promise, give me life.")
                    },
                    PsalmVerse {
                        number: 155,
                        a: String::from("Deliverance is far from the wicked, *"),
                        b: String::from("for they do not study your statutes.")
                    },
                    PsalmVerse {
                        number: 156,
                        a: String::from("Great is your compassion, O LORD; *"),
                        b: String::from("preserve my life, according to your judgments.")
                    },
                    PsalmVerse {
                        number: 157,
                        a: String::from("There are many who persecute and oppress me, *"),
                        b: String::from("yet I have not swerved from your decrees.")
                    },
                    PsalmVerse {
                        number: 158,
                        a: String::from("I look with loathing at the faithless, *"),
                        b: String::from("for they have not kept your word.")
                    },
                    PsalmVerse {
                        number: 159,
                        a: String::from("See how I love your commandments! *"),
                        b: String::from("O LORD, in your mercy, preserve me.")
                    },
                    PsalmVerse {
                        number: 160,
                        a: String::from("The heart of your word is truth; *"),
                        b: String::from("all your righteous judgments endure for evermore.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 777
                },
                local_name: String::from("Shin"),
                latin_name: String::from("Principes persecuti sunt"),
                verses: vec![
                    PsalmVerse {
                        number: 161,
                        a: String::from("Rulers have persecuted me without a cause, *"),
                        b: String::from("but my heart stands in awe of your word.")
                    },
                    PsalmVerse {
                        number: 162,
                        a: String::from("I am as glad because of your promise *"),
                        b: String::from("as one who finds great spoils.")
                    },
                    PsalmVerse {
                        number: 163,
                        a: String::from("As for lies, I hate and abhor them, *"),
                        b: String::from("but your law is my love.")
                    },
                    PsalmVerse {
                        number: 164,
                        a: String::from("Seven times a day do I praise you, *"),
                        b: String::from("because of your righteous judgments.")
                    },
                    PsalmVerse {
                        number: 165,
                        a: String::from("Great peace have they who love your law; *"),
                        b: String::from("for them there is no stumbling block.")
                    },
                    PsalmVerse {
                        number: 166,
                        a: String::from("I have hoped for your salvation, O LORD, *"),
                        b: String::from("and have fulfilled your commandments.")
                    },
                    PsalmVerse {
                        number: 167,
                        a: String::from("I have kept your decrees *"),
                        b: String::from("and I have loved them deeply.")
                    },
                    PsalmVerse {
                        number: 168,
                        a: String::from("I have kept your commandments and decrees, *"),
                        b: String::from("for all my ways are before you.")
                    },
                ]
            },
            PsalmSection {
                reference: Reference {
                    source: Source::BCP1979,
                    page: 777
                },
                local_name: String::from("Taw"),
                latin_name: String::from("Appropinquet deprecatio"),
                verses: vec![
                    PsalmVerse {
                        number: 169,
                        a: String::from("Let my cry come before you, O LORD; *"),
                        b: String::from("give me understanding, according to your word.")
                    },
                    PsalmVerse {
                        number: 170,
                        a: String::from("Let my supplication come before you; *"),
                        b: String::from("deliver me, according to your promise.")
                    },
                    PsalmVerse {
                        number: 171,
                        a: String::from("My lips shall pour forth your praise, *"),
                        b: String::from("when you teach me your statutes.")
                    },
                    PsalmVerse {
                        number: 172,
                        a: String::from("My tongue shall sing of your promise, *"),
                        b: String::from("for all your commandments are righteous.")
                    },
                    PsalmVerse {
                        number: 173,
                        a: String::from("Let your hand be ready to help me, *"),
                        b: String::from("for I have chosen your commandments.")
                    },
                    PsalmVerse {
                        number: 174,
                        a: String::from("I long for your salvation, O LORD, *"),
                        b: String::from("and your law is my delight.")
                    },
                    PsalmVerse {
                        number: 175,
                        a: String::from("Let me live, and I will praise you, *"),
                        b: String::from("and let your judgments help me.")
                    },
                    PsalmVerse {
                        number: 176,
                        a: String::from("I have gone astray like a sheep that is lost; *"),
                        b: String::from(
                            "search for your servant,\n for I do not forget your commandments."
                        )
                    },
                ]
            }
        ]
    };
}
