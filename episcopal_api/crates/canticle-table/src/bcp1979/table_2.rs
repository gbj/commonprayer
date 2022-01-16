use calendar::{Season, Weekday};

use crate::{CanticleId, CanticleNumber, CanticleTable, CanticleTableEntry};

lazy_static! {
    pub static ref BCP1979_CANTICLE_TABLE_RITE_II: CanticleTable = CanticleTable::from([
        CanticleTableEntry {
            canticle: CanticleId::Canticle11,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle14,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Lent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle10,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Tue),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle14,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: Some(Season::Lent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle8,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle13,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle16,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle14,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: Some(Season::HolyWeek)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle14,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: Some(Season::Lent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle15,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle15,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: true,
            weekday: None,
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle10,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle14,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: Some(Season::Lent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle11,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Advent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle12,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sat),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle14,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Mon),
            season: Some(Season::HolyWeek)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle8,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Easter)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle16,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: true,
            weekday: None,
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle9,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Mon),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle9,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sat),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle8,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Mon),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle11,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle13,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Tue),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle12,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle14,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Mon),
            season: Some(Season::Lent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle11,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::HolyWeek)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle17,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: true,
            weekday: None,
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle20,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle17,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle17,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle16,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle17,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Mon),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle15,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Tue),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle18,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Tue),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle18,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle21,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: true,
            weekday: None,
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle16,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Advent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle21,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle19,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Mon),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle19,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: Some(Season::Lent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle15,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sat),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle16,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Lent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle19,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: Some(Season::Advent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle20,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: Some(Season::HolyWeek)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle19,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: Some(Season::HolyWeek)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle18,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle16,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::HolyWeek)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle15,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle19,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sat),
            season: None
        }
    ]);
}
