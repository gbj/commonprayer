use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_66: Psalm = Psalm {
        number: 66,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 673
              },
              local_name: String::from("Psalm 66"),
              latin_name: String::from("Jubilate Deo"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Be joyful in God, all you lands; *"),
                      b: String::from("sing the glory of his Name;\n sing the glory of his praise.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Say to God, “How awesome are your deeds! *"),
                      b: String::from("because of your great strength your enemies cringe before you.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("All the earth bows down before you, *"),
                      b: String::from("sings to you, sings out your Name.”")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Come now and see the works of God, *"),
                      b: String::from("how wonderful he is in his doing toward all people.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("He turned the sea into dry land,\nso that they went through the water on foot, *"),
                      b: String::from("and there we rejoiced in him.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("In his might he rules for ever;\nhis eyes keep watch over the nations; *"),
                      b: String::from("let no rebel rise up against him.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("Bless our God, you peoples; *"),
                      b: String::from("make the voice of his praise to be heard;")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Who holds our souls in life, *"),
                      b: String::from("and will not allow our feet to slip.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("For you, O God, have proved us; *"),
                      b: String::from("you have tried us just as silver is tried.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("You brought us into the snare; *"),
                      b: String::from("you laid heavy burdens upon our backs.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("You let enemies ride over our heads;\nwe went through fire and water; *"),
                      b: String::from("but you brought us out into a place of refreshment.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("I will enter your house with burnt-offerings\nand will pay you my vows, *"),
                      b: String::from("which I promised with my lips\n and spoke with my mouth when I was in trouble.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("I will offer you sacrifices of fat beasts\nwith the smoke of rams; *"),
                      b: String::from("I will give you oxen and goats.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("Come and listen, all you who fear God, *"),
                      b: String::from("and I will tell you what he has done for me.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("I called out to him with my mouth, *"),
                      b: String::from("and his praise was on my tongue.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("If I had found evil in my heart, *"),
                      b: String::from("the Lord would not have heard me;")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("But in truth God has heard me; *"),
                      b: String::from("he has attended to the voice of my prayer.")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("Blessed be God, who has not rejected my prayer, *"),
                      b: String::from("nor withheld his love from me.")
                  },
              ]
            }
        ]
    };
}
