use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
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
