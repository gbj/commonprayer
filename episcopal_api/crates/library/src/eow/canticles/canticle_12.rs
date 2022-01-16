use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
  pub static ref CANTICLE_12_EOW: Document = Document::from(Canticle {
            number: CanticleId::Canticle12,
            citation: Some(String::from("Song of the Three Young Men, 35-65")),
            local_name: String::from("A Song of Creation"),
            latin_name: Some(String::from("Benedicite, omnia opera Domini")),
            sections: vec![CanticleSection {
              title: None,
              verses: vec![
                CanticleVerse::from(("Glorify the Lord, all you works of the Lord, *", "sing praise and give honor for ever.")),
CanticleVerse::from(("In the high vault of heaven, glorify the Lord, *", "sing praise and give honor for ever."))
              ]
            },
CanticleSection {
              title: None,
              verses: vec![
                CanticleVerse::from(("Glorify the Lord, you angels and all powers of the Lord, *", "O heavens and all waters above the heavens.")),
CanticleVerse::from(("Sun and moon and stars of the sky, glorify the Lord, *", "sing praise and give honor for ever.")),
CanticleVerse::from(("Glorify the Lord, every shower of rain and fall of dew, *", "all winds and fire and heat.")),
CanticleVerse::from(("Winter and Summer, glorify the Lord, *", "sing praise and give honor for ever.")),
CanticleVerse::from(("Glorify the Lord, O chill and cold, *", "drops of dew and flakes of snow")),
CanticleVerse::from(("Frost and cold, ice and sleet, glorify the Lord, *", "sing praise and give honor for ever.")),
CanticleVerse::from(("Glorify the Lord, O nights and days, *", "O shining light and enfolding dark.")),
CanticleVerse::from(("Storm clouds and thunderbolts, glorify the Lord, *", "sing praise and give honor for ever."))
              ]
            },
CanticleSection {
              title: None,
              verses: vec![
                CanticleVerse::from(("Let the earth glorify the Lord, *", "sing praise and give honor for ever.")),
CanticleVerse::from(("Glorify the Lord, O mountains and hills, and all that grows upon the earth, *", "sing praise and give honor for ever.")),
CanticleVerse::from(("Glorify the Lord, O springs of water, seas, and streams, *", "O whales and all that move in the waters.")),
CanticleVerse::from(("All birds of the air, glorify the Lord, *", "sing praise and give honor for ever.")),
CanticleVerse::from(("Glorify the Lord, O beasts of the wild, *", "and all you flocks and herds.")),
CanticleVerse::from(("O men and women everywhere, glorify the Lord, *", "sing praise and give honor for ever."))
              ]
            },
CanticleSection {
              title: None,
              verses: vec![
                CanticleVerse::from(("Let the people of God glorify the Lord, *", "sing praise and give honor for ever.")),
CanticleVerse::from(("Glorify the Lord, O priests and servants of the Lord, *", "sing praise and give honor for ever.")),
CanticleVerse::from(("Glorify the Lord, O spirits and souls of the righteous, *", "sing praise and give honor for ever.")),
CanticleVerse::from(("You that are holy and humble of heart, glorify the Lord, *", "sing praise and give honor for ever."))
              ]
            },
CanticleSection {
              title: None,
              verses: vec![
                CanticleVerse::from(("Let us glorify the Lord:  Father, Son, and Holy Spirit; *", "sing praise and give honor for ever.")),
CanticleVerse::from(("In the firmament of his power, glorify the Lord, *", "sing praise and give honor for ever."))
              ]
            }]
          }).version(Version::EOW);
          }
