use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum Rank {
    PrincipalFeast = 10,
    PrecedenceOverSunday = 9,
    Sunday = 8,
    PrecedenceOverHolyDay = 7,
    HolyDay = 6,
    SpecialDevotion = 5,
    PrecedenceOverWeekday = 4,
    OptionalObservance = 2,
    FerialWeekday = 0,
}
