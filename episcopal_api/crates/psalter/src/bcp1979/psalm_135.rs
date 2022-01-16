use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_135: Psalm = Psalm {
        number: 135,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 788
              },
              local_name: String::from("Psalm 135"),
              latin_name: String::from("Laudate nomen"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Hallelujah!\nPraise the Name of the LORD; *"),
                      b: String::from("give praise, you servants of the LORD.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("You who stand in the house of the LORD, *"),
                      b: String::from("in the courts of the house of our God.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Praise the LORD, for the LORD is good; *"),
                      b: String::from("sing praises to his Name, for it is lovely.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("For the LORD has chosen Jacob for himself *"),
                      b: String::from("and Israel for his own possession.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("For I know that the LORD is great, *"),
                      b: String::from("and that our Lord is above all gods.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("The LORD does whatever pleases him, in heaven and on earth, *"),
                      b: String::from("in the seas and all the deeps.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("He brings up rain clouds from the ends of the earth; *"),
                      b: String::from("he sends out lightning with the rain,\n and brings the winds out of his storehouse.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("It was he who struck down the firstborn of Egypt, *"),
                      b: String::from("the firstborn both of man and beast.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("He sent signs and wonders into the midst of you, O Egypt, *"),
                      b: String::from("against Pharaoh and all his servants.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("He overthrew many nations *"),
                      b: String::from("and put mighty kings to death:")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("Sihon, king of the Amorites,\nand Og, the kingdoms of Bashan, *"),
                      b: String::from("and all the kings of Canaan.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("He gave their land to be an inheritance, *"),
                      b: String::from("an inheritance for Israel his people.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("O LORD, your Name is everlasting; *"),
                      b: String::from("your renown, O LORD, endures from age to age.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("For the LORD gives his people justice *"),
                      b: String::from("and shows compassion to his servants.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("The idols of the heathen are silver and gold, *"),
                      b: String::from("the work of human hands.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("They have mouths, but they cannot speak; *"),
                      b: String::from("eyes have they, but they cannot see.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("They have ears, but they cannot hear; *"),
                      b: String::from("neither is there any breath in their mouth.")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("Those who make them are like them, *"),
                      b: String::from("and so are all who put their trust in them.")
                  },
                PsalmVerse {
                      number: 19,
                      a: String::from("Bless the LORD, O house of Israel; *"),
                      b: String::from("O house of Aaron, bless the LORD.")
                  },
                PsalmVerse {
                      number: 20,
                      a: String::from("Bless the LORD, O house of Levi; *"),
                      b: String::from("you who fear the LORD, bless the LORD.")
                  },
                PsalmVerse {
                      number: 21,
                      a: String::from("Blessed be the LORD out of Zion, *"),
                      b: String::from("who dwells in Jerusalem.\n Hallelujah!")
                  },
              ]
            }
        ]
    };
}
