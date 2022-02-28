use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_84: Psalm = Psalm {
        number: 84,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 707
              },
              local_name: String::from(""),
              latin_name: String::from("Quam dilecta!"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("How dear to me is your dwelling, O LORD of hosts! *"),
                      b: String::from("My soul has a desire and longing for the courts of the LORD;\nmy heart and my flesh rejoice in the living God.")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("The sparrow has found her a house\nand the swallow a nest where she may lay her young; *"),
                      b: String::from("by the side of your altars, O LORD of hosts,\nmy King and my God.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("Happy are they who dwell in your house! *"),
                      b: String::from("they will always be praising you.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("Happy are the people whose strength is in you! *"),
                      b: String::from("whose hearts are set on the pilgrims’ way.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("Those who go through the desolate valley will find it a place of springs, *"),
                      b: String::from("for the early rains have covered it with pools of water.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("They will climb from height to height, *"),
                      b: String::from("and the God of gods will reveal himself in Zion.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("LORD God of hosts, hear my prayer; *"),
                      b: String::from("hearken, O God of Jacob.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("Behold our defender, O God; *"),
                      b: String::from("and look upon the face of your Anointed.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("For one day in your courts is better than a thousand in my own room, *"),
                      b: String::from("and to stand at the threshold of the house of my God\nthan to dwell in the tents of the wicked.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("For the LORD God is both sun and shield; *"),
                      b: String::from("he will give grace and glory;")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("No good thing will the LORD withhold *"),
                      b: String::from("from those who walk with integrity.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("O LORD of hosts, *"),
                      b: String::from("happy are they who put their trust in you!")
                  },
              ]
            }
        ]
    };
}
