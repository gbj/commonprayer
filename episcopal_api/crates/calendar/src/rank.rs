use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize, AsRefStr, Display, EnumIter, EnumString, IntoStaticStr)]
pub enum Rank {
    PrincipalFeast = 10,
    PrecedenceOverSunday = 9,
    Sunday = 8,
    PrecedenceOverHolyDay = 7,
    HolyDay = 6,
    SpecialDevotion = 5,
    PrecedenceOverWeekday = 3,
    EmberDay = 2,
    OptionalObservance = 1,
    FerialWeekday = 0,
}
