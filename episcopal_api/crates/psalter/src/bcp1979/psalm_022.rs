use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_22: Psalm = Psalm {
        number: 22,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 610
              },
              local_name: String::from("Psalm 22"),
              latin_name: String::from("Deus, Deus meus"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("My God, my God, why have you forsaken me? *"),
                      b: String::from("and are so far from my cry\n and from the words of my distress?")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("O my God, I cry in the daytime, but you do not answer; *"),
                      b: String::from("by night as well, but I find no rest.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Yet you are the Holy One, *"),
                      b: String::from("enthroned upon the praises of Israel.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Our forefathers put their trust in you; *"),
                      b: String::from("they trusted, and you delivered them.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("They cried out to you and were delivered; *"),
                      b: String::from("they trusted in you and were not put to shame.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("But as for me, I am a worm and no man, *"),
                      b: String::from("scorned by all and despised by the people.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("All who see me laugh me to scorn; *"),
                      b: String::from("they curl their lips and wag their heads, saying,")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("“He trusted in the LORD; let him deliver him; *"),
                      b: String::from("let him rescue him, if he delights in him.”")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("Yet you are he who took me out of the womb, *"),
                      b: String::from("and kept me safe upon my mother’s breast.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("I have been entrusted to you ever since I was born; *"),
                      b: String::from("you were my God when I was still in my mother’s womb.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Be not far from me, for trouble is near, *"),
                      b: String::from("and there is none to help.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Many young bulls encircle me; *"),
                      b: String::from("strong bulls of Bashan surround me.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("They open wide their jaws at me, *"),
                      b: String::from("like a ravening and a roaring lion.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("I am poured out like water;\nall my bones are out of joint; *"),
                      b: String::from("my heart within my breast is melting wax.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("My mouth is dried out like a pot-sherd;\nmy tongue sticks to the roof of my mouth; *"),
                      b: String::from("and you have laid me in the dust of the grave.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("Packs of dogs close me in,\nand gangs of evildoers circle around me; *"),
                      b: String::from("they pierce my hands and my feet;\n I can count all my bones.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("They stare and gloat over me; *"),
                      b: String::from("they divide my garments among them;\n they cast lots for my clothing.")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("Be not far away, O LORD; *"),
                      b: String::from("you are my strength; hasten to help me.")
                  },
                PsalmVerse {
                      number: 19,
                      a: String::from("Save me from the sword, *"),
                      b: String::from("my life from the power of the dog.")
                  },
                PsalmVerse {
                      number: 20,
                      a: String::from("Save me from the lion’s mouth, *"),
                      b: String::from("my wretched body from the horns of wild bulls.")
                  },
                PsalmVerse {
                      number: 21,
                      a: String::from("I will declare your Name to my brethren; *"),
                      b: String::from("in the midst of the congregation I will praise you.")
                  },
                PsalmVerse {
                      number: 22,
                      a: String::from("Praise the LORD, you that fear him; *"),
                      b: String::from("stand in awe of him, O offspring of Israel;\n all you of Jacob’s line, give glory.")
                  },
                PsalmVerse {
                      number: 23,
                      a: String::from("For he does not despise nor abhor the poor in their poverty;\nneither does he hide his face from them; *"),
                      b: String::from("but when they cry to him he hears them.")
                  },
                PsalmVerse {
                      number: 24,
                      a: String::from("My praise is of him in the great assembly; *"),
                      b: String::from("I will perform my vows in the presence of those who worship him.")
                  },
                PsalmVerse {
                      number: 25,
                      a: String::from("The poor shall eat and be satisfied,\nand those who seek the LORD shall praise him: *"),
                      b: String::from("“May your heart live for ever!”")
                  },
                PsalmVerse {
                      number: 26,
                      a: String::from("All the ends of the earth shall remember and turn to the LORD, *"),
                      b: String::from("and all the families of the nations bow before him.")
                  },
                PsalmVerse {
                      number: 27,
                      a: String::from("For kingship belongs to the LORD; *"),
                      b: String::from("he rules over the nations.")
                  },
                PsalmVerse {
                      number: 28,
                      a: String::from("To him alone all who sleep in the earth bow down in worship; *"),
                      b: String::from("all who go down to the dust fall before him.")
                  },
                PsalmVerse {
                      number: 29,
                      a: String::from("My soul shall live for him;\nmy descendants shall serve him; *"),
                      b: String::from("they shall be known as the LORD’S for ever.")
                  },
                PsalmVerse {
                      number: 30,
                      a: String::from("They shall come and make known to a people yet unborn *"),
                      b: String::from("the saving deeds that he has done.")
                  },
              ]
            }
        ]
    };
}
