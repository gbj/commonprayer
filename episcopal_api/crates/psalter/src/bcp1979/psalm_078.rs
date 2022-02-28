use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_78: Psalm = Psalm {
        number: 78,
        citation: None,
        sections: vec![
          PsalmSection {
            reference: Reference {
              source: Source::BCP1979,
              page: 694
            },
            local_name: String::from("Part I"),
            latin_name: String::from("Attendite, popule"),
            verses: vec![
                          PsalmVerse {
                    number: 1,
                    a: String::from("Hear my teaching, O my people; *"),
                    b: String::from("incline your ears to the words of my mouth.")
                },
              PsalmVerse {
                    number: 2,
                    a: String::from("I will open my mouth in a parable; *"),
                    b: String::from("I will declare the mysteries of ancient times.")
                },
              PsalmVerse {
                    number: 3,
                    a: String::from("That which we have heard and known,\nand what our forefathers have told us, *"),
                    b: String::from("we will not hide from their children.")
                },
              PsalmVerse {
                    number: 4,
                    a: String::from("We will recount to generations to come\nthe praiseworthy deeds and the power of the LORD, *"),
                    b: String::from("and the wonderful works he has done.")
                },
              PsalmVerse {
                    number: 5,
                    a: String::from("He gave his decrees to Jacob\nand established a law for Israel, *"),
                    b: String::from("which he commanded them to teach their children;")
                },
              PsalmVerse {
                    number: 6,
                    a: String::from("That the generations to come might know,\nand the children yet unborn; *"),
                    b: String::from("that they in their turn might tell it to their children;")
                },
              PsalmVerse {
                    number: 7,
                    a: String::from("So that they might put their trust in God, *"),
                    b: String::from("and not forget the deeds of God,\nbut keep his commandments;")
                },
              PsalmVerse {
                    number: 8,
                    a: String::from("And not be like their forefathers,\na stubborn and rebellious generation, *"),
                    b: String::from("a generation whose heart was not steadfast,\nand whose spirit was not faithful to God.")
                },
              PsalmVerse {
                    number: 9,
                    a: String::from("The people of Ephraim, armed with the bow, *"),
                    b: String::from("turned back in the day of battle;")
                },
              PsalmVerse {
                    number: 10,
                    a: String::from("They did not keep the covenant of God, *"),
                    b: String::from("and refused to walk in his law;")
                },
              PsalmVerse {
                    number: 11,
                    a: String::from("They forgot what he had done, *"),
                    b: String::from("and the wonders he had shown them.")
                },
              PsalmVerse {
                    number: 12,
                    a: String::from("He worked marvels in the sight of their forefathers, *"),
                    b: String::from("in the land of Egypt, in the field of Zoan.")
                },
              PsalmVerse {
                    number: 13,
                    a: String::from("He split open the sea and let them pass through; *"),
                    b: String::from("he made the waters stand up like walls.")
                },
              PsalmVerse {
                    number: 14,
                    a: String::from("He led them with a cloud by day, *"),
                    b: String::from("and all the night through with a glow of fire.")
                },
              PsalmVerse {
                    number: 15,
                    a: String::from("He split the hard rocks in the wilderness *"),
                    b: String::from("and gave them drink as from the great deep.")
                },
              PsalmVerse {
                    number: 16,
                    a: String::from("He brought streams out of the cliff, *"),
                    b: String::from("and the waters gushed out like rivers.")
                },
              PsalmVerse {
                    number: 17,
                    a: String::from("But they went on sinning against him, *"),
                    b: String::from("rebelling in the desert against the Most High.")
                },
              PsalmVerse {
                    number: 18,
                    a: String::from("They tested God in their hearts, *"),
                    b: String::from("demanding food for their craving.")
                },
              PsalmVerse {
                    number: 19,
                    a: String::from("They railed against God and said, *"),
                    b: String::from("“Can God set a table in the wilderness?")
                },
              PsalmVerse {
                    number: 20,
                    a: String::from("True, he struck the rock, the waters gushed out, and the gullies overflowed; *"),
                    b: String::from("but is he able to give bread\nor to provide meat for his people?”")
                },
              PsalmVerse {
                    number: 21,
                    a: String::from("When the LORD heard this, he was full of wrath; *"),
                    b: String::from("a fire was kindled against Jacob,\nand his anger mounted against Israel;")
                },
              PsalmVerse {
                    number: 22,
                    a: String::from("For they had no faith in God, *"),
                    b: String::from("nor did they put their trust in his saving power.")
                },
              PsalmVerse {
                    number: 23,
                    a: String::from("So he commanded the clouds above *"),
                    b: String::from("and opened the doors of heaven.")
                },
              PsalmVerse {
                    number: 24,
                    a: String::from("He rained down manna upon them to eat *"),
                    b: String::from("and gave them grain from heaven.")
                },
              PsalmVerse {
                    number: 25,
                    a: String::from("So mortals ate the bread of angels; *"),
                    b: String::from("he provided for them food enough.")
                },
              PsalmVerse {
                    number: 26,
                    a: String::from("He caused the east wind to blow in the heavens *"),
                    b: String::from("and led out the south wind by his might.")
                },
              PsalmVerse {
                    number: 27,
                    a: String::from("He rained down flesh upon them like dust *"),
                    b: String::from("and wingèd birds like the sand of the sea.")
                },
              PsalmVerse {
                    number: 28,
                    a: String::from("He let it fall in the midst of their camp *"),
                    b: String::from("and round about their dwellings.")
                },
              PsalmVerse {
                    number: 29,
                    a: String::from("So they ate and were well filled, *"),
                    b: String::from("for he gave them what they craved.")
                },
              PsalmVerse {
                    number: 30,
                    a: String::from("But they did not stop their craving, *"),
                    b: String::from("though the food was still in their mouths.")
                },
              PsalmVerse {
                    number: 31,
                    a: String::from("So God’s anger mounted against them; *"),
                    b: String::from("he slew their strongest men\nand laid low the youth of Israel.")
                },
              PsalmVerse {
                    number: 32,
                    a: String::from("In spite of all this, they went on sinning *"),
                    b: String::from("and had no faith in his wonderful works.")
                },
              PsalmVerse {
                    number: 33,
                    a: String::from("So he brought their days to an end like a breath *"),
                    b: String::from("and their years in sudden terror.")
                },
              PsalmVerse {
                    number: 34,
                    a: String::from("Whenever he slew them, they would seek him, *"),
                    b: String::from("and repent, and diligently search for God.")
                },
              PsalmVerse {
                    number: 35,
                    a: String::from("They would remember that God was their rock, *"),
                    b: String::from("and the Most High God their redeemer.")
                },
              PsalmVerse {
                    number: 36,
                    a: String::from("But they flattered him with their mouths *"),
                    b: String::from("and lied to him with their tongues.")
                },
              PsalmVerse {
                    number: 37,
                    a: String::from("Their heart was not steadfast toward him, *"),
                    b: String::from("and they were not faithful to his covenant.")
                },
              PsalmVerse {
                    number: 38,
                    a: String::from("But he was so merciful that he forgave their sins\nand did not destroy them; *"),
                    b: String::from("many times he held back his anger\nand did not permit his wrath to be roused.")
                },
              PsalmVerse {
                    number: 39,
                    a: String::from("For he remembered that they were but flesh, *"),
                    b: String::from("a breath that goes forth and does not return.")
                },
            ]
              },
              PsalmSection {
                reference: Reference {
                  source: Source::BCP1979,
                  page: 698
                },
                local_name: String::from("Part II"),
                latin_name: String::from("Quoties exacerbaverunt"),
                verses: vec![
                  PsalmVerse {
                        number: 40,
                        a: String::from("How often the people disobeyed him in the wilderness *"),
                        b: String::from("and offended him in the desert!")
                    },
                  PsalmVerse {
                        number: 41,
                        a: String::from("Again and again they tempted God *"),
                        b: String::from("and provoked the Holy One of Israel.")
                    },
                  PsalmVerse {
                        number: 42,
                        a: String::from("They did not remember his power *"),
                        b: String::from("in the day when he ransomed them from the enemy;")
                    },
                  PsalmVerse {
                        number: 43,
                        a: String::from("How he wrought his signs in Egypt *"),
                        b: String::from("and his omens in the field of Zoan.")
                    },
                  PsalmVerse {
                        number: 44,
                        a: String::from("He turned their rivers into blood, *"),
                        b: String::from("so that they could not drink of their streams.")
                    },
                  PsalmVerse {
                        number: 45,
                        a: String::from("He sent swarms of flies among them, which ate them up, *"),
                        b: String::from("and frogs, which destroyed them.")
                    },
                  PsalmVerse {
                        number: 46,
                        a: String::from("He gave their crops to the caterpillar, *"),
                        b: String::from("the fruit of their toil to the locust.")
                    },
                  PsalmVerse {
                        number: 47,
                        a: String::from("He killed their vines with hail *"),
                        b: String::from("and their sycamores with frost.")
                    },
                  PsalmVerse {
                        number: 48,
                        a: String::from("He delivered their cattle to hailstones *"),
                        b: String::from("and their livestock to hot thunderbolts.")
                    },
                  PsalmVerse {
                        number: 49,
                        a: String::from("He poured out upon them his blazing anger: *"),
                        b: String::from("fury, indignation, and distress,\na troop of destroying angels.")
                    },
                  PsalmVerse {
                        number: 50,
                        a: String::from("He gave full rein to his anger;\nhe did not spare their souls from death; *"),
                        b: String::from("but delivered their lives to the plague.")
                    },
                  PsalmVerse {
                        number: 51,
                        a: String::from("He struck down all the firstborn of Egypt, *"),
                        b: String::from("the flower of manhood in the dwellings of Ham.")
                    },
                  PsalmVerse {
                        number: 52,
                        a: String::from("He led out his people like sheep *"),
                        b: String::from("and guided them in the wilderness like a flock.")
                    },
                  PsalmVerse {
                        number: 53,
                        a: String::from("He led them to safety, and they were not afraid; *"),
                        b: String::from("but the sea overwhelmed their enemies.")
                    },
                  PsalmVerse {
                        number: 54,
                        a: String::from("He brought them to his holy land, *"),
                        b: String::from("the mountain his right hand had won.")
                    },
                  PsalmVerse {
                        number: 55,
                        a: String::from("He drove out the Canaanites before them\nand apportioned an inheritance to them by lot; *"),
                        b: String::from("he made the tribes of Israel to dwell in their tents.")
                    },
                  PsalmVerse {
                        number: 56,
                        a: String::from("But they tested the Most High God, and defied him, *"),
                        b: String::from("and did not keep his commandments.")
                    },
                  PsalmVerse {
                        number: 57,
                        a: String::from("They turned away and were disloyal like their fathers; *"),
                        b: String::from("they were undependable like a warped bow.")
                    },
                  PsalmVerse {
                        number: 58,
                        a: String::from("The grieved him with their hill-altars *"),
                        b: String::from("they provoked his displeasure with their idols.")
                    },
                  PsalmVerse {
                        number: 59,
                        a: String::from("When God heard this, he was angry *"),
                        b: String::from("and utterly rejected Israel.")
                    },
                  PsalmVerse {
                        number: 60,
                        a: String::from("He forsook the shrine at Shiloh, *"),
                        b: String::from("the tabernacle where he had lived among his people.")
                    },
                  PsalmVerse {
                        number: 61,
                        a: String::from("He delivered the ark into captivity, *"),
                        b: String::from("his glory into the adversary’s hand.")
                    },
                  PsalmVerse {
                        number: 62,
                        a: String::from("He gave his people to the sword *"),
                        b: String::from("and was angered against his inheritance.")
                    },
                  PsalmVerse {
                        number: 63,
                        a: String::from("The fire consumed their young men; *"),
                        b: String::from("there were no wedding songs for their maidens.")
                    },
                  PsalmVerse {
                        number: 64,
                        a: String::from("Their priests fell by the sword, *"),
                        b: String::from("and their widows made no lamentation.")
                    },
                  PsalmVerse {
                        number: 65,
                        a: String::from("Then the LORD woke as though from sleep, *"),
                        b: String::from("like a warrior refreshed with wine.")
                    },
                  PsalmVerse {
                        number: 66,
                        a: String::from("He struck his enemies on the backside *"),
                        b: String::from("and put them to perpetual shame.")
                    },
                  PsalmVerse {
                        number: 67,
                        a: String::from("He rejected the tent of Joseph *"),
                        b: String::from("and did not choose the tribe of Ephraim;")
                    },
                  PsalmVerse {
                        number: 68,
                        a: String::from("He chose instead the tribe of Judah *"),
                        b: String::from("and Mount Zion, which he loved.")
                    },
                  PsalmVerse {
                        number: 69,
                        a: String::from("He built his sanctuary like the heights of heaven, *"),
                        b: String::from("like the earth which he founded for ever.")
                    },
                  PsalmVerse {
                        number: 70,
                        a: String::from("He chose David his servant, *"),
                        b: String::from("and took him away from the sheepfolds.")
                    },
                  PsalmVerse {
                        number: 71,
                        a: String::from("He brought him from following the ewes, *"),
                        b: String::from("to be a shepherd over Jacob his people\nand over Israel his inheritance.")
                    },
                  PsalmVerse {
                        number: 72,
                        a: String::from("So he shepherded them with a faithful and true heart *"),
                        b: String::from("and guided them with the skillfulness of his hands.")
                    },
                ]
              }
        ]
    };
}
