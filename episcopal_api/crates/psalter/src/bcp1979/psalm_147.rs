use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_147: Psalm = Psalm {
        number: 147,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 804
            },
            local_name: String::from("Psalm 147"),
            latin_name: String::from("Laudate Dominum"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("Hallelujah!\nHow good it is to sing praises to our God! *"),
                    b: String::from("how pleasant it is to honor him with praise!")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("The LORD rebuilds Jerusalem; *"),
                    b: String::from("he gathers the exiles of Israel.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("He heals the brokenhearted *"),
                    b: String::from("and binds up their wounds.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("He counts the number of the stars *"),
                    b: String::from("and calls them all by their names.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("Great is our LORD and mighty in power; *"),
                    b: String::from("there is no limit to his wisdom.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("The LORD lifts up the lowly, *"),
                    b: String::from("but casts the wicked to the ground.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("Sing to the LORD with thanksgiving; *"),
                    b: String::from("make music to our God upon the harp.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("He covers the heavens with clouds *"),
                    b: String::from("and prepares rain for the earth;")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("He makes grass to grow upon the mountains *"),
                    b: String::from("and green plants to serve mankind.")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("He provides food for flocks and herds *"),
                    b: String::from("and for the young ravens when they cry.")
                },
                PsalmVerse {
                    number: 11,
                    a: String::from("He is not impressed by the might of a horse; *"),
                    b: String::from("he has no pleasure in the strength of a man;")
                },
                PsalmVerse {
                    number: 12,
                    a: String::from("But the LORD has pleasure in those who fear him, *"),
                    b: String::from("in those who await his gracious favor.")
                },
                PsalmVerse {
                    number: 13,
                    a: String::from("Worship the LORD, O Jerusalem; *"),
                    b: String::from("praise your God, O Zion;")
                },
                PsalmVerse {
                    number: 14,
                    a: String::from("For he has strengthened the bars of your gates; *"),
                    b: String::from("he has blessed your children within you.")
                },
                PsalmVerse {
                    number: 15,
                    a: String::from("He has established peace on your borders; *"),
                    b: String::from("he satisfies you with the finest wheat.")
                },
                PsalmVerse {
                    number: 16,
                    a: String::from("He sends out his command to the earth, *"),
                    b: String::from("and his word runs very swiftly.")
                },
                PsalmVerse {
                    number: 17,
                    a: String::from("He gives snow like wool; *"),
                    b: String::from("he scatters hoarfrost like ashes.")
                },
                PsalmVerse {
                    number: 18,
                    a: String::from("He scatters his hail like bread crumbs; *"),
                    b: String::from("who can stand against his cold?")
                },
                PsalmVerse {
                    number: 19,
                    a: String::from("He sends forth his word and melts them; *"),
                    b: String::from("he blows with his wind, and the waters flow.")
                },
                PsalmVerse {
                    number: 20,
                    a: String::from("He declares his word to Jacob, *"),
                    b: String::from("his statutes and his judgments to Israel.")
                },
                PsalmVerse {
                    number: 21,
                    a: String::from("He has not done so to any other nation; *"),
                    b: String::from("to them he has not revealed his judgments.\n Hallelujah!")
                },
            ]
        }]
    };
}
