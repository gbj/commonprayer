use calendar::{feasts::CommonOfSaints, Feast, LiturgicalWeek, Proper, Season, VariousOccasions};
use liturgy::Document;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CollectData {
    pub document: Document,
    pub preface: String,
    pub rubric_before: Option<String>,
    pub rubric_after: Option<String>,
}

impl From<Document> for CollectData {
    fn from(document: Document) -> Self {
        Self {
            document,
            preface: String::new(),
            rubric_before: None,
            rubric_after: None,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CollectId {
    Week(LiturgicalWeek),
    Proper(Proper),
    Season(Season),
    Feast(Feast),
    CommonOfSaints(CommonOfSaints),
    VariousOccasions(VariousOccasions),
}
pub struct CollectLinks([(CollectId, CollectId); 11]);

impl CollectLinks {
    pub fn linked_id(&self, initial_id: &CollectId) -> CollectId {
        self.0
            .iter()
            .find(|(from, _)| from == initial_id)
            .map(|(_, to)| *to)
            .unwrap_or(*initial_id)
    }
}

pub const COLLECT_LINKS: CollectLinks = CollectLinks([
    (
        CollectId::Feast(Feast::EmberDay),
        CollectId::VariousOccasions(VariousOccasions::EmberDays),
    ),
    (
        CollectId::Feast(Feast::RogationDay),
        CollectId::VariousOccasions(VariousOccasions::RogationDays),
    ),
    (
        CollectId::Season(Season::Advent),
        CollectId::Week(LiturgicalWeek::Advent1),
    ),
    (
        CollectId::Week(LiturgicalWeek::Christmas),
        CollectId::Feast(Feast::ChristmasDay),
    ),
    (
        CollectId::Week(LiturgicalWeek::Epiphany),
        CollectId::Feast(Feast::Epiphany),
    ),
    (
        CollectId::Season(Season::Lent),
        CollectId::Feast(Feast::AshWednesday),
    ),
    (
        CollectId::Feast(Feast::ThursdayAfterAshWednesday),
        CollectId::Feast(Feast::AshWednesday),
    ),
    (
        CollectId::Feast(Feast::FridayAfterAshWednesday),
        CollectId::Feast(Feast::AshWednesday),
    ),
    (
        CollectId::Feast(Feast::SaturdayAfterAshWednesday),
        CollectId::Feast(Feast::AshWednesday),
    ),
    (
        CollectId::Feast(Feast::FridayAfterAscension),
        CollectId::Feast(Feast::AscensionDay),
    ),
    (
        CollectId::Feast(Feast::SaturdayAfterAscension),
        CollectId::Feast(Feast::AscensionDay),
    ),
]);
