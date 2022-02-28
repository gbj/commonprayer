use liturgy::{Psalm, PsalmSection, PsalmVerse, Reference, Source};
lazy_static! {
    pub static ref PSALM_87: Psalm = Psalm {
        number: 87,
        citation: None,
        sections: vec![PsalmSection {
            reference: Reference {
                source: Source::BCP1979,
                page: 711
            },
            local_name: String::from(""),
            latin_name: String::from("Fundamenta ejus"),
            verses: vec![
                PsalmVerse {
                    number: 1,
                    a: String::from("On the holy mountain stands the city he has founded; *"),
                    b: String::from(
                        "the LORD loves the gates of Zion\nmore than all the dwellings of Jacob."
                    )
                },
                PsalmVerse {
                    number: 2,
                    a: String::from("Glorious things are spoken of you, *"),
                    b: String::from("O city of our God.")
                },
                PsalmVerse {
                    number: 3,
                    a: String::from("I count Egypt and Babylon among those who know me; *"),
                    b: String::from(
                        "behold Philistia, Tyre, and Ethiopia:\nin Zion were they born."
                    )
                },
                PsalmVerse {
                    number: 4,
                    a: String::from("Of Zion it shall be said, “Everyone was born in her, *"),
                    b: String::from("and the Most High himself shall sustain her.”")
                },
                PsalmVerse {
                    number: 5,
                    a: String::from("The LORD will record as he enrolls the peoples, *"),
                    b: String::from("“These also were born there.”")
                },
                PsalmVerse {
                    number: 6,
                    a: String::from("The singers and the dancers will say, *"),
                    b: String::from("“All my fresh springs are in you.”")
                },
            ]
        }]
    };
}
