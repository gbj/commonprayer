use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, AsRefStr, Display, EnumIter, EnumString, IntoStaticStr)]
pub enum VariousOccasions {
    HolyTrinity,
    HolySpirit,
    HolyAngels,
    Incarnation,
    HolyEucharist,
    HolyCross,
    AllBaptizedChristians,
    TheDeparted,
    ReignOfChrist,
    Baptism,
    Confirmation,
    Dedication,
    ChurchConvention,
    UnityOfTheChurch,
    EmberDays,
    MissionOfTheChurch,
    Nation,
    Peace,
    RogationDays,
    Sick,
    SocialJustice,
    SocialService,
    Education,
    Vocation,
    LaborDay,
}
