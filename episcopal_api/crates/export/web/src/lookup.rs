use liturgy::{CanticleTableEntry, Version, LectionaryTableChoice};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum LookupType {
    Category(Version, String),
    Canticle(CanticleTableEntry),
    Collect(Version),
    Lectionary(LectionaryTableChoice),
}

pub fn lookup_links(lookup_type: &LookupType) -> String {
    fn slugify(s: &str) -> String {
        s.to_lowercase().replace([' '], "-")
    }

    match lookup_type {
        LookupType::Category(version, name) => format!("/{:#?}/category/{}", version, slugify(name)),
        LookupType::Canticle(_) => "/canticle-table".to_string(),
        LookupType::Collect(version) => format!("/{:#?}/collects", version),
        LookupType::Lectionary(lectionary) => match lectionary {
            LectionaryTableChoice::Preference(_) => "/daily-readings".to_string(),
            LectionaryTableChoice::Selected(lectionary) => format!("/daily-readings/{:#?}", lectionary)
        }
    }
}
