use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_18: Psalm = Psalm {
        number: 18,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 602
            },
            local_name: String::from("Part I"),
            latin_name: String::from("Diligam te, Domine."),
            verses: vec![
                          PsalmVerse {
                    number: 1,
                    a: String::from("I love you, O LORD my strength, *"),
                    b: String::from("O LORD my stronghold, my crag, and my haven.")
                },
              PsalmVerse {
                    number: 2,
                    a: String::from("My God, my rock in whom I put my trust, *"),
                    b: String::from("my shield, the horn of my salvation, and my refuge;\nyou are worthy of praise.")
                },
              PsalmVerse {
                    number: 3,
                    a: String::from("I will call upon the LORD, *"),
                    b: String::from("and so shall I be saved from my enemies.")
                },
              PsalmVerse {
                    number: 4,
                    a: String::from("The breakers of death rolled over me, *"),
                    b: String::from("and the torrents of oblivion made me afraid.")
                },
              PsalmVerse {
                    number: 5,
                    a: String::from("The cords of hell entangled me, *"),
                    b: String::from("and the snares of death were set for me.")
                },
              PsalmVerse {
                    number: 6,
                    a: String::from("I called upon the LORD in my distress *"),
                    b: String::from("and cried out to my God for help.")
                },
              PsalmVerse {
                    number: 7,
                    a: String::from("He heard my voice from his heavenly dwelling; *"),
                    b: String::from("my cry of anguish came to his ears.")
                },
              PsalmVerse {
                    number: 8,
                    a: String::from("The earth reeled and rocked; *"),
                    b: String::from("the roots of the mountains shook;\nthey reeled because of his anger.")
                },
              PsalmVerse {
                    number: 9,
                    a: String::from("Smoke rose from his nostrils\nand a consuming fire out of his mouth; *"),
                    b: String::from("hot burning coals blazed forth from him.")
                },
              PsalmVerse {
                    number: 10,
                    a: String::from("He parted the heavens and came down *"),
                    b: String::from("with a storm cloud under his feet.")
                },
              PsalmVerse {
                    number: 11,
                    a: String::from("He mounted on cherubim and flew; *"),
                    b: String::from("he swooped on the wings of the wind.")
                },
              PsalmVerse {
                    number: 12,
                    a: String::from("He wrapped darkness about him; *"),
                    b: String::from("he made dark waters and thick clouds his pavilion.")
                },
              PsalmVerse {
                    number: 13,
                    a: String::from("From the brightness of his presence, through the clouds, *"),
                    b: String::from("burst hailstones and coals of fire.")
                },
              PsalmVerse {
                    number: 14,
                    a: String::from("The LORD thundered out of heaven; *"),
                    b: String::from("the Most High uttered his voice.")
                },
              PsalmVerse {
                    number: 15,
                    a: String::from("He loosed his arrows and scattered them; *"),
                    b: String::from("he hurled thunderbolts and routed them.")
                },
              PsalmVerse {
                    number: 16,
                    a: String::from("The beds of the seas were uncovered,\nand the foundations of the world laid bare, *"),
                    b: String::from("at your battle cry, O LORD,\nat the blast of the breath of your nostrils.")
                },
              PsalmVerse {
                    number: 17,
                    a: String::from("He reached down from on high and grasped me; *"),
                    b: String::from("he drew me out of great waters.")
                },
              PsalmVerse {
                    number: 18,
                    a: String::from("He delivered me from my strong enemies\nand from those who hated me; *"),
                    b: String::from("for they were too mighty for me.")
                },
              PsalmVerse {
                    number: 19,
                    a: String::from("They confronted me in the day of my disaster; *"),
                    b: String::from("but the LORD was my support.")
                },
              PsalmVerse {
                    number: 20,
                    a: String::from("He brought me out into an open place; *"),
                    b: String::from("he rescued me because he delighted in me.")
                },
            ]
        },
        PsalmSection {
          reference: Reference {
            source: Source::BCP1979,
            page: 604
          },
          local_name: String::from("Part II"),
          latin_name: String::from("Et retribuet mihi"),
          verses: vec![
                        PsalmVerse {
                  number: 21,
                  a: String::from("The LORD rewarded me because of my righteous dealing; *"),
                  b: String::from("because my hands were clean he rewarded me;")
              },
            PsalmVerse {
                  number: 22,
                  a: String::from("For I have kept the ways of the LORD *"),
                  b: String::from("and have not offended against my God;")
              },
            PsalmVerse {
                  number: 23,
                  a: String::from("For all his judgments are before my eyes, *"),
                  b: String::from("and his decrees I have not put away from me;")
              },
            PsalmVerse {
                  number: 24,
                  a: String::from("For I have been blameless with him *"),
                  b: String::from("and have kept myself from iniquity;")
              },
            PsalmVerse {
                  number: 25,
                  a: String::from("Therefore the LORD rewarded me according to my righteous dealing, *"),
                  b: String::from("because of the cleanness of my hands in his sight.")
              },
            PsalmVerse {
                  number: 26,
                  a: String::from("With the faithful you show yourself faithful, O God; *"),
                  b: String::from("with the forthright you show yourself forthright.")
              },
            PsalmVerse {
                  number: 27,
                  a: String::from("With the pure you show yourself pure, *"),
                  b: String::from("but with the crooked you are wily.")
              },
            PsalmVerse {
                  number: 28,
                  a: String::from("You will save a lowly people, *"),
                  b: String::from("but you will humble the haughty eyes.")
              },
            PsalmVerse {
                  number: 29,
                  a: String::from("You, O LORD, are my lamp; *"),
                  b: String::from("my God, you make my darkness bright.")
              },
            PsalmVerse {
                  number: 30,
                  a: String::from("With you I will break down an enclosure; *"),
                  b: String::from("with the help of my God I will scale any wall.")
              },
            PsalmVerse {
                  number: 31,
                  a: String::from("As for God, his ways are perfect;\nthe words of the LORD are tried in the fire; *"),
                  b: String::from("he is a shield to all who trust in him.")
              },
            PsalmVerse {
                  number: 32,
                  a: String::from("For who is God, but the LORD? *"),
                  b: String::from("who is the Rock, except our God?")
              },
            PsalmVerse {
                  number: 33,
                  a: String::from("It is God who girds me about with strength *"),
                  b: String::from("and makes my way secure.")
              },
            PsalmVerse {
                  number: 34,
                  a: String::from("He makes me sure-footed like a deer *"),
                  b: String::from("and lets me stand firm on the heights.")
              },
            PsalmVerse {
                  number: 35,
                  a: String::from("He trains my hands for battle *"),
                  b: String::from("and my arms for bending even a bow of bronze.")
              },
            PsalmVerse {
                  number: 36,
                  a: String::from("You have given me your shield of victory; *"),
                  b: String::from("your right hand also sustains me;\nyour loving care makes me great.")
              },
            PsalmVerse {
                  number: 37,
                  a: String::from("You lengthen my stride beneath me, *"),
                  b: String::from("and my ankles do not give way.")
              },
            PsalmVerse {
                  number: 38,
                  a: String::from("I pursue my enemies and overtake them; *"),
                  b: String::from("I will not turn back till I have destroyed them.")
              },
            PsalmVerse {
                  number: 39,
                  a: String::from("I strike them down, and they cannot rise; *"),
                  b: String::from("they fall defeated at my feet.")
              },
            PsalmVerse {
                  number: 40,
                  a: String::from("You have girded me with strength for the battle; *"),
                  b: String::from("you have cast down my adversaries beneath me;\nyou have put my enemies to flight.")
              },
            PsalmVerse {
                  number: 41,
                  a: String::from("I destroy those who hate me;\nthey cry out, but there is none to help them; *"),
                  b: String::from("they cry to the LORD, but he does not answer.")
              },
            PsalmVerse {
                  number: 42,
                  a: String::from("I beat them small like dust before the wind; *"),
                  b: String::from("I trample them like mud in the streets.")
              },
            PsalmVerse {
                  number: 43,
                  a: String::from("You deliver me from the strife of the peoples; *"),
                  b: String::from("you put me at the head of the nations.")
              },
            PsalmVerse {
                  number: 44,
                  a: String::from("A people I have not known shall serve me;\nno sooner shall they hear than they shall obey me; *"),
                  b: String::from("strangers will cringe before me.")
              },
            PsalmVerse {
                  number: 45,
                  a: String::from("The foreign peoples will lose heart; *"),
                  b: String::from("they shall come trembling out of their strongholds.")
              },
            PsalmVerse {
                  number: 46,
                  a: String::from("The LORD lives!  Blessed is my Rock! *"),
                  b: String::from("Exalted is the God of my salvation!")
              },
            PsalmVerse {
                  number: 47,
                  a: String::from("He is the God who gave me victory *"),
                  b: String::from("and cast down the peoples beneath me.")
              },
            PsalmVerse {
                  number: 48,
                  a: String::from("You rescued me from the fury of my enemies;\nyou exalted me above those who rose against me; *"),
                  b: String::from("you saved me from my deadly foe.")
              },
            PsalmVerse {
                  number: 49,
                  a: String::from("Therefore will I extol you among the nations, O LORD, *"),
                  b: String::from("and sing praises to your Name.")
              },
            PsalmVerse {
                  number: 50,
                  a: String::from("He multiplies the victories of his king; *"),
                  b: String::from("he shows loving-kindness to his anointed,\nto David and his descendants for ever.")
              },
          ]
        }]
     };
}
