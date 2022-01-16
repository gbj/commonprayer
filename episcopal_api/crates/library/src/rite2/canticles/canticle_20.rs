use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_20: Document = Document::from(Canticle {
        number: CanticleId::Canticle20,
        citation: None,
        local_name: String::from("Gloria in excelsis"),
        latin_name: Some(String::from("Gloria in excelsis")),
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "Glory to God in the highest, ",
                    "and peace to his people on earth. 
"
                )),
                CanticleVerse::from((
                    "Lord God, heavenly King,
almighty God and Father, ",
                    "we worship you, we give you thanks,
we praise you for your glory. 
"
                )),
                CanticleVerse::from((
                    "Lord Jesus Christ, only Son of the Father,
Lord God, Lamb of God,
you take away the sin of the world: ",
                    "have mercy on us; "
                )),
                CanticleVerse::from((
                    "you are seated at the right hand of the Father: ",
                    "receive our prayer. 
"
                )),
                CanticleVerse::from((
                    "For you alone are the Holy One,
you alone are the Lord,
you alone are the Most High,",
                    "Jesus Christ,
with the Holy Spirit,
in the glory of God the Father. Amen."
                ))
            ]
        }]
    })
    .version(Version::RiteII);
}
