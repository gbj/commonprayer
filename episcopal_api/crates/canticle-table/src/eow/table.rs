use calendar::{Season, Weekday};

use crate::{CanticleId, CanticleNumber, CanticleTable, CanticleTableEntry};

lazy_static! {
    pub static ref EOW_CANTICLE_TABLE: CanticleTable = CanticleTable::from([
        CanticleTableEntry {
            canticle: CanticleId::Canticle15,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleJ,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle14,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: Some(Season::HolyWeek)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleC,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Mon),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleD,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sat),
            season: None
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
            weekday: Some(Weekday::Sun),
            season: Some(Season::Christmas)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle11,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Mon),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleJ,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: Some(Season::Christmas)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleJ,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle10,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: Some(Season::Lent)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleE,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: true,
            weekday: None,
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleC,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Epiphany)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleH,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Lent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle16,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: true,
            weekday: None,
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleG,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleD,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Advent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle9,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Epiphany)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleA,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleD,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Tue),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleH,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleB,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Tue),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleB,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sat),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleH,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Lent)
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
            canticle: CanticleId::CanticleF,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: Some(Season::Lent)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleG,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: Some(Season::Easter)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleI,
            evening: false,
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
            canticle: CanticleId::CanticleI,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: Some(Season::HolyWeek)
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
            canticle: CanticleId::Canticle10,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: Some(Season::HolyWeek)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleA,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Easter)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleA,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Mon),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleC,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Christmas)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleC,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleG,
            evening: true,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: None
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
            canticle: CanticleId::CanticleE,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleI,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: Some(Season::Lent)
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
            canticle: CanticleId::CanticleF,
            evening: false,
            nth: CanticleNumber::One,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: Some(Season::HolyWeek)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleS,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: Some(Season::HolyWeek)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleS,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleO,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: true,
            weekday: None,
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleS,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleK,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: Some(Season::Easter)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleM,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle17,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleL,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Mon),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleQ,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleN,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Christmas)
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
            canticle: CanticleId::CanticleN,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Mon),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleK,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: true,
            weekday: None,
            season: None
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
            canticle: CanticleId::CanticleL,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Wed),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle16,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sat),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleM,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Tue),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleS,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: Some(Season::Lent)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleO,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sat),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleL,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Lent)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleM,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Easter)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleP,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Tue),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle20,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Epiphany)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleN,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Tue),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleN,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Epiphany)
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleR,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: Some(Season::Christmas)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle16,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleK,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleP,
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
            canticle: CanticleId::CanticleQ,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Mon),
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
            canticle: CanticleId::CanticleP,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Advent)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle19,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sat),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleR,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle16,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Tue),
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
            canticle: CanticleId::Canticle18,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Fri),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle20,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: Some(Season::Christmas)
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle16,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Thu),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::CanticleR,
            evening: true,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sat),
            season: None
        },
        CanticleTableEntry {
            canticle: CanticleId::Canticle21,
            evening: false,
            nth: CanticleNumber::Two,
            feast_day: false,
            weekday: Some(Weekday::Sun),
            season: None
        }
    ]);
}
