use calendar::{Season, Weekday};

use crate::{CanticleId, CanticleNumber, CanticleTable, CanticleTableEntry};

lazy_static! {
    pub static ref BCP1979_CANTICLE_TABLE_RITE_I: CanticleTable = CanticleTable::from([
        CanticleTableEntry {
            canticle: CanticleId::Canticle3,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: true,
            weekday: None,
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle1,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sat),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle3,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle2,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle4,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle2,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Tue),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle1,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle4,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: true,
            weekday: None,
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle4,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Lent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle4,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Advent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle4,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::HolyWeek)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle4,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle3,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Tue),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle5,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Mon),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle5,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle7,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle5,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle3,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sat),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle5,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: true,
            weekday: None,
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle3,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle7,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: true,
            weekday: None,
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle5,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle6,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: None
        }
    ]);
}
