use episcopal_api::liturgy::{CanticleTableEntry, LectionaryTableChoice, Version};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum LookupType {
    Category(Version, String),
    Canticle(CanticleTableEntry),
    Collect(Version),
    Lectionary(LectionaryTableChoice),
}

pub fn lookup_links(locale: &str, lookup_type: &LookupType) -> String {
    fn slugify(s: &str) -> String {
        s.to_lowercase().replace([' '], "-")
    }

    match lookup_type {
        LookupType::Category(version, name) => {
            format!(
                "/{}/document/category/{}/{:#?}",
                locale,
                slugify(name),
                version
            )
        }
        LookupType::Canticle(_) => format!("/{}/canticle-table", locale),
        LookupType::Collect(version) => {
            format!("/{}/document/category/collects/{:#?}", locale, version)
        }
        LookupType::Lectionary(lectionary) => match lectionary {
            LectionaryTableChoice::Preference(_) => format!("/{}/daily-readings", locale),
            LectionaryTableChoice::Selected(lectionary) => {
                format!("/{}/daily-readings/{:#?}", locale, lectionary)
            }
        },
    }
}
