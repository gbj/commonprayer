use hymnal::{HymnNumber, Hymnals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
/// A reference to a [Hymnal](hymnal::Hymnal), [HymnNumber](hymnal::HymnNumber), or [Hymn](hymnal::Hymn) tag.
pub enum HymnLink {
    Hymnals,
    Hymnal(Hymnals),
    Hymn(Hymnals, HymnNumber),
    Tag(String),
    TagWithLabel(String, String)
}
