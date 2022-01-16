use crate::{
    year::{DailyOfficeYear, RCLYear},
    Date, Feast, LiturgicalWeek, Proper, Weekday,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct LiturgicalDay {
    pub date: Date,
    pub evening: bool,
    pub week: LiturgicalWeek,
    pub proper: Option<Proper>,
    pub weekday: Weekday,
    pub daily_office_year: DailyOfficeYear,
    pub rcl_year: RCLYear,
    pub holy_days: Vec<Feast>,
    pub observed: LiturgicalDayId,
    pub alternate: Option<LiturgicalDayId>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum LiturgicalDayId {
    Feast(Feast),
    WeekAndDay(LiturgicalWeek, Weekday),
    ProperAndDay(Proper, Weekday),
    TransferredFeast(Feast),
    DayOfMonth(u8),
}
