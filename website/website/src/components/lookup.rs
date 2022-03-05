use episcopal_api::liturgy::{CanticleTableEntry, Categories, LectionaryTableChoice, Version};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum LookupType {
    Category(Categories, Version, String),
    Canticle(CanticleTableEntry),
    Collect(Version),
    Lectionary(LectionaryTableChoice),
}

pub fn slugify(s: &str) -> String {
    s.to_lowercase().replace([' '], "-")
}

pub fn lookup_links(locale: &str, lookup_type: &LookupType) -> String {
    match lookup_type {
        LookupType::Category(category, version, name) => {
            if category == &Categories::ServiceOfLight {
                format!(
                    "/{}/document/office/{}/{:#?}",
                    locale,
                    slugify(name),
                    version
                )
            } else {
                format!("/{}/document/{}/{:#?}", locale, slugify(name), version)
            }
        }
        LookupType::Canticle(_) => format!("/{}/canticle-table", locale),
        LookupType::Collect(version) => {
            format!("/{}/document/collects/{:#?}", locale, version)
        }
        LookupType::Lectionary(lectionary) => match lectionary {
            LectionaryTableChoice::Preference(_) => format!("/{}/readings", locale),
            LectionaryTableChoice::Selected(lectionary) => {
                format!("/{}/readings/{:#?}", locale, lectionary)
            }
        },
    }
}
