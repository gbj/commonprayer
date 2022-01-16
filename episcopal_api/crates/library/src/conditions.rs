use calendar::Season;
use liturgy::{Condition, GlobalPref, PreferenceKey, PreferenceValue};

lazy_static! {
    /// True when it is not Lent (including Holy Week)
    pub static ref NOT_LENT: Condition = Condition::Not(Box::new(Condition::Or(
        Box::new(Condition::Season(Season::Lent)),
        Box::new(Condition::Season(Season::HolyWeek))
    )));

    /// True when the "Insert Gloria Patri between psalms" preference is not set
    pub static ref NOT_INSERT_GLORIA: Condition = Condition::Not(Box::new(Condition::Preference(
        PreferenceKey::from(GlobalPref::InsertGloria),
        PreferenceValue::from("true")
    )));

    /// True only from Easter to Pentecost
    pub static ref EASTER_SEASON: Condition = Condition::Any(vec![
        Condition::Season(Season::Easter),
        Condition::Season(Season::Ascension),
        Condition::Season(Season::Pentecost)
    ]);
}
