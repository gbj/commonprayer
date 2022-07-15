use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_1: Document = Document::from(Canticle {
        number: CanticleId::Canticle1,
        changeable: None,
        citation: Some(String::from("Song of the Three Young Men, 35-65")),
        local_name: String::from("A Song of Creation"),
        latin_name: Some(String::from("Benedicite, omnia opera Domini")),
        rubric: Some(String::from(
            "This Canticle may be shortened by omitting section II or III"
        )),
        sections: vec![
            CanticleSection {
                title: Some(String::from("I\tInvocation")),
                verses: vec![
                    CanticleVerse::from((
                        "O all ye works of the Lord, bless ye the Lord; *",
                        "praise him and magnify him for ever."
                    )),
                    CanticleVerse::from((
                        "O ye angels of the Lord, bless ye the Lord; *",
                        "praise him and magnify him for ever."
                    ))
                ]
            },
            CanticleSection {
                title: Some(String::from("II\tThe Cosmic Order")),
                verses: vec![
                    CanticleVerse::from((
                        "O ye heavens, bless ye the Lord; *",
                        "O ye waters that be above the firmament, bless ye the Lord;"
                    )),
                    CanticleVerse::from((
                        "O all ye powers of the Lord, bless ye the Lord; *",
                        "praise him and magnify him for ever."
                    )),
                    CanticleVerse::from((
                        "O ye sun and moon, bless ye the Lord; *",
                        "O ye stars of heaven, bless ye the Lord;"
                    )),
                    CanticleVerse::from((
                        "O ye showers and dew, bless ye the Lord; *",
                        "praise him and magnify him for ever."
                    )),
                    CanticleVerse::from((
                        "O ye winds of God, bless ye the Lord; *",
                        "O ye fire and heat, bless ye the Lord;"
                    )),
                    CanticleVerse::from((
                        "O ye winter and summer, bless ye the Lord; *",
                        "praise him and magnify him for ever."
                    )),
                    CanticleVerse::from((
                        "O ye dews and frosts, bless ye the Lord; *",
                        "O ye frost and cold, bless ye the Lord;"
                    )),
                    CanticleVerse::from((
                        "O ye ice and snow, bless ye the Lord; *",
                        "praise him and magnify him for ever."
                    )),
                    CanticleVerse::from((
                        "O ye nights and days, bless ye the Lord; *",
                        "O ye light and darkness, bless ye the Lord;"
                    )),
                    CanticleVerse::from((
                        " O ye lightnings and clouds, bless ye the Lord; *",
                        "praise him and magnify him for ever."
                    ))
                ]
            },
            CanticleSection {
                title: Some(String::from("III\tThe Earth and its Creatures")),
                verses: vec![
                    CanticleVerse::from((
                        "O let the earth bless the Lord; *",
                        "O ye mountains and hills, bless ye the Lord;"
                    )),
                    CanticleVerse::from((
                        " O all ye green things upon the earth, bless ye the Lord; *",
                        "praise him and magnify him for ever."
                    )),
                    CanticleVerse::from((
                        "O ye wells, bless ye the Lord; *",
                        "O ye seas and floods, bless ye the Lord;"
                    )),
                    CanticleVerse::from((
                        "O ye whales and all that move in the waters, bless ye the Lord;",
                        "praise him and magnify him for ever."
                    )),
                    CanticleVerse::from((
                        "O all ye fowls of the air, bless ye the Lord; *",
                        "O all ye beasts and cattle, bless ye the Lord;"
                    )),
                    CanticleVerse::from((
                        "O ye children of men, bless ye the Lord; *",
                        "praise him and magnify him for ever."
                    ))
                ]
            },
            CanticleSection {
                title: Some(String::from("IV\tThe People of God")),
                verses: vec![
                    CanticleVerse::from((
                        "O ye people of God, bless ye the Lord; *",
                        "O ye priests of the Lord, bless ye the Lord;"
                    )),
                    CanticleVerse::from((
                        " O ye servants of the Lord, bless ye the Lord; *",
                        "praise him and magnify him for ever."
                    ))
                ]
            },
            CanticleSection {
                title: None,
                verses: vec![
                    CanticleVerse::from((
                        "O ye spirits and souls of the righteous, bless ye the Lord; *",
                        "O ye holy and humble men of heart, bless ye the Lord."
                    )),
                    CanticleVerse::from((
                        "Let us bless the Father, the Son, and the Holy Spirit; *",
                        "praise him and magnify him for ever."
                    ))
                ]
            }
        ],
        gloria_patri: None
    })
    .version(Version::RiteI)
    .page(47);
}
