use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_D: Document = Document::from(Canticle {
        number: CanticleId::CanticleD,
        citation: Some(String::from("Isaiah 35:1-7,10")),
        local_name: String::from("Canticle D"),
        latin_name: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "The wilderness and the dry land shall be glad, *",
                    "the desert shall rejoice and blossom;"
                )),
                CanticleVerse::from((
                    "It shall blossom abundantly, *",
                    "and rejoice with joy and singing."
                )),
                CanticleVerse::from((
                    "They shall see the glory of the Lord, *",
                    "the majesty of our God."
                )),
                CanticleVerse::from((
                    "Strengthen the weary hands, *",
                    "and make firm the feeble knees."
                )),
                CanticleVerse::from((
                    "Say to the anxious, “Be strong, do not fear! *",
                    "Here is your God, coming with judgment to save you.”"
                )),
                CanticleVerse::from((
                    "Then shall the eyes of the blind be opened, *",
                    "and the ears of the deaf be unstopped."
                )),
                CanticleVerse::from((
                    "Then shall the lame leap like a deer, *",
                    "and the tongue of the speechless sing for joy."
                )),
                CanticleVerse::from((
                    "For waters shall break forth in the wilderness *",
                    "and streams in the desert;"
                )),
                CanticleVerse::from((
                    "The burning sand shall become a pool *",
                    "and the thirsty ground, springs of water."
                )),
                CanticleVerse::from((
                    "The ransomed of God shall return with singing, *",
                    "with everlasting joy upon their heads."
                )),
                CanticleVerse::from((
                    "Joy and gladness shall be theirs, *",
                    "and sorrow and sighing shall flee away."
                ))
            ]
        }]
    })
    .version(Version::EOW)
    ;
}
