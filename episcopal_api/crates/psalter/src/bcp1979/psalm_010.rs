use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_10: Psalm = Psalm {
        number: 10,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 594
            },
            local_name: String::from("Psalm 10"),
            latin_name: String::from("Ut quid, Domine?"),
            verses: vec![
              PsalmVerse {
        number: 1,
        a: String::from("Why do you stand so far off, O LORD, *"),
        b: String::from("and hide yourself in time of trouble?")
    },
  PsalmVerse {
        number: 2,
        a: String::from("The wicked arrogantly persecute the poor, *"),
        b: String::from("but they are trapped in the schemes they have devised.")
    },
  PsalmVerse {
        number: 3,
        a: String::from("The wicked boast of their heart’s desire; *"),
        b: String::from("the covetous curse and revile the LORD.")
    },
  PsalmVerse {
        number: 4,
        a: String::from("The wicked are so proud that they care not for God; *"),
        b: String::from("their only thought is, “God does not matter.”")
    },
  PsalmVerse {
        number: 5,
        a: String::from("Their ways are devious at all times;\nyour judgments are far above out of their sight; *"),
        b: String::from("they defy all their enemies.")
    },
  PsalmVerse {
        number: 6,
        a: String::from("They say in their heart, “I shall not be shaken; *"),
        b: String::from("no harm shall happen to me ever.”")
    },
  PsalmVerse {
        number: 7,
        a: String::from("Their mouth is full of cursing, deceit, and oppression; *"),
        b: String::from("under their tongue are mischief and wrong.")
    },
  PsalmVerse {
        number: 8,
        a: String::from("They lurk in ambush in public squares\nand in secret places they murder the innocent; *"),
        b: String::from("they spy out the helpless.")
    },
  PsalmVerse {
        number: 9,
        a: String::from("They lie in wait, like a lion in a covert;\nthey lie in wait to seize upon the lowly; *"),
        b: String::from("they seize the lowly and drag them away in their net.")
    },
  PsalmVerse {
        number: 10,
        a: String::from("The innocent are broken and humbled before them; *"),
        b: String::from("the helpless fall before their power.")
    },
  PsalmVerse {
        number: 11,
        a: String::from("They say in their heart, “God has forgotten; *"),
        b: String::from("he hides his face; he will never notice.”")
    },
  PsalmVerse {
        number: 12,
        a: String::from("Rise up, O LORD;\nlift up your hand, O God; *"),
        b: String::from("do not forget the afflicted.")
    },
  PsalmVerse {
        number: 13,
        a: String::from("Why should the wicked revile God? *"),
        b: String::from("why should they say in their heart, “You do not care”?")
    },
  PsalmVerse {
        number: 14,
        a: String::from("Surely, you behold trouble and misery; *"),
        b: String::from("you see it and take it into your own hand.")
    },
  PsalmVerse {
        number: 15,
        a: String::from("The helpless commit themselves to you, *"),
        b: String::from("for you are the helper of orphans.")
    },
  PsalmVerse {
        number: 16,
        a: String::from("Break the power of the wicked and evil; *"),
        b: String::from("search out their wickedness until you find none.")
    },
  PsalmVerse {
        number: 17,
        a: String::from("The LORD is King for ever and ever; *"),
        b: String::from("the ungodly shall perish from his land.")
    },
  PsalmVerse {
        number: 18,
        a: String::from("The LORD will hear the desire of the humble; *"),
        b: String::from("you will strengthen their heart and your ears shall hear;")
    },
  PsalmVerse {
        number: 19,
        a: String::from("To give justice to the orphan and oppressed, *"),
        b: String::from("so that mere mortals may strike terror no more.")
    },
]
        }]
    };
}
