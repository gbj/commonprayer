use calendar::{Feast, LiturgicalDay, LiturgicalDayId};
use lectionary::Reading;
use liturgy::Psalm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DailySummary {
    pub morning: PartialDailySummary,
    pub evening: PartialDailySummary,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PartialDailySummary {
    pub day: LiturgicalDay,
    pub observed: ObservanceSummary,
    pub alternate: Option<ObservanceSummary>,
    pub thirty_day_psalms: Vec<Psalm>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ObservanceSummary {
    pub observance: LiturgicalDayId,
    pub localized_name: String,
    pub black_letter_days: Vec<(Feast, String)>,
    pub daily_office_readings: Vec<Reading>,
    pub daily_office_psalms: Vec<Psalm>,
}
