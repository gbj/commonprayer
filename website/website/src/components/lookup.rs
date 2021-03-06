use liturgy::{CanticleTableEntry, LectionaryTableChoice, Version, Lectionaries};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum LookupType {
    Canticle(CanticleTableEntry),
    Collect(Version),
    Lectionary(LectionaryTableChoice),
}

pub fn lookup_links(locale: &str, lookup_type: &LookupType) -> String {
    match lookup_type {
        LookupType::Canticle(_) => format!("/{}/canticle-table", locale),
        LookupType::Collect(version) => {
            format!("/{}/document/collects/{:#?}", locale, version)
        }
        LookupType::Lectionary(lectionary) => match lectionary {
            LectionaryTableChoice::Preference(_) => format!("/{}/readings", locale),
            LectionaryTableChoice::Selected(lectionary) => {
                match lectionary {
                    Lectionaries::RCLTrack1 | Lectionaries::RCLTrack2 => format!("/{}/lectionary", locale),
                    _ => format!("/{}/readings/{:#?}", locale, lectionary)

                }
            }
        },
    }
}
