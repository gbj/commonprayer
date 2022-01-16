use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_19: Psalm = Psalm {
        number: 19,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 606
              },
              local_name: String::from("Psalm 19"),
              latin_name: String::from("Caeli enarrant"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("The heavens declare the glory of God, *"),
                      b: String::from("and the firmament shows his handiwork.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("One day tells its tale to another, *"),
                      b: String::from("and one night imparts knowledge to another.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Although they have no words or language, *"),
                      b: String::from("and their voices are not heard,")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Their sound has gone out into all lands, *"),
                      b: String::from("and their message to the ends of the world.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("In the deep has he set a pavilion for the sun; *"),
                      b: String::from("it comes forth like a bridegroom out of his chamber;\n it rejoices like a champion to run its course.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("It goes forth from the uttermost edge of the heavens\nand runs about to the end of it again; *"),
                      b: String::from("nothing is hidden from its burning heat.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("The law of the LORD is perfect and revives the soul; *"),
                      b: String::from("the testimony of the LORD is sure and gives wisdom to the innocent.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("The statutes of the LORD are just and rejoice the heart; *"),
                      b: String::from("the commandment of the LORD is clear and gives light to the eyes.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("The fear of the LORD is clean and endures for ever; *"),
                      b: String::from("the judgments of the LORD are true and righteous altogether.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("More to be desired are they than gold, more than much fine gold, *"),
                      b: String::from("sweeter far than honey, than honey in the comb.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("By them also is your servant enlightened, *"),
                      b: String::from("and in keeping them there is great reward.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Who can tell how often he offends? *"),
                      b: String::from("cleanse me from my secret faults.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("Above all, keep your servant from presumptuous sins;\nlet them not get dominion over me; *"),
                      b: String::from("then shall I be whole and sound,\n and innocent of a great offense.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("Let the words of my mouth and the meditation of my heart be acceptable in your sight, *"),
                      b: String::from("O LORD, my strength and my redeemer.")
                  },
              ]
            }
        ]
    };
}
