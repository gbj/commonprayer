use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_144: Psalm = Psalm {
        number: 144,
        citation: None,
        sections: vec![
        PsalmSection {
            reference: Reference {
              source: Source::BCP1979,
              page: 800
            },
            local_name: String::from(""),
            latin_name: String::from("Benedictus Dominus"),
            verses: vec![
                          PsalmVerse {
                    number: 1,
                    a: String::from("Blessed be the LORD my rock! *"),
                    b: String::from("who trains my hands to fight and my fingers to battle;")
                },
              PsalmVerse {
                    number: 2,
                    a: String::from("My help and my fortress, my stronghold and my deliverer, *"),
                    b: String::from("my shield in whom I trust,\nwho subdues the peoples under me.")
                },
              PsalmVerse {
                    number: 3,
                    a: String::from("O LORD, what are we that you should care for us? *"),
                    b: String::from("mere mortals that you should think of us?")
                },
              PsalmVerse {
                    number: 4,
                    a: String::from("We are like a puff of wind; *"),
                    b: String::from("our days are like a passing shadow.")
                },
              PsalmVerse {
                    number: 5,
                    a: String::from("Bow your heavens, O LORD, and come down; *"),
                    b: String::from("touch the mountains, and they shall smoke.")
                },
              PsalmVerse {
                    number: 6,
                    a: String::from("Hurl the lightning and scatter them; *"),
                    b: String::from("shoot out your arrows and rout them.")
                },
              PsalmVerse {
                    number: 7,
                    a: String::from("Stretch out your hand from on high; *"),
                    b: String::from("rescue me and deliver me from the great waters,\nfrom the hand of foreign peoples,")
                },
              PsalmVerse {
                    number: 8,
                    a: String::from("Whose mouths speak deceitfully *"),
                    b: String::from("and whose right hand is raised in falsehood.")
                },
              PsalmVerse {
                    number: 9,
                    a: String::from("O God, I will sing to you a new song; *"),
                    b: String::from("I will play to you on a ten-stringed lyre.")
                },
              PsalmVerse {
                    number: 10,
                    a: String::from("You give victory to kings *"),
                    b: String::from("and have rescued David your servant.")
                },
              PsalmVerse {
                    number: 11,
                    a: String::from("Rescue me from the hurtful sword *"),
                    b: String::from("and deliver me from the hand of foreign peoples,")
                },
              PsalmVerse {
                    number: 12,
                    a: String::from("Whose mouths speak deceitfully *"),
                    b: String::from("and whose right hand is raised in falsehood.")
                },
              PsalmVerse {
                    number: 13,
                    a: String::from("May our sons be like plants well nurtured from their youth, *"),
                    b: String::from("and our daughters like sculptured corners of a palace.")
                },
              PsalmVerse {
                    number: 14,
                    a: String::from("May our barns be filled to overflowing with all manner of crops; *"),
                    b: String::from("may the flocks in our pastures increase by thousands and tens of thousands;\nmay our cattle be fat and sleek.")
                },
              PsalmVerse {
                    number: 15,
                    a: String::from("May there be no breaching of the walls, no going into exile, *"),
                    b: String::from("no wailing in the public squares.")
                },
              PsalmVerse {
                    number: 16,
                    a: String::from("Happy are the people of whom this is so! *"),
                    b: String::from("happy are the people whose God is the LORD!")
                },
            ]
          }
      ]
    };
}
