use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_72: Psalm = Psalm {
        number: 72,
        citation: None,
        sections: vec![
          PsalmSection {
              reference: Reference {
                source: Source::BCP1979,
                page: 685
              },
              local_name: String::from("Psalm 72"),
              latin_name: String::from("Deus, judicium"),
              verses: vec![
                PsalmVerse {
                      number: 1,
                      a: String::from("Give the King your justice, O God, *"),
                      b: String::from("and your righteousness to the King’s son;")
                  },
                PsalmVerse {
                      number: 2,
                      a: String::from("That he may rule your people righteously *"),
                      b: String::from("and the poor with justice.")
                  },
                PsalmVerse {
                      number: 3,
                      a: String::from("That the mountains may bring prosperity to the people, *"),
                      b: String::from("and the little hills bring righteousness.")
                  },
                PsalmVerse {
                      number: 4,
                      a: String::from("He shall defend the needy among the people; *"),
                      b: String::from("he shall rescue the poor and crush the oppressor.")
                  },
                PsalmVerse {
                      number: 5,
                      a: String::from("He shall live as long as the sun and moon endure, *"),
                      b: String::from("from one generation to another.")
                  },
                PsalmVerse {
                      number: 6,
                      a: String::from("He shall come down like rain upon the mown field, *"),
                      b: String::from("like showers that water the earth.")
                  },
                PsalmVerse {
                      number: 7,
                      a: String::from("In his time shall the righteous flourish; *"),
                      b: String::from("there shall be abundance of peace till the moon shall be no more.")
                  },
                PsalmVerse {
                      number: 8,
                      a: String::from("He shall rule from sea to sea, *"),
                      b: String::from("and from the River to the ends of the earth.")
                  },
                PsalmVerse {
                      number: 9,
                      a: String::from("His foes shall bow down before him, *"),
                      b: String::from("and his enemies lick the dust.")
                  },
                PsalmVerse {
                      number: 10,
                      a: String::from("The kings of Tarshish and of the isles shall pay tribute, *"),
                      b: String::from("and the kings of Arabia and Saba offer gifts.")
                  },
                PsalmVerse {
                      number: 11,
                      a: String::from("All kings shall bow down before him, *"),
                      b: String::from("and all the nations do him service.")
                  },
                PsalmVerse {
                      number: 12,
                      a: String::from("For he shall deliver the poor who cries out in distress, *"),
                      b: String::from("and the oppressed who has no helper.")
                  },
                PsalmVerse {
                      number: 13,
                      a: String::from("He shall have pity on the lowly and poor; *"),
                      b: String::from("he shall preserve the lives of the needy.")
                  },
                PsalmVerse {
                      number: 14,
                      a: String::from("He shall redeem their lives from oppression and violence, *"),
                      b: String::from("and dear shall their blood be in his sight.")
                  },
                PsalmVerse {
                      number: 15,
                      a: String::from("Long may he live!\nand may there be given to him gold from Arabia; *"),
                      b: String::from("may prayer be made for him always,\n and may they bless him all the day long.")
                  },
                PsalmVerse {
                      number: 16,
                      a: String::from("May there be abundance of grain on the earth,\ngrowing thick even on the hilltops; *"),
                      b: String::from("may its fruit flourish like Lebanon,\n and its grain like grass upon the earth.")
                  },
                PsalmVerse {
                      number: 17,
                      a: String::from("May his Name remain for ever\nand be established as long as the sun endures; *"),
                      b: String::from("may all the nations bless themselves in him and call him blessed.")
                  },
                PsalmVerse {
                      number: 18,
                      a: String::from("Blessed be the Lord GOD, the God of Israel, *"),
                      b: String::from("who alone does wondrous deeds!")
                  },
                PsalmVerse {
                      number: 19,
                      a: String::from("And blessed be his glorious Name for ever! *"),
                      b: String::from("and may all the earth be filled with his glory. Amen. Amen.")
                  },
              ]
            }
        ]
    };
}
