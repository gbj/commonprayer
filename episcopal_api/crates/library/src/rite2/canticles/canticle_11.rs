use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_11: Document = Document::from(Canticle {
        number: CanticleId::Canticle11,
        citation: Some(String::from("Isaiah 60:1-3, 11a, 14c, 18-19")),
        local_name: String::from("The Third Song of Isaiah"),
        latin_name: Some(String::from("Surge, illuminare")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Arise, shine, for your light has come, *",
                    "and the glory of the Lord has dawned upon you."
                )),
                CanticleVerse::from((
                    "For behold, darkness covers the land; *",
                    "deep gloom enshrouds the peoples."
                )),
                CanticleVerse::from((
                    "But over you the Lord will rise, *",
                    "and his glory will appear upon you."
                )),
                CanticleVerse::from((
                    "Nations will stream to your light, *",
                    "and kings to the brightness of your dawning."
                )),
                CanticleVerse::from((
                    "Your gates will always be open; *",
                    "by day or night they will never be shut."
                )),
                CanticleVerse::from((
                    "They will call you, The City of the Lord, *",
                    "The Zion of the Holy One of Israel."
                )),
                CanticleVerse::from((
                    "Violence will no more be heard in your land, *",
                    "ruin or destruction within your borders."
                )),
                CanticleVerse::from((
                    "You will call your walls, Salvation, *",
                    "and all your portals, Praise."
                )),
                CanticleVerse::from((
                    "The sun will no more be your light by day; *",
                    "by night you will not need the brightness of the moon."
                )),
                CanticleVerse::from((
                    "The Lord will be your everlasting light, *",
                    "and your God will be your glory."
                ))
            ]
        }]
    })
    .version(Version::RiteII);
}
