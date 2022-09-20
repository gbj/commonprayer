mod eucharist;
mod holy_day;
mod office;
mod reading_links;

use crate::{i18n::use_language, icon::Icon};
use calendar::Date;
pub use eucharist::*;
pub use holy_day::*;
use language::Language;
use liturgy::Version;
pub use office::*;

use leptos::*;

use crate::{
    header::*,
    i18n::use_i18n,
    settings::use_display_settings,
    time::{get_timezone_offset, today},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ReadingsData {
    pub date: Memo<Date>,
    pub version: Memo<Version>,
}

pub fn readings_data(cx: Scope, _params: Memo<ParamsMap>, location: Location) -> ReadingsData {
    let tzoffset = get_timezone_offset(cx);

    let date = create_memo(cx, move |_| {
        location.query.with(|q| {
            q.get("date")
                .and_then(|date| date.parse::<Date>().ok())
                .unwrap_or_else(|| today(&tzoffset))
        })
    });

    let version = create_memo(cx, move |_| {
        location.query.with(|q| {
            q.get("version")
                .and_then(|version| version.parse::<Version>().ok())
                .unwrap_or(Version::NRSV)
        })
    });

    ReadingsData { date, version }
}

#[component]
pub fn Readings(cx: Scope) -> Vec<Element> {
    let (display_settings, _) = use_display_settings(cx);
    let (t, _, _) = use_i18n(cx);
    let ReadingsData { date, version } = use_loader(cx);

    view! {
        <>
            <Header label=t("daily-readings-title")></Header>
            <main class=move || display_settings().to_class()>
                // POST to download DOCX
                <form method="POST">
                    <button type="submit">
                        <img src=Icon::Download.to_string() alt=t("export-word")/>
                    </button>
                </form>

                // Change date
                <Form>
                    <label class="stacked">
                        {t("date")}
                        <input
                            type="date"
                            name="date"
                            value=move || date().to_padded_string()
                            onchange="this.form.requestSubmit()"
                        />
                    </label>
                    <label class="stacked">
                        {t("settings-bible_version")}
                        <select name="version"
                            onchange="this.form.requestSubmit()"
                        >
                            <BibleVersionOptions version />
                        </select>
                    </label>
                </Form>

                // Select category of readings
                <div class="toggle-links">
                    <NavLink to=move || format!("office/?date={}&version={}", date(), version())
                    >
                        {t("toc-daily_office")}
                    </NavLink>
                    <NavLink to=move || format!("eucharist/?date={}&version={}", date(), version())
                    >
                        {t("toc-holy_eucharist")}
                    </NavLink>
                    <NavLink to=move || format!("holy-day/?date={}&version={}", date(), version())
                    >
                        {t("toc-holy_days")}
                    </NavLink>
                </div>

                <Outlet/>
            </main>
        </>
    }
}

#[component]
fn BibleVersionOptions(cx: Scope, version: Memo<Version>) -> Memo<Vec<Element>> {
    let value = version;
    let locale = use_language(cx);
    let locale = move || Language::from_locale(&locale());
    let versions = move || match locale() {
        Language::Es => vec![
            Version::RV09,
            Version::NRSV,
            Version::CEB,
            Version::ESV,
            Version::KJV,
        ],
        _ => vec![
            Version::NRSV,
            Version::CEB,
            Version::ESV,
            Version::KJV,
            Version::RV09,
        ],
    };

    view! {
        <For each=versions key=|v| *v>{move |cx, version: &Version| {
            let value_str: &'static str = version.into();
            view! {
                <option value=value_str selected=value() == *version>{version.to_string()}</option>
            }
        }}</For>
    }
}
