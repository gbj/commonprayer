use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_145: Psalm = Psalm {
        number: 145,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 801
            },
            local_name: String::from(""),
            latin_name: String::from("Exaltabo te, Deus"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("I will exalt you, O God my King, *"),
                    b: String::from("and bless your Name for ever and ever.")
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Every day will I bless you *"),
                    b: String::from("and praise your Name for ever and ever.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("Great is the LORD and greatly to be praised; *"),
                    b: String::from("there is no end to his greatness.")
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("One generation shall praise your works to another *"),
                    b: String::from("and shall declare your power.")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("I will ponder the glorious splendor of your majesty *"),
                    b: String::from("and all your marvelous works.")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("They shall speak of the might of your wondrous acts, *"),
                    b: String::from("and I will tell of your greatness.")
                },
                PsalmVerse {
                    number: 7,
                    a: String::from("They shall publish the remembrance of your great goodness; *"),
                    b: String::from("they shall sing of your righteous deeds.")
                },
                PsalmVerse {
                    number: 8,
                    a: String::from("The LORD is gracious and full of compassion, *"),
                    b: String::from("slow to anger and of great kindness.")
                },
                PsalmVerse {
                    number: 9,
                    a: String::from("The LORD is loving to everyone *"),
                    b: String::from("and his compassion is over all his works.")
                },
                PsalmVerse {
                    number: 10,
                    a: String::from("All your works praise you, O LORD, *"),
                    b: String::from("and your faithful servants bless you.")
                },
                PsalmVerse {
                    number: 11,
                    a: String::from("They make known the glory of your kingdom *"),
                    b: String::from("and speak of your power;")
                },
                PsalmVerse {
                    number: 12,
                    a: String::from("That the peoples may know of your power *"),
                    b: String::from("and the glorious splendor of your kingdom.")
                },
                PsalmVerse {
                    number: 13,
                    a: String::from("Your kingdom is an everlasting kingdom; *"),
                    b: String::from("your dominion endures throughout all ages.")
                },
                PsalmVerse {
                    number: 14,
                    a: String::from("The LORD is faithful in all his words *"),
                    b: String::from("and merciful in all his deeds.")
                },
                PsalmVerse {
                    number: 15,
                    a: String::from("The LORD upholds all those who fall; *"),
                    b: String::from("he lifts up those who are bowed down.")
                },
                PsalmVerse {
                    number: 16,
                    a: String::from("The eyes of all wait upon you, O LORD, *"),
                    b: String::from("and you give them their food in due season.")
                },
                PsalmVerse {
                    number: 17,
                    a: String::from("You open wide your hand *"),
                    b: String::from("and satisfy the needs of every living creature.")
                },
                PsalmVerse {
                    number: 18,
                    a: String::from("The LORD is righteous in all his ways *"),
                    b: String::from("and loving in all his works.")
                },
                PsalmVerse {
                    number: 19,
                    a: String::from("The LORD is near to those who call upon him, *"),
                    b: String::from("to all who call upon him faithfully.")
                },
                PsalmVerse {
                    number: 20,
                    a: String::from("He fulfills the desire of those who fear him; *"),
                    b: String::from("he hears their cry and helps them.")
                },
                PsalmVerse {
                    number: 21,
                    a: String::from("The LORD preserves all those who love him, *"),
                    b: String::from("but he destroys all the wicked.")
                },
                PsalmVerse {
                    number: 22,
                    a: String::from("My mouth shall speak the praise of the LORD; *"),
                    b: String::from("let all flesh bless his holy Name for ever and ever.")
                },
            ]
        }]
    };
}
