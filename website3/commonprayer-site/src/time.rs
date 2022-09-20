use calendar::Date;
use leptos::Scope;

/// The current timezone offset in minutes, per
/// [Date.getTimezoneOffset()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTimezoneOffset).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimezoneOffset(pub i32);

impl Default for TimezoneOffset {
    fn default() -> Self {
        Self(0)
    }
}

#[cfg(feature = "csr")]
pub fn get_timezone_offset(cx: Scope) -> TimezoneOffset {
    TimezoneOffset(crate::js_sys::Date::new_0().get_timezone_offset() as i32)
}

pub fn today(tzoffset: &TimezoneOffset) -> Date {
    Date::parse_from_str(&input_date_now(tzoffset), "%Y-%m-%d").unwrap()
}

#[cfg(feature = "csr")]
pub fn input_date_now(_tzoffset: &TimezoneOffset) -> String {
    let now = crate::js_sys::Date::new_0();
    format!(
        "{}-{:02}-{:02}",
        now.get_full_year(),
        now.get_month() + 1,
        now.get_date()
    )
}

#[cfg(feature = "ssr")]
pub fn input_date_now(tzoffset: &TimezoneOffset) -> String {
    use chrono::Datelike;

    let utc_now = chrono::Utc::now();
    let local = utc_now - chrono::Duration::minutes(tzoffset.0.into());
    format!("{}-{:02}-{:02}", local.year(), local.month(), local.day())
}

#[cfg(not(any(feature = "ssr", feature = "csr", feature = "hydrate")))]
pub fn input_date_now(_tzoffset: &TimezoneOffset) -> String {
    // Every use of the crate should use one of these three features
    unreachable!()
}
