use calendar::{Season, Weekday};
use liturgy::{Condition, GlobalPref, Lectionaries, PreferenceKey, PreferenceValue};

lazy_static! {
    /// True when it is not Lent (including Holy Week)
    pub static ref LENT: Condition = Condition::Or(
        Box::new(Condition::Season(Season::Lent)),
        Box::new(Condition::Season(Season::HolyWeek))
    );
    pub static ref NOT_LENT: Condition = Condition::Not(Box::new(LENT.clone()));

    /// True when the "Insert Gloria Patri between psalms" preference is not set
    pub static ref NOT_INSERT_GLORIA: Condition = Condition::Not(Box::new(Condition::Preference(
        PreferenceKey::from(GlobalPref::InsertGloria),
        PreferenceValue::Bool(true)
    )));

    /// True only from Easter to Pentecost
    pub static ref EASTER_SEASON: Condition = Condition::Any(vec![
        Condition::Season(Season::Easter),
        Condition::Season(Season::Ascension),
        Condition::Season(Season::Pentecost)
    ]);

    /// True only on Fridays in Lent
    pub static ref FRIDAY_IN_LENT: Condition = Condition::And(
        Box::new(Condition::Season(Season::Lent)),
        Box::new(Condition::Weekday(Weekday::Fri))
    );


    /// True unless it's the 19th of the month and you're using the 30-day Psalter,
    /// in which case Psalm 95 is appointed for Morning Prayer, so we use the Jubilate
    pub static ref VENITE_NOT_IN_PSALMS: Condition = Condition::Not(Box::new(Condition::And(
        Box::new(Condition::Preference(PreferenceKey::from(GlobalPref::PsalmCycle), PreferenceValue::from(Lectionaries::BCP1979ThirtyDayPsalms))),
        Box::new(Condition::DayOfMonth(19))
    )));
}
