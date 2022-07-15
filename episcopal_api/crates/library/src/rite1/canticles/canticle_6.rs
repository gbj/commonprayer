use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_6: Document = Document::from(Canticle {
        number: CanticleId::Canticle6,
        changeable: None,
        citation: None,
        local_name: String::from("Glory be to God"),
        latin_name: Some(String::from("Gloria in Excelsis")),
        rubric: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![CanticleVerse::from((
                "Glory be to God on high,
    and on earth peace, good will towards men.

We praise thee, we bless thee,
    we worship thee,
    we glorify thee,
    we give thanks to thee for thy great glory,
O Lord God, heavenly King, God the Father Almighty.

O Lord, the only-begotten Son, Jesus Christ;
O Lord God, Lamb of God, Son of the Father,
    that takest away the sins of the world,
    have mercy upon us.
Thou that takest away the sins of the world,
    receive our prayer.
Thou that sittest at the right hand of God the Father,
    have mercy upon us.

For thou only art holy,
thou only art the Lord,
thou only, O Christ,
    with the Holy Ghost,
    art most high in the glory of God the Father.",
                "undefined"
            ))]
        }],
        gloria_patri: None
    })
    .version(Version::RiteI)
    .page(52);
}
