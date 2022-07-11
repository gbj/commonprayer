use crate::{
    bcp1979::burial::parallels::*,
    rite1::{APOSTLES_CREED_TRADITIONAL, CANTICLE_4, CANTICLE_5, PASCHA_NOSTRUM_TRADITIONAL},
    rite2::LORDS_PRAYER_TRADITIONAL_TEXT,
};
use lazy_static::lazy_static;
use liturgy::*;

lazy_static! {
    pub static ref BURIAL_RITE_I: Document = Document::new()
        .version(Version::RiteI)
        .label("Burial I")
        .page(469)
        .content(Content::Liturgy(Liturgy::from(Series::from(vec![
            Document::from(Heading::from((HeadingLevel::Heading1, "The Burial of the Dead:\nRite One"))).tags([TITLE]),
            Document::from(Rubric::from("All stand while one or more of the following anthems are sung or said.\n\nA hymn, psalm, or some other suitable anthem may be sung instead.")).tags([OPENING_RUBRIC]),
            Document::from(Text::from("I am the resurrection and the life, saith the Lord; he that believeth in me, though he were dead, yet shall he live; and whosoever liveth and believeth in me shall never die.\n\nI know that my Redeemer liveth, and that he shall stand at the latter day upon the earth; and though this body be destroyed, yet shall I see God; whom I shall see for myself and mine eyes shall behold, and not as a stranger.\n\nFor none of us liveth to himself, and no man dieth to himself. For if we live, we live unto the Lord; and if we die, we die unto the Lord. Whether we live, therefore, or die, we are the Lord’s.\n\nBlessed are the dead who die in the Lord; even so saith the Spirit, for they rest from their labors.")).tags([ANTHEMS]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Celebrant says one of the following Collects, first saying")),
                Preces::from([
                    ("", "The Lord be with you."),
                    ("People", "And with thy spirit."),
                    ("Celebrant", "Let us pray.")
                ]).into(),
                Choice::from(vec![
                    Document::from(Text::from("O God, whose mercies cannot be numbered: Accept our prayers on behalf of thy servant *N.*, and grant *him* an entrance into the land of light and joy, in the fellowship of thy saints; through Jesus Christ thy Son our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.").response("Amen.")).label("At the Burial of an Adult"),
                    Document::from(Text::from("O God, whose beloved Son did take little children into his arms and bless them: Give us grace, we beseech thee, to entrust this child *N.* to thy never-failing care and love, and bring us all to thy heavenly kingdom; through the same thy Son Jesus Christ our Lord, who liveth and reigneth with thee and the Holy Spirit, one God, now and for ever.").response("Amen.")).label("At the Burial of a Child"),
                ]).into(),
                Document::from(Rubric::from("The people sit."))
            ])).tags([OPENING_PRAYERS]),

            Document::from(Rubric::from("One or more of the following passages from Holy Scripture is read. If there is to be a Communion, a passage from the Gospel always concludes the Readings.")).tags([LESSONS_RUBRIC]),

            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("Isaiah 25:6-9").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Isaiah.")))).label("From the Old Testament").version_label("Isaiah 25:6-9 (He will swallow up death in victory)"),
                Document::from(BiblicalCitation::from("Isaiah 61:1-3").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Isaiah.")))).label("From the Old Testament").version_label("Isaiah 61:1-3 (To comfort all that mourn)"),
                Document::from(BiblicalCitation::from("Lamentations 3:22-26, 31-33").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Lamentations.")))).label("From the Old Testament").version_label("Lamentations 3:22-26, 31-33 (The Lord is good unto them that wait for him)"),
                Document::from(BiblicalCitation::from("Wisdom 3:1-5, 9").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Wisdom of Solomon.")))).label("From the Old Testament").version_label("Wisdom 3:1-5, 9 (The souls of the righteous are in the hands of God)"),
                Document::from(BiblicalCitation::from("Job 19:21-27a").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Job.")))).label("From the Old Testament").version_label("Job 19:21-27a (I know that my Redeemer liveth)"),
            ])).tags([FIRST_LESSON]),

            Document::from(Rubric::from("After the Old Testament Lesson, a suitable canticle or one of the following Psalms may be sung or said")).tags([PSALM_RUBRIC]),
            Document::from(Choice::from(vec![
                Document::from(Psalm {
                    citation: None,
                    number: 42,
                    sections: vec![PsalmSection {
                        reference: Reference {
                            source: Source::BCP1979,
                            page: 471
                        },
                        local_name: String::from(""),
                        latin_name: String::from("Quemadmodum"),
                        verses: vec![
                            PsalmVerse {
                                number: 1,
                                a: "Like as the hart desireth the water-brooks, *".into(),
                                b: "so longeth my soul after thee, O God.".into()
                            },
                            PsalmVerse {
                                number: 2,
                                a: "My soul is athirst for God, yea, even for the living God; *".into(),
                                b: "when shall I come to appear before the presence of God?".into()
                            },
                            PsalmVerse {
                                number: 3,
                                a: "My tears have been my meat day and night, *".into(),
                                b: "while they daily say unto me, Where is now thy God?".into()
                            },
                            PsalmVerse {
                                number: 4,
                                a: "Now when I think thereupon, I pour out my heart by myself; *".into(),
                                b: "for I went with the multitude, and brought them forth into the house of God".into()
                            },
                            PsalmVerse {
                                number: 5,
                                a: "In the voice of praise and thanksgiving, *".into(),
                                b: "among such as keep holy-day.".into()
                            },
                            PsalmVerse {
                                number: 6,
                                a: "Why art thou so full of heaviness, O my soul? *".into(),
                                b: "and why art thou so disquieted within me?".into()
                            },
                            PsalmVerse {
                                number: 7,
                                a: "O put thy trust in God; *".into(),
                                b: "for I will yet thank him, which is the help of my\ncountenance, and my God.".into()
                            }
                        ]
                    }]
                }),
                Document::from(Psalm {
                    citation: None,
                    number: 46,
                    sections: vec![PsalmSection {
                        reference: Reference {
                            source: Source::BCP1979,
                            page: 471
                        },
                        local_name: String::from(""),
                        latin_name: String::from("Deus noster refugium"),
                        verses: vec![
                            PsalmVerse {
                                number: 1,
                                a: "God is our hope and strength, *".into(),
                                b: "a very present help in trouble.".into()
                            },
                            PsalmVerse {
                                number: 2,
                                a: "Therefore will we not fear, though the earth be moved, *".into(),
                                b: "and though the hills be carried into the midst of the sea;".into()
                            },
                            PsalmVerse {
                                number: 3,
                                a: "Though the waters thereof rage and swell, *".into(),
                                b: "and though the mountains shake at the tempest of the same.".into()
                            },
                            PsalmVerse {
                                number: 4,
                                a: "There is a river, the streams whereof make glad the city of God, *".into(),
                                b: "the holy place of the tabernacle of the Most Highest.".into()
                            },
                            PsalmVerse {
                                number: 5,
                                a: "God is in the midst of her,\ntherefore shall she not be removed; *".into(),
                                b: "God shall help her, and that right early.".into()
                            },
                            PsalmVerse {
                                number: 6,
                                a: "Be still then, and know that I am God; *".into(),
                                b: "I will be exalted among the nations,\nand I will be exalted in the earth.".into()
                            },
                            PsalmVerse {
                                number: 7,
                                a: "The LORD of hosts is with us; *".into(),
                                b: "the God of Jacob is our refuge.".into()
                            },
                        ]
                    }]
                }),
                Document::from(Psalm {
                    citation: None,
                    number: 90,
                    sections: vec![PsalmSection {
                        reference: Reference {
                            source: Source::BCP1979,
                            page: 472
                        },
                        local_name: String::from(""),
                        latin_name: String::from("Domine, refugium"),
                        verses: vec![
                            PsalmVerse {
                                number: 1,
                                a: "LORD, thou hast been our refuge, *".into(),
                                b: "from one generation to another.".into()
                            },
                            PsalmVerse {
                                number: 2,
                                a: "Before the mountains were brought forth,\nor ever the earth and the world were made, *".into(),
                                b: "thou art God from everlasting, and world without end.".into()
                            },
                            PsalmVerse {
                                number: 3,
                                a: "Thou turnest man to destruction; *".into(),
                                b: "again thou sayest, Come again, ye children of men.".into()
                            },
                            PsalmVerse {
                                number: 4,
                                a: "For a thousand years in thy sight are but as yesterday when it is past, *".into(),
                                b: "and as a watch in the night.".into()
                            },
                            PsalmVerse {
                                number: 5,
                                a: "As soon as thou scatterest them they are even as a sleep, *".into(),
                                b: "and fade away suddenly like the grass.".into()
                            },
                            PsalmVerse {
                                number: 6,
                                a: "In the morning it is green, and groweth up; *".into(),
                                b: "but in the evening it is cut down, dried up, and withered.".into()
                            },
                            PsalmVerse {
                                number: 7,
                                a: "For we consume away in thy displeasure, *".into(),
                                b: "and are afraid at thy wrathful indignation.".into()
                            },
                            PsalmVerse {
                                number: 8,
                                a: "Thou hast set our misdeeds before thee, *".into(),
                                b: "and our secret sins in the light of thy countenance.".into()
                            },
                            PsalmVerse {
                                number: 9,
                                a: "For when thou art angry all our days are gone; *".into(),
                                b: "we bring our years to an end, as it were a tale that is told.".into()
                            },
                            PsalmVerse {
                                number: 10,
                                a: "The days of our age are threescore years and ten;\nand though men be so strong that they come to fourscore years, *".into(),
                                b: "yet is their strength then but labor and sorrow,\nso soon passeth it away, and we are gone.".into()
                            },
                            PsalmVerse {
                                number: 11,
                                a: "So teach us to number our days, *".into(),
                                b: "that we may apply our hearts unto wisdom.".into()
                            },
                        ]
                    }]
                }),
                Document::from(Psalm {
                    citation: None,
                    number: 121,
                    sections: vec![PsalmSection {
                        reference: Reference {
                            source: Source::BCP1979,
                            page: 473
                        },
                        local_name: String::from(""),
                        latin_name: String::from("Levavi oculos"),
                        verses: vec![
                            PsalmVerse {
                                number: 1,
                                a: "I will lift up mine eyes unto the hills; *".into(),
                                b: "from whence cometh my help?".into()
                            },
                            PsalmVerse {
                                number: 2,
                                a: "My help cometh even from the LORD".into(),
                                b: "who hath made heaven and earth.".into()
                            },
                            PsalmVerse {
                                number: 3,
                                a: "He will not suffer thy foot to be moved, *".into(),
                                b: "and he that keepeth thee will not sleep.".into()
                            },
                            PsalmVerse {
                                number: 4,
                                a: "Behold, he that keepeth Israel *".into(),
                                b: "shall neither slumber nor sleep.".into()
                            },
                            PsalmVerse {
                                number: 5,
                                a: "The LORD himself is thy keeper; *".into(),
                                b: "the LORD is thy defence upon thy right hand;".into()
                            },
                            PsalmVerse {
                                number: 6,
                                a: "So that the sun shall not burn thee by day, *".into(),
                                b: "neither the moon by night.".into()
                            },
                            PsalmVerse {
                                number: 7,
                                a: "The LORD shall preserve thee from all evil; *".into(),
                                b: "yea, it is even he that shall keep thy soul.".into()
                            },
                            PsalmVerse {
                                number: 8,
                                a: "The LORD shall preserve thy going out, and thy coming in, *".into(),
                                b: "from this time forth for evermore.".into()
                            },
                        ]
                    }]
                }),
                Document::from(Psalm {
                    citation: None,
                    number: 130,
                    sections: vec![PsalmSection {
                        reference: Reference {
                            source: Source::BCP1979,
                            page: 474
                        },
                        local_name: String::from(""),
                        latin_name: String::from("De profundis"),
                        verses: vec![
                            PsalmVerse {
                                number: 1,
                                a: "Out of the deep have I called unto thee, O LORD; *".into(),
                                b: "Lord, hear my voice.".into()
                            },
                            PsalmVerse {
                                number: 2,
                                a: "O let thine ears consider well *".into(),
                                b: "the voice of my complaint.".into()
                            },
                            PsalmVerse {
                                number: 3,
                                a: "If thou LORD, wilt be extreme to mark what is done amiss, *".into(),
                                b: "O Lord, who may abide it?".into()
                            },
                            PsalmVerse {
                                number: 4,
                                a: "For there is mercy with thee, *".into(),
                                b: "therefore shalt thou be feared.".into()
                            },
                            PsalmVerse {
                                number: 5,
                                a: "I look for the LORD; my soul doth wait for him; ".into(),
                                b: "in his word is my trust.".into()
                            },
                            PsalmVerse {
                                number: 6,
                                a: "".into(),
                                b: "".into()
                            },
                            PsalmVerse {
                                number: 7,
                                a: "My soul fleeth unto the Lord before the morning watch; *".into(),
                                b: "I say, before the morning watch.".into()
                            },
                            PsalmVerse {
                                number: 8,
                                a: "O Israel, trust in the LORD,\nfor with the LORD there is mercy, *".into(),
                                b: "and with him is plenteous redemption.".into()
                            },
                            PsalmVerse {
                                number: 9,
                                a: "And he shall redeem Israel *".into(),
                                b: "from all his sins.".into()
                            }
                        ]
                    }]
                }),
                Document::from(Psalm {
                    citation: None,
                    number: 139,
                    sections: vec![PsalmSection {
                        reference: Reference {
                            source: Source::BCP1979,
                            page: 474
                        },
                        local_name: String::from(""),
                        latin_name: String::from("Domine, probasti"),
                        verses: vec![
                            PsalmVerse {
                                number: 1,
                                a: "O LORD, thou hast searched me out, and known me. *".into(),
                                b: "Thou knowest my down-sitting and mine up-rising;\nthou understandest my thoughts long before.".into()
                            },
                            PsalmVerse {
                                number: 2,
                                a: "Thou art about my path, and about my bed, *".into(),
                                b: "and art acquainted with all my ways.".into()
                            },
                            PsalmVerse {
                                number: 3,
                                a: "For lo, there is not a word in my tongue, *".into(),
                                b: "But thou, O LORD, knowest it altogether.".into()
                            },
                            PsalmVerse {
                                number: 4,
                                a: "Thou hast beset me behind and before, *".into(),
                                b: "and laid thine hand upon me.".into()
                            },
                            PsalmVerse {
                                number: 5,
                                a: "Such knowledge is too wonderful and excellent for me; *".into(),
                                b: "I cannot attain unto it.".into()
                            },
                            PsalmVerse {
                                number: 6,
                                a: "Whither shall I go then from thy Spirit? *".into(),
                                b: "or whither shall I go then from thy presence?".into()
                            },
                            PsalmVerse {
                                number: 7,
                                a: "If I climb up into heaven, thou art there; *".into(),
                                b: "if I go down to hell, thou art there also.".into()
                            },
                            PsalmVerse {
                                number: 8,
                                a: "If I take the wings of the morning, *".into(),
                                b: "and remain in the uttermost parts of the sea;".into()
                            },
                            PsalmVerse {
                                number: 9,
                                a: "Even there also shall thy hand lead me, *".into(),
                                b: "and thy right hand shall hold me.".into()
                            },
                            PsalmVerse {
                                number: 10,
                                a: "If I say, Peradventure the darkness shall cover me, *".into(),
                                b: "then shall my night be turned to day.".into()
                            },
                            PsalmVerse {
                                number: 11,
                                a: "Yea, the darkness is no darkness with thee,\nbut the night is as clear as day; *".into(),
                                b: "the darkness and light to thee are both alike.".into()
                            },
                        ]
                    }]
                })
            ])).tags([PSALM]),

            Document::from(Choice::from(vec![
                Document::from(BiblicalCitation::from("Romans 8:14-19, 34-35, 37-39").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Letter to the Romans.")))).label("From the New Testament").version_label("Romans 8:14-19, 34-35, 37-39 (The glory that shall be revealed)"),
                Document::from(BiblicalCitation::from("1 Corinthians 15:20-26, 35-38, 42-44, 53-58").intro(BiblicalReadingIntro::from(Document::from("A Reading from the First Letter to the Corinthians.")))).label("From the New Testament").version_label("1 Corinthians 15:20-26, 35-38, 42-44, 53-58 (Raised in incorruption)"),
                Document::from(BiblicalCitation::from("2 Corinthians 4:16-5:9").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Second Letter to the Corinthians.")))).label("From the New Testament").version_label("2 Corinthians 4:16–5:9 (Things which are not seen are eternal)"),
                Document::from(BiblicalCitation::from("1 John 3:1-2").intro(BiblicalReadingIntro::from(Document::from("A Reading from the First Letter of John.")))).label("From the New Testament").version_label("1 John 3:1-2 (We shall be like him)"),
                Document::from(BiblicalCitation::from("Revelation 7:9-17").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Revelation.")))).label("From the New Testament").version_label("Revelation 7:9-17 (God shall wipe away all tears)"),
                Document::from(BiblicalCitation::from("Revelation 21:2-7").intro(BiblicalReadingIntro::from(Document::from("A Reading from the Book of Revelation.")))).label("From the New Testament").version_label("Revelation 21:2-7 (Behold, I make all things new)"),
            ])).tags([SECOND_LESSON]),

            Document::from(Rubric::from("After the New Testament Lesson, a suitable canticle or hymn, or one of the following Psalms may be sung or said")).tags([PSALM_RUBRIC_2]),
            Document::from(Choice::from(vec![
                Document::from(Psalm {
                    citation: None,
                    number: 23,
                    sections: vec![PsalmSection {
                        reference: Reference {
                            source: Source::BCP1979,
                            page: 376
                        },
                        local_name: String::from(""),
                        latin_name: String::from("Dominus regit me"),
                        verses: vec![
                            PsalmVerse {
                                number: 1,
                                a: "The LORD is my shepherd; *".into(),
                                b: "therefore can I lack nothing.".into()
                            },
                            PsalmVerse {
                                number: 2,
                                a: "He shall feed me in a green pasture, *".into(),
                                b: "and lead me forth beside the waters of comfort.".into()
                            },
                            PsalmVerse {
                                number: 3,
                                a: "He shall convert my soul, *".into(),
                                b: "and bring me forth in the paths of righteousness for his Name’s sake.".into()
                            },
                            PsalmVerse {
                                number: 4,
                                a: "Yea, though I walk through the valley of the shadow of death,\nI will fear no evil; *".into(),
                                b: "for thou art with me;\nthy rod and thy staff comfort me.".into()
                            },
                            PsalmVerse {
                                number: 5,
                                a: "Thou shalt prepare a table before me in the presence of them that trouble me; *".into(),
                                b: "thou hast anointed my head with oil,\nand my cup shall be full.".into()
                            },
                            PsalmVerse {
                                number: 6,
                                a: "Surely thy loving-kindness and mercy shall follow me all the days of my life; *".into(),
                                b: "and I will dwell in the house of the LORD for ever.".into()
                            },
                        ]
                    }]
                }),
                Document::from(Psalm {
                    citation: None,
                    number: 23,
                    sections: vec![PsalmSection {
                        reference: Reference {
                            source: Source::BCP1979,
                            page: 376
                        },
                        local_name: String::from(""),
                        latin_name: String::from("King James Version"),
                        verses: vec![
                            PsalmVerse {
                                number: 1,
                                a: "The LORD is my shepherd; *".into(),
                                b: "I shall not want.".into()
                            },
                            PsalmVerse {
                                number: 2,
                                a: "He maketh me to lie down in green pastures; *".into(),
                                b: "he leadeth me beside the still waters.".into()
                            },
                            PsalmVerse {
                                number: 3,
                                a: "He restoreth my soul; *".into(),
                                b: "He leadeth me in the paths of righteousness for his Name’s sake.".into()
                            },
                            PsalmVerse {
                                number: 4,
                                a: "Yea, though I walk through the valley of the shadow of death,\nI will fear no evil; *".into(),
                                b: "for thou art with me;\nthy rod and thy staff, they comfort me.".into()
                            },
                            PsalmVerse {
                                number: 5,
                                a: "Thou preparest a table before me in the presence of mine enemies; *".into(),
                                b: "thou anointest my head with oil;\nmy cup runneth over.".into()
                            },
                            PsalmVerse {
                                number: 6,
                                a: "Surely goodness and mercy shall follow me all the days of my life, *".into(),
                                b: "and I will dwell in the house of the LORD for ever.".into()
                            },
                        ]
                    }]
                }),
                Document::from(Psalm {
                    number: 27,
                    citation: None,
                    sections: vec![PsalmSection {
                        reference: Reference {
                            source: Source::BCP1979,
                            page: 477
                        },
                        local_name: String::from(""),
                        latin_name: String::from("Dominus illuminatio"),
                        verses: vec![
                            PsalmVerse {
                                number: 1,
                                a: "The Lord is my light and my salvation;\nwhom then shall I fear? *".into(),
                                b: "the LORD is the strength of my life;\nof whom then shall I be afraid?".into()
                            },
                            PsalmVerse {
                                number: 2,
                                a: "One thing have I desired of the LORD, which I will require, *".into(),
                                b: "even that I may dwell in the house of the LORD all the days of my life,\nto behold the fair beauty of the LORD, and to visit his temple.".into()
                            },
                            PsalmVerse {
                                number: 3,
                                a: "For in the time of trouble he shall hide me in his tabernacle; *".into(),
                                b: "yea, in the secret place of his dwelling shall he hide me,\nand set me up upon a rock of stone.".into()
                            },
                            PsalmVerse {
                                number: 4,
                                a: "And now shall he lift up mine head *".into(),
                                b: "above mine enemies round about me.".into()
                            },
                            PsalmVerse {
                                number: 5,
                                a: "Therefore will I offer in his dwelling an oblation with great gladness; *".into(),
                                b: "I will sing and speak praises unto the LORD.".into()
                            },
                            PsalmVerse {
                                number: 6,
                                a: "Hearken unto my voice, O LORD, when I cry unto thee; *".into(),
                                b: "have mercy upon me, and hear me.".into()
                            },
                            PsalmVerse {
                                number: 7,
                                a: "My heart hath talked of thee, Seek ye my face. *".into(),
                                b: "Thy face, LORD, will I seek.".into()
                            },
                            PsalmVerse {
                                number: 8,
                                a: "O hide not thou thy face from me, *".into(),
                                b: "nor cast thy servant away in displeasure.".into()
                            },
                            PsalmVerse {
                                number: 9,
                                a: "I should utterly have fainted, *".into(),
                                b: "but that I believe verily to see the goodness of the LORD in the land of the living.".into()
                            },
                            PsalmVerse {
                                number: 10,
                                a: "O tarry thou the LORD’s leisure; *".into(),
                                b: "be strong, and he shall comfort thine heart;\nand put thou thy trust in the LORD.".into()
                            },
                        ]
                    }]
                }),
                Document::from(Psalm {
                    number: 106,
                    citation: None,
                    sections: vec![PsalmSection {
                        reference: Reference {
                            source: Source::BCP1979,
                            page: 478
                        },
                        local_name: String::from(""),
                        latin_name: String::from("Confitemini Domino"),
                        verses: vec![
                            PsalmVerse {
                                number: 1,
                                a: "O give thanks unto the LORD, for he is gracious, *".into(),
                                b: "and his mercy endureth for ever.".into()
                            },
                            PsalmVerse {
                                number: 2,
                                a: "Who can express the noble acts of the LORD, *".into(),
                                b: "or show forth all his praise?".into()
                            },
                            PsalmVerse {
                                number: 3,
                                a: "Blessed are they that alway keep judgment, *".into(),
                                b: "and do righteousness.".into()
                            },
                            PsalmVerse {
                                number: 4,
                                a: "Remember me, O LORD, according to the favor that thou bearest unto thy people; *".into(),
                                b: "O visit me with thy salvation;".into()
                            },
                            PsalmVerse {
                                number: 5,
                                a: "That I may see the felicity of thy chosen, *".into(),
                                b: "and rejoice in the gladness of thy people,\nand give thanks with thine inheritance.".into()
                            },
                        ]
                    }]
                }),
                Document::from(Psalm {
                    number: 116,
                    citation: None,
                    sections: vec![PsalmSection {
                        reference: Reference {
                            source: Source::BCP1979,
                            page: 578
                        },
                        local_name: String::from(""),
                        latin_name: String::from("Dilexi, quoniam"),
                        verses: vec![
                            PsalmVerse {
                                number: 1,
                                a: "My delight is in the LORD, *".into(),
                                b: "because he hath heard the voice of my prayer;".into()
                            },
                            PsalmVerse {
                                number: 2,
                                a: "Because he hath inclined his ear unto me, *".into(),
                                b: "therefore will I call upon him as long as I live.".into()
                            },
                            PsalmVerse {
                                number: 3,
                                a: "The snares of death compassed me round about, *".into(),
                                b: "and the pains of hell gat hold upon me.".into()
                            },
                            PsalmVerse {
                                number: 4,
                                a: "I found trouble and heaviness;\nthen called I upon the Name of the LORD; *".into(),
                                b: "O LORD, I beseech thee, deliver my soul.".into()
                            },
                            PsalmVerse {
                                number: 5,
                                a: "Gracious is the LORD, and righteous; *".into(),
                                b: "yea, our God is merciful.".into()
                            },
                            PsalmVerse {
                                number: 6,
                                a: "The LORD preserveth the simple; *".into(),
                                b: "I was in misery, and he helped me.".into()
                            },
                            PsalmVerse {
                                number: 7,
                                a: "Turn again then unto thy rest, O my soul, *".into(),
                                b: "for the LORD hath rewarded thee.".into()
                            },
                            PsalmVerse {
                                number: 8,
                                a: "And why? thou hast delivered my soul from death, *".into(),
                                b: "mine eyes from tears, and my feet from falling.".into()
                            },
                            PsalmVerse {
                                number: 9,
                                a: "I will walk before the LORD *".into(),
                                b: "in the land of the living.".into()
                            },
                            PsalmVerse {
                                number: 10,
                                a: "I will pay my vows now in the presence of all his people; *".into(),
                                b: "right dear in the sight of the LORD is the death of his saints.".into()
                            },
                        ]
                    }]
                }),
            ])).tags([PSALM_2]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("Then, all standing, the Deacon or Minister appointed reads the Gospel, first saying")),
                Document::from(Preces::from([
                    ("", "The Holy Gospel of our Lord Jesus Christ according to John."),
                    ("People", "Glory be to thee, O Lord.")
                ]))
            ]))
                .display(Show::TemplateOnly)
                .tags([GOSPEL_RUBRIC]),

            Document::from(Choice::from(vec![
                Document::from(
                    BiblicalCitation::from("John 5:24-27")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to John."),
                            ("People", "Glory be to thee, O Lord.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("John 5:24-27 (He that believeth hath everlasting life)"),
                Document::from(
                    BiblicalCitation::from("John 6:37-40")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to John."),
                            ("People", "Glory be to thee, O Lord.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("John 6:37-40 (All that the Father giveth me shall come to me)"),
               Document::from(
                    BiblicalCitation::from("John 10:11-16")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to John."),
                            ("People", "Glory be to thee, O Lord.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("John 10:11-16 (I am the good shepherd)"),
                Document::from(
                    BiblicalCitation::from("John 11:21-27")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to John."),
                            ("People", "Glory be to thee, O Lord.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("John 11:21-27 (I am the resurrection and the life)"),
                Document::from(
                    BiblicalCitation::from("John 14:1-6")
                        .intro(BiblicalReadingIntro::from(Document::from(Preces::from([
                            ("", "The Holy Gospel of our Lord Jesus Christ according to John."),
                            ("People", "Glory to you, Lord Christ.")
                        ]))))
                )
                    .label("The Gospel")
                    .version_label("John 14:1-6 (In my Father’s house are many mansions)"),
            ])).tags([GOSPEL]),
            Document::from(Rubric::from("At the end of the Gospel, the Reader says")).tags([GOSPEL]),
            Document::from(Preces::from([
                ("", "The Gospel of the Lord."),
                ("People", "Praise be to thee, O Christ.")
            ])).tags([GOSPEL]),

            Document::from(Rubric::from("A homily may be preached, the people being seated.")).tags([HOMILY]),

            Document::from(Series::from(vec![
                Document::from(Rubric::from("The Apostles’ Creed may be said, all standing.")),
                Document::from(Content::DocumentLink{
                    label: "Apostles’ Creed".into(),
                    path: SlugPath::from([Slug::ApostlesCreed, Slug::Version(Version::RiteI)]),
                    rotate: false,
                    link_only: false
                }).display(Show::TemplateOnly)
            ])).tags([CREED]),
            APOSTLES_CREED_TRADITIONAL.clone().display(Show::CompiledOnly),

            // Prayers
            Document::from(Rubric::from("If there is not to be a Communion, the Lord’s Prayer is said here, and the service continues with the following prayer of intercession, or with one or more suitable prayers (see pages 487-489).\n\nWhen there is a Communion, the following serves for the Prayers of the People.")).tags([RUBRIC_BEFORE_PRAYERS]),
            Document::from(Content::DocumentLink {
                label: "Additional Prayers".into(),
                path: SlugPath::from([Slug::Burial, Slug::AdditionalPrayers, Slug::Version(Version::RiteI)]),
                rotate: false,
                link_only: false
            }).version(Version::RiteI).tags([RUBRIC_BEFORE_PRAYERS]),

            Document::from(Rubric::from("The People respond to every petition with Amen.\n\nThe Deacon or other leader says")).tags([PRAYERS_RUBRIC]),
            Document::from(Series::from(vec![
                Document::from(Litany::from((
                    "Amen.",
                    [
                        "In peace, let us pray to the Lord.\n\nAlmighty God, who hast knit together thine elect in one communion and fellowship, in the mystical body of thy Son Christ our Lord: Grant, we beseech thee, to thy whole Church in paradise and on earth, thy light and thy peace.",
                        "Grant that all who have been baptized into Christ’s death and resurrection may die to sin and rise to newness of life, and that through the grave and gate of death we may pass with him to our joyful resurrection.",
                        "Grant to us who are still in our pilgrimage, and who walk as yet by faith, that thy Holy Spirit may lead us in holiness and righteousness all our days.",
                        "Grant to thy faithful people pardon and peace, that we may be cleansed from all our sins, and serve thee with a quiet mind.",
                        "Grant to all who mourn a sure confidence in thy fatherly care, that, casting all their grief on thee, they may know the consolation of thy love.",
                        "| Give courage and faith to those who are bereaved, that they may have strength to meet the days ahead in the comfort of a reasonable and holy hope, in the joyful expectation of eternal life with those they love.",
                        "| Help us, we pray, in the midst of things we cannot understand, to believe and trust in the communion of saints, the forgiveness of sins, and the resurrection to life everlasting.",
                        "| Grant us grace to entrust *N.* to thy never-failing love; receive *him* into the arms of thy mercy, and remember him according to the favor which thou bearest unto thy people.",
                        "| Grant that, increasing in knowledge and love of thee, *he* may go from strength to strength in the life of perfect service in thy heavenly kingdom.",
                        "Grant us, with all who have died in the hope of the resurrection, to have our consummation and bliss in thy eternal and everlasting glory, and, with [blessed *N.* and] all thy saints, to receive the crown of life which thou dost promise to all who share in the victory of thy Son Jesus Christ; who liveth and reigneth with thee and the Holy Spirit, one God, for ever and ever.",
                    ]
                ))),
            ])).tags([PRAYERS]),

            Document::from(Rubric::from("When there is no Communion, the service continues with the Commendation, or with the Committal.")).tags([RUBRIC_AFTER_PRAYERS]),

            Document::from(Heading::from((HeadingLevel::Heading2, "At the Eucharist."))).tags([AT_THE_EUCHARIST_TITLE]),
            Document::from(Rubric::from("The service continues with the Peace and the Offertory.")).tags([AT_THE_EUCHARIST]),
            Document::from(Content::DocumentLink {
                label: "Preface of the Commemoration of the Dead".into(),
                path: SlugPath::from([Slug::Eucharist, Slug::ProperPrefaces, Slug::Burial, Slug::Version(Version::RiteI)]),
                rotate: false,
                link_only: false
            }).tags([PROPER_PREFACE]),
            Document::from(Rubric::from("In place of the usual postcommunion prayer, the following is said")).tags([POSTCOMMUNION_PRAYER]),
            Document::from(Text::from("Almighty God, we thank thee that in thy great love thou hast fed us with the spiritual food and drink of the Body and Blood of thy Son Jesus Christ, and hast given unto us a foretaste of thy heavenly banquet. Grant that this Sacrament may be unto us a comfort in affliction, and a pledge of our inheritance in that kingdom where there is no death, neither sorrow nor crying, but the fullness of joy with all thy saints; through Jesus Christ our Savior.").response("Amen.").display_format(DisplayFormat::Unison)).tags([POSTCOMMUNION_PRAYER]),

            Document::from(Rubric::from("If the body is not present, the service continues with the [blessing and] dismissal.\n\nUnless the Committal follows immediately in the church, the following Commendation is used.")).tags([RUBRICS_BEFORE_COMMENDATION]),

            // Commendation
            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "The Commendation"))),
                Document::from(Rubric::from("The Celebrant and other ministers take their places at the body.\n\nThis anthem, or some other suitable anthem, or a hymn, may be sung or said.")),
                Document::from(ResponsivePrayer::from([
                    "Give rest, O Christ, to thy servant(s) with thy saints,",
                    "where sorrow and pain are no more,\nneither sighing, but life everlasting.\n",
                    "Thou only art immortal, the creator and maker of mankind; and we are mortal, formed of the earth, and unto earth shall we return. For so thou didst ordain when thou createdst me, saying, “Dust thou art, and unto dust shalt thou return.” All we go down to the dust; yet even at the grave we make our song: Alleluia, alleluia, alleluia.",
                    "Give rest, O Christ, to thy servant(s) with thy saints, where sorrow and pain are no more, neither sighing, but life everlasting.",
                ])),
                Document::from(Rubric::from("The Celebrant, facing the body, says")),
                Document::from(Text::from("Into thy hands, O merciful Savior, we commend thy servant *N.* Acknowledge, we humbly beseech thee, a sheep of thine own fold, a lamb of thine own flock, a sinner of thine own redeeming. Receive *him* into the arms of thy mercy, into the blessed rest of everlasting peace, and into the glorious company of the saints in light.").response("Amen.")),
                Document::from(Rubric::from("The Celebrant, or the Bishop if present, may then bless the people, and a Deacon or other Minister may dismiss them, saying")),
                Document::from(ResponsivePrayer::from([
                    "Let us go forth in the name of Christ.",
                    "Thanks be to God."
                ])),
                Document::from(Rubric::from("As the body is borne from the church, a hymn, or one or more of these anthems may be sung or said")),
                Document::from(Choice::from(vec![
                    Document::from(Text::from("Christ is risen from the dead, trampling down death by death, and giving life to those in the tomb. \n\nThe Sun of Righteousness is gloriously risen, giving light to those who sat in darkness and in the shadow of death. \n\nThe Lord will guide our feet into the way of peace, having taken away the sin of the world. \n\nChrist will open the kingdom of heaven to all who believe in his Name, saying, Come, O blessed of my Father; inherit the kingdom prepared for you. \n\nInto paradise may the angels lead thee. At thy coming may the martyrs receive thee, and bring thee into the holy city Jerusalem.")).version_label("Anthems").version(Version::RiteI),
                    CANTICLE_4.clone(),
                    CANTICLE_5.clone(),
                    Document::from(PASCHA_NOSTRUM_TRADITIONAL.clone()).version_label("Pascha Nostrum").version(Version::RiteI),
                ])),
            ])).tags([COMMENDATION]),

            // Committal
            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "The Committal"))),
                Document::from(Rubric::from("The following anthem is sung or said")),
                Document::from(Choice::from(vec![
                    Document::from(Text::from("In the midst of life we are in death;\nof whom may we seek for succor,\nbut of thee, O Lord,\nwho for our sins art justly displeased?\n\nYet, O Lord God most holy, O Lord most mighty,\nO holy and most merciful Savior,\ndeliver us not into the bitter pains of eternal death.\n\nThou knowest, Lord, the secrets of our hearts;\nshut not thy merciful ears to our prayer;\nbut spare us, Lord most holy, O God most mighty,\nO holy and merciful Savior,\nthou most worthy Judge eternal.\nSuffer us not, at our last hour,\nthrough any pains of death, to fall from thee.")),
                    Document::from(Text::from("All that the Father giveth me shall come to me;\nand him that cometh to me I will in no wise cast out.\n\nHe that raised up Jesus from the dead\nwill also give life to our mortal bodies,\nby his Spirit that dwelleth in us.\n\nWherefore my heart is glad, and my spirit rejoiceth;\nmy flesh also shall rest in hope.\n\nThou shalt show me the path of life;\nin thy presence is the fullness of joy,\nand at thy right hand there is pleasure for evermore."))
                ])),
                Document::from(Rubric::from("Then, while earth is cast upon the coffin, the Celebrant says these words")),
                Document::from(Text::from("In sure and certain hope of the resurrection to eternal life through our Lord Jesus Christ, we commend to Almighty God our brother *N.*; and we commit *his* body to the ground; \\* earth to earth, ashes to ashes, dust to dust. The Lord bless *him* and keep *him*, the Lord make his face to shine upon *him* and be gracious unto *him*, the Lord lift up his countenance upon *him* and give *him* peace.").response("Amen.")),
                Document::from(Text::from("\\* *Or* the deep, *or* the elements, *or* its resting place.")),
            ])).tags([COMMITTAL]),
            Document::from(Rubric::from("The Celebrant says")).tags([COMMITTAL_PRAYERS]),
            Document::from(Preces::from([
                ("", "The Lord be with you."),
                ("People", "And with thy spirit."),
                ("Celebrant", "Let us pray.")
            ])).tags([COMMITTAL_PRAYERS]),
            Document::from(Rubric::from("Celebrant and People")).tags([COMMITTAL_PRAYERS]),
            Document::from(Text::from(LORDS_PRAYER_TRADITIONAL_TEXT).response("Amen.").display_format(DisplayFormat::Unison)).tags([COMMITTAL_LORDS_PRAYER]),
            Document::from(Rubric::from("Then the Celebrant may say")).tags([COMMITTAL_PRAYERS_2]),

            Document::from(Text::from("O Almighty God, the God of the spirits of all flesh, who by a voice from heaven didst proclaim, Blessed are the dead who die in the Lord: Multiply, we beseech thee, to those who rest in Jesus the manifold blessings of thy love, that the good work which thou didst begin in them may be made perfect unto the day of Jesus Christ. And of thy mercy, O heavenly Father, grant that we, who now serve thee on earth, may at last, together with them, be partakers of the inheritance of the saints in light; for the sake of thy Son Jesus Christ our Lord.").response("Amen.")).tags([COMMITTAL_PRAYERS_2]),
            Document::from(Rubric::from("In place of this prayer, or in addition to it, the Celebrant may use any of the Additional Prayers.")).tags([COMMITTAL_PRAYERS_2]),
            Document::from(Content::DocumentLink {
                label: "Additional Prayers".into(),
                path: SlugPath::from([Slug::Burial, Slug::AdditionalPrayers, Slug::Version(Version::RiteI)]),
                rotate: false,
                link_only: false
            }).version(Version::RiteI).tags([COMMITTAL_PRAYERS_2]),

            Document::from(Rubric::from("Then may be said.")).tags([COMMITTAL_PRAYERS_2]),
            Document::from(ResponsivePrayer::from([
                "Rest eternal grant to *him*, O Lord;",
                "And let light perpetual shine upon *him*."
            ])).tags([COMMITTAL_PRAYERS_2]),
            Document::from(Text::from("May *his* soul, and the souls of all the departed,\nthrough the mercy of God, rest in peace.").response("Amen.")).tags([COMMITTAL_PRAYERS_2]),
            Document::from(Rubric::from("The Celebrant dismisses the people with these words")).tags([DISMISSAL]),
            Document::from(Text::from("The God of peace, who brought again from the dead our Lord Jesus Christ, the great Shepherd of the sheep, through the blood of the everlasting covenant: Make you perfect in every good work to do his will, working in you that which is well pleasing in his sight; through Jesus Christ, to whom be glory for ever and ever.").response("Amen.")).tags([DISMISSAL]),

            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading3, "The Consecration of a Grave"))),
                Document::from(Rubric::from("If the grave is in a place that has not previously been set apart for Christian burial, the Priest may use the following prayer, either before the service of Committal or at some other convenient time")),
                Document::from(Text::from("O God, whose blessed Son was laid in a sepulcher in the garden: Bless, we pray, this grave, and grant that *he* whose body is (is to be) buried here may dwell with Christ in paradise, and may come to thy heavenly kingdom; through thy Son Jesus Christ our Lord.").response("Amen."))
            ])).tags([CONSECRATION_OF_A_GRAVE]),

            Document::from(Series::from(vec![
                Document::from(Heading::from((HeadingLevel::Heading2, "Additional Prayers"))),
                Document::from(Content::DocumentLink {
                    label: "Additional Prayers".into(),
                    path: SlugPath::from([Slug::Burial, Slug::AdditionalPrayers, Slug::Version(Version::RiteI)]),
                    rotate: false,
                    link_only: false
                }).version(Version::RiteI)
            ])).tags([ADDITIONAL_PRAYERS])
        ]))));

    pub static ref ADDITIONAL_PRAYERS_BURIAL_I: Vec<Document> = vec![
        Document::from(Text::from("Almighty and everlasting God, we yield unto thee most high praise and hearty thanks for the wonderful grace and virtue declared in all thy saints, who have been the choice vessels of thy grace, and the lights of the world in their several generations; most humbly beseeching thee to give us grace so to follow the example of their steadfastness in thy faith, and obedience to thy holy commandments, that at the day of the general resurrection, we, with all those who are of the mystical body of thy Son, may be set on his right hand, and hear that his most joyful voice: “Come, ye blessed of my Father, inherit the kingdom prepared for you from the foundation of the world.” Grant this, O Father, for the sake of the same thy Son Jesus Christ, our only Mediator and Advocate.").response("Amen.")).page(487),
        Document::from(Text::from("Almighty God, with whom do live the spirits of those who depart hence in the Lord, and with whom the souls of the faithful, after they are delivered from the burden of the flesh, are in joy and felicity: We give thee hearty thanks for the good examples of all those thy servants, who, having finished their course in faith, do now rest from their labors. And we beseech thee that we, with all those who are departed in the true faith of thy holy Name, may have our perfect consummation and bliss, both in body and soul, in thy eternal and everlasting glory; through Jesus Christ our Lord.").response("Amen.")).page(488),
        Document::from(Text::from("Into thy hands, O Lord, we commend thy servant *N.*, our dear *brother*, as into the hands of a faithful Creator and most merciful Savior, beseeching thee that *he* may be precious in thy sight. Wash *him*, we pray thee, in the blood of that immaculate Lamb that was slain to take away the sins of the world; that, whatsoever defilements *he* may have contracted in the midst of this earthly life being purged and done away, *he* may be presented pure and without spot before thee; through the merits of Jesus Christ thine only Son our Lord.").response("Amen.")).page(488),
        Document::from(Text::from("Remember thy servant, O Lord, according to the favor which thou bearest unto thy people; and grant that, increasing in knowledge and love of thee, *he* may go from strength to strength in the life of perfect service in thy heavenly kingdom; through Jesus Christ our Lord.").response("Amen.")).page(488),
        Document::from(Text::from("Almighty God, our heavenly Father, in whose hands are the living and the dead: We give thee thanks for all thy servants who have laid down their lives in the service of our country. Grant to them thy mercy and the light of thy presence; and give us such a lively sense of thy righteous will, that the work which thou hast begun in them may be perfected; through Jesus Christ thy Son our Lord.").response("Amen.")).page(488),
        Document::from(Text::from("O God, whose days are without end, and whose mercies cannot be numbered: Make us, we beseech thee, deeply sensible of the shortness and uncertainty of life; and let thy Holy Spirit lead us in holiness and righteousness all our days; that, when we shall have served thee in our generation, we may be gathered unto our fathers, having the testimony of a good conscience; in the communion of the Catholic Church; in the confidence of a certain faith; in the comfort of a reasonable, religious, and holy hope; in favor with thee our God; and in perfect charity with the world. All which we ask through Jesus Christ our Lord.").response("Amen.")).page(489),
        Document::from(Text::from("O God, the King of saints, we praise and magnify thy holy Name for all thy servants who have finished their course in thy faith and fear; for the blessed Virgin Mary; for the holy patriarchs, prophets, apostles, and martyrs; and for all other thy righteous servants, known to us and unknown; and we beseech thee that, encouraged by their examples, aided by their prayers, and strengthened by their fellowship, we also may be partakers of the inheritance of the saints in light through the merits of thy Son Jesus Christ our Lord.").response("Amen.")).page(489),
        Document::from(Text::from("O Lord Jesus Christ, Son of the living God, we pray thee to set thy passion, cross, and death, between thy judgment and our souls, now and in the hour of our death. Give mercy and grace to the living, pardon and rest to the dead, to thy holy Church peace and concord, and to us sinners everlasting life and glory; who with the Father and the Holy Spirit livest and reignest, one God, now and for ever.").response("Amen.")).page(489),
        Document::from(Text::from("Almighty God, Father of mercies and giver of all comfort: Deal graciously, we pray thee, with all those who mourn, that casting every care on thee, they may know the consolation of thy love; through Jesus Christ our Lord.").response("Amen.")).page(489),
    ];
}
