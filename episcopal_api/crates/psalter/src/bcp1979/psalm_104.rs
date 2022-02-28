use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_104: Psalm = Psalm {
        number: 104,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 735
              },
              local_name: String::from(""),
              latin_name: String::from("Benedic, anima mea"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Bless the LORD, O my soul; *"),
                      b: String::from("O LORD my God, how excellent is your greatness!\nyou are clothed with majesty and splendor.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("You wrap yourself with light as with a cloak *"),
                      b: String::from("and spread out the heavens like a curtain.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("You lay the beams of your chambers in the waters above; *"),
                      b: String::from("you make the clouds your chariot;\nyou ride on the wings of the wind.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("You make the winds your messengers *"),
                      b: String::from("and flames of fire your servants.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("You have set the earth upon its foundations, *"),
                      b: String::from("so that it never shall move at any time.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("You covered it with the Deep as with a mantle; *"),
                      b: String::from("the waters stood higher than the mountains.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("At your rebuke they fled; *"),
                      b: String::from("at the voice of your thunder they hastened away.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("They went up into the hills and down to the valleys beneath, *"),
                      b: String::from("to the places you had appointed for them.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("You set the limits that they should not pass; *"),
                      b: String::from("they shall not again cover the earth.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("You send the springs into the valleys; *"),
                      b: String::from("they flow between the mountains.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("All the beasts of the field drink their fill from them, *"),
                      b: String::from("and the wild asses quench their thirst.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("Beside them the birds of the air make their nests *"),
                      b: String::from("and sing among the branches.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("You water the mountains from your dwelling on high; *"),
                      b: String::from("the earth is fully satisfied by the fruit of your works.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("You make grass grow for flocks and herds *"),
                      b: String::from("and plants to serve mankind;")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("That they may bring forth food from the earth, *"),
                      b: String::from("and wine to gladden our hearts,")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("Oil to make a cheerful countenance, *"),
                      b: String::from("and bread to strengthen the heart.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("The trees of the LORD are full of sap, *"),
                      b: String::from("the cedars of Lebanon which he planted,")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("In which the birds build their nests, *"),
                      b: String::from("and in whose tops the stork makes his dwelling.")
                  },
                PsalmVerse {
                      number: 19,
                      a: String::from("The high hills are a refuge for the mountain goats, *"),
                      b: String::from("and the stony cliffs for the rock badgers.")
                  },
                PsalmVerse {
                      number: 20,
                      a: String::from("You appointed the moon to mark the seasons, *"),
                      b: String::from("and the sun knows the time of its setting.")
                  },
                PsalmVerse {
                      number: 21,
                      a: String::from("You make darkness that it may be night, *"),
                      b: String::from("in which all the beasts of the forest prowl.")
                  },
                PsalmVerse {
                      number: 22,
                      a: String::from("The lions roar after their prey *"),
                      b: String::from("and seek their food from God.")
                  },
                PsalmVerse {
                      number: 23,
                      a: String::from("The sun rises, and they slip away *"),
                      b: String::from("and lay themselves down in their dens.")
                  },
                PsalmVerse {
                      number: 24,
                      a: String::from("Man goes forth to his work *"),
                      b: String::from("and to his labor until the evening.")
                  },
                PsalmVerse {
                      number: 25,
                      a: String::from("O LORD, how manifold are your works! *"),
                      b: String::from("in wisdom you have made them all;\nthe earth is full of your creatures.")
                  },
                PsalmVerse {
                      number: 26,
                      a: String::from("Yonder is the great and wide sea\nwith its living things too many to number, *"),
                      b: String::from("creatures both small and great.")
                  },
                PsalmVerse {
                      number: 27,
                      a: String::from("There move the ships,\nand there is that Leviathan, *"),
                      b: String::from("which you have made for the sport of it.")
                  },
                PsalmVerse {
                      number: 28,
                      a: String::from("All of them look to you *"),
                      b: String::from("to give them their food in due season.")
                  },
                PsalmVerse {
                      number: 29,
                      a: String::from("You give it to them; they gather it; *"),
                      b: String::from("you open your hand, and they are filled with good things.")
                  },
                PsalmVerse {
                      number: 30,
                      a: String::from("You hide your face, and they are terrified; *"),
                      b: String::from("you take away their breath,\nand they die and return to their dust.")
                  },
                PsalmVerse {
                      number: 31,
                      a: String::from("You send forth your Spirit, and they are created; *"),
                      b: String::from("and so you renew the face of the earth.")
                  },
                PsalmVerse {
                      number: 32,
                      a: String::from("May the glory of the LORD endure for ever; *"),
                      b: String::from("may the LORD rejoice in all his works.")
                  },
                PsalmVerse {
                      number: 33,
                      a: String::from("He looks at the earth and it trembles; *"),
                      b: String::from("he touches the mountains and they smoke.")
                  },
                PsalmVerse {
                      number: 34,
                      a: String::from("I will sing to the LORD as long as I live; *"),
                      b: String::from("I will praise my God while I have my being.")
                  },
                PsalmVerse {
                      number: 35,
                      a: String::from("May these words of mine please him; *"),
                      b: String::from("I will rejoice in the LORD.")
                  },
                PsalmVerse {
                      number: 36,
                      a: String::from("Let sinners be consumed out of the earth, *"),
                      b: String::from("and the wicked be no more.")
                  },
                PsalmVerse {
                      number: 37,
                      a: String::from("Bless the LORD, O my soul. *"),
                      b: String::from("Hallelujah!")
                  },
              ]
            }
        ]
    };
}
