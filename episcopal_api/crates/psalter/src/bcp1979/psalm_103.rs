use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_103: Psalm = Psalm {
        number: 103,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 733
              },
              local_name: String::from(""),
              latin_name: String::from("Benedic, anima mea"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Bless the LORD, O my soul, *"),
                      b: String::from("and all that is within me, bless his holy Name.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("Bless the LORD, O my soul, *"),
                      b: String::from("and forget not all his benefits.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("He forgives all your sins *"),
                      b: String::from("and heals all your infirmities;")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("He redeems your life from the grave *"),
                      b: String::from("and crowns you with mercy and loving-kindness;")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("He satisfies you with good things, *"),
                      b: String::from("and your youth is renewed like an eagle’s.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("The LORD executes righteousness *"),
                      b: String::from("and judgment for all who are oppressed.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("He made his ways known to Moses *"),
                      b: String::from("and his works to the children of Israel.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("The LORD is full of compassion and mercy, *"),
                      b: String::from("slow to anger and of great kindness.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("He will not always accuse us, *"),
                      b: String::from("nor will he keep his anger for ever.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("He has not dealt with us according to our sins, *"),
                      b: String::from("nor rewarded us according to our wickedness.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("For as the heavens are high above the earth, *"),
                      b: String::from("so is his mercy great upon those who fear him.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("As far as the east is from the west, *"),
                      b: String::from("so far has he removed our sins from us.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("As a father cares for his children, *"),
                      b: String::from("so does the LORD care for those who fear him.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("For he himself knows whereof we are made; *"),
                      b: String::from("he remembers that we are but dust.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("Our days are like the grass; *"),
                      b: String::from("we flourish like a flower of the field;")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("When the wind goes over it, it is gone, *"),
                      b: String::from("and its place shall know it no more.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("But the merciful goodness of the LORD endures for ever on those who fear him, *"),
                      b: String::from("and his righteousness on children’s children;")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("On those who keep his covenant *"),
                      b: String::from("and remember his commandments and do them.")
                  },
                PsalmVerse {
                      number: 19,
                      a: String::from("The LORD has set his throne in heaven, *"),
                      b: String::from("and his kingship has dominion over all.")
                  },
                PsalmVerse {
                      number: 20,
                      a: String::from("Bless the LORD, you angels of his,\nyou mighty ones who do his bidding, *"),
                      b: String::from("and hearken to the voice of his word.")
                  },
                PsalmVerse {
                      number: 21,
                      a: String::from("Bless the LORD, all you his hosts, *"),
                      b: String::from("you ministers of his who do his will.")
                  },
                PsalmVerse {
                      number: 22,
                      a: String::from("Bless the LORD, all you works of his,\nin all places of his dominion; *"),
                      b: String::from("bless the LORD, O my soul.")
                  },
              ]
            }
        ]
    };
}
