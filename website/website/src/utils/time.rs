use episcopal_api::calendar::Date;

#[cfg(target_arch = "wasm32")]
pub fn current_hour() -> u32 {
    js_sys::Date::new_0().get_hours()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn current_hour() -> u32 {
    use chrono::Timelike;

    chrono::Local::now().hour()
}

#[cfg(target_arch = "wasm32")]
pub fn input_date_now() -> String {
    let now = js_sys::Date::new_0();
    format!(
        "{}-{:02}-{:02}",
        now.get_full_year(),
        now.get_month() + 1,
        now.get_date()
    )
}
#[cfg(not(target_arch = "wasm32"))]
pub fn input_date_now() -> String {
    use chrono::Datelike;

    let local = chrono::Local::now();
    format!("{}-{:02}-{:02}", local.year(), local.month(), local.day())
}

pub fn today() -> Date {
    Date::parse_from_str(&input_date_now(), "%Y-%m-%d").unwrap()
}

#[cfg(target_arch = "wasm32")]
pub fn now() -> (u32, u32) {
    let now = js_sys::Date::new_0();
    let hours: u32 = now.get_hours().into();
    let minutes: u32 = now.get_minutes().into();
    (hours, minutes)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn now() -> (u32, u32) {
    use chrono::Timelike;

    let time = chrono::offset::Local::now();
    (time.hour(), time.minute())
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

pub fn current_preferred_liturgy(ranges: &OfficeTimeRanges) -> &'static str {
    let (hour, minute) = now();
    if ranges.morning.includes(hour, minute) {
        "morning-prayer"
    } else if ranges.noon.includes(hour, minute) {
        "noonday-prayer"
    } else if ranges.evening.includes(hour, minute) {
        "evening-prayer"
    } else if ranges.night.includes(hour, minute) {
        "compline"
    } else {
        ""
    }
}
