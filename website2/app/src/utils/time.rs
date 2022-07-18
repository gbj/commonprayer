use calendar::Date;
use leptos2::Arc;
use liturgy::{Slug, SlugPath};

/// The current timezone offset in minutes, per
/// [Date.getTimezoneOffset()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTimezoneOffset).
pub struct TimezoneOffset(pub i32);

impl Default for TimezoneOffset {
    fn default() -> Self {
        Self(0)
    }
}

/// The `tzoffset` cookie stores the current timezone offset in minutes, per
/// [Date.getTimezoneOffset()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTimezoneOffset).
/// This will only be set if cookies are allowed and you've already loaded a page on the site.
#[cfg(not(target_arch = "wasm32"))]
impl From<&Arc<dyn leptos2::Request>> for TimezoneOffset {
    fn from(req: &Arc<dyn leptos2::Request>) -> Self {
        use leptos2::Cookies;

        TimezoneOffset(
            req.headers()
                .cookies()
                .filter_map(|cookie| match cookie {
                    Ok(cookie) => Some(cookie),
                    Err(e) => {
                        eprintln!("invalid cookie: {:#?}", e);
                        None
                    }
                })
                .find(|cookie| cookie.name() == "tzoffset")
                .and_then(|cookie| cookie.value().parse::<i32>().ok())
                .unwrap_or(0),
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<Arc<dyn leptos2::Request>> for TimezoneOffset {
    fn from(req: Arc<dyn leptos2::Request>) -> Self {
        Self::from(&req)
    }
}

#[cfg(target_arch = "wasm32")]
pub fn current_hour(_tzoffset: &TimezoneOffset) -> u32 {
    js_sys::Date::new_0().get_hours()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn current_hour(tzoffset: &TimezoneOffset) -> u32 {
    use chrono::Timelike;

    let utc_now = chrono::Utc::now();
    let local_now = utc_now - chrono::Duration::minutes(tzoffset.0.into());
    local_now.hour()
}

#[cfg(target_arch = "wasm32")]
pub fn input_date_now(_tzoffset: &TimezoneOffset) -> String {
    let now = js_sys::Date::new_0();
    format!(
        "{}-{:02}-{:02}",
        now.get_full_year(),
        now.get_month() + 1,
        now.get_date()
    )
}
#[cfg(not(target_arch = "wasm32"))]
pub fn input_date_now(tzoffset: &TimezoneOffset) -> String {
    use chrono::Datelike;

    let utc_now = chrono::Utc::now();
    let local = utc_now - chrono::Duration::minutes(tzoffset.0.into());
    format!("{}-{:02}-{:02}", local.year(), local.month(), local.day())
}

pub fn today(tzoffset: &TimezoneOffset) -> Date {
    Date::parse_from_str(&input_date_now(tzoffset), "%Y-%m-%d").unwrap()
}

#[cfg(target_arch = "wasm32")]
pub fn now(_tzoffset: &TimezoneOffset) -> (u32, u32) {
    let now = js_sys::Date::new_0();
    let hours: u32 = now.get_hours().into();
    let minutes: u32 = now.get_minutes().into();
    (hours, minutes)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn now(tzoffset: &TimezoneOffset) -> (u32, u32) {
    use chrono::Timelike;

    let utc_now = chrono::Utc::now();
    let local_now = utc_now - chrono::Duration::minutes(tzoffset.0.into());
    (local_now.hour(), local_now.minute())
}

pub struct OfficeTimeRanges {
    morning: OfficeTimeRange,
    noon: OfficeTimeRange,
    evening: OfficeTimeRange,
    night: OfficeTimeRange,
}

pub struct OfficeTimeRange((u32, u32), (u32, u32));

impl OfficeTimeRange {
    pub fn includes(&self, hour: u32, minute: u32) -> bool {
        let (start_hour, start_minute) = self.0;
        let (end_hour, end_minute) = self.1;

        (hour > start_hour || (hour == start_hour && minute > start_minute))
            && (hour < end_hour
                || (hour == end_hour && minute < end_minute)
                || end_hour < start_hour)
    }
}

pub const DEFAULT_OFFICE_TIMES: OfficeTimeRanges = OfficeTimeRanges {
    morning: OfficeTimeRange((3, 0), (11, 0)),
    noon: OfficeTimeRange((11, 0), (14, 0)),
    evening: OfficeTimeRange((14, 0), (20, 0)),
    night: OfficeTimeRange((20, 0), (3, 0)),
};

pub fn current_preferred_liturgy(tzoffset: &TimezoneOffset, ranges: &OfficeTimeRanges) -> SlugPath {
    let (hour, minute) = now(tzoffset);
    if ranges.morning.includes(hour, minute) {
        SlugPath::from([Slug::Office, Slug::MorningPrayer])
    } else if ranges.noon.includes(hour, minute) {
        SlugPath::from([Slug::Office, Slug::NoondayPrayer])
    } else if ranges.evening.includes(hour, minute) {
        SlugPath::from([Slug::Office, Slug::EveningPrayer])
    } else if ranges.night.includes(hour, minute) {
        SlugPath::from([Slug::Office, Slug::Compline])
    } else {
        SlugPath::from([Slug::Office])
    }
}
