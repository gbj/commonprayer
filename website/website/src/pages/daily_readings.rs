use std::rc::Rc;

use episcopal_api::{
    api::summary::{DailySummary, ObservanceSummary},
    calendar::Date,
    lectionary::Reading,
    library::CommonPrayer,
    liturgy::{BiblicalCitation, Document, Psalm},
};
use futures::{Stream, StreamExt};
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::UnwrapThrowExt;

use crate::{
    components::*,
    utils::{language::locale_to_language, time::current_hour},
};

pub fn daily_readings() -> Page<DailyReadingsPageProps, DailyReadingsUrlParams> {
    Page::new("daily-readings")
        .head_fn(head)
        .body_fn(body)
        .static_props_fn(static_props)
        .build_paths_fn(build_paths_fn)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DailyReadingsPageProps {
    summary: Option<DailySummary>,
}

#[derive(Deserialize, Clone)]
pub struct DailyReadingsUrlParams {
    date: Option<String>,
}

fn head(locale: &str, props: &DailyReadingsPageProps) -> View {
    let title = format!("{} – {}", t!("toc.daily_readings"), t!("common_prayer"));

    // no summary means no date param is present in URL => redirect to client's current day
    let redirect_script = if props.summary.is_none() {
        let js = format!(
            r#"
            const now = new Date(),
                formatted = `${{now.getFullYear()}}-${{now.getMonth() + 1}}-${{now.getDate()}}`;
            window.location.href = `/{}/daily-readings/${{formatted}}`;
        "#,
            locale
        );
        view! {
            <script>{js}</script>
        }
    } else {
        View::Empty
    };

    view! {
        <>
            <title>{title}</title>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/document.css"/>
            <link rel="stylesheet" href="/static/daily-readings.css"/>
            {redirect_script}
        </>
    }
}

fn static_props(
    locale: &str,
    _path: &str,
    params: DailyReadingsUrlParams,
) -> DailyReadingsPageProps {
    let language = locale_to_language(locale);

    let date = params
        .date
        .as_ref()
        .and_then(|date| Date::parse_from_str(date, "%Y-%m-%d").ok());
    let summary = date.map(|date| CommonPrayer::summarize_date(&date, language));

    DailyReadingsPageProps { summary }
}

fn build_paths_fn() -> Vec<String> {
    vec!["{date}".into(), "".into()]
}

fn body(locale: &str, props: &DailyReadingsPageProps) -> View {
    if let Some(summary) = props.summary.as_ref() {
        let controls = Controls::new(summary.clone());

        let primary_morning = observance_view(
            locale,
            current_hour() < 13,
            (
                controls.use_evening.toggled.stream(),
                controls.use_alternate_observance.toggled.stream(),
            )
                .lift()
                .map(|(evening, alternate)| {
                    evening.map(|evening| !evening && !alternate.unwrap_or(false))
                }),
            controls.use_30_day_psalter.toggled.clone(),
            &summary.morning.observed,
            &summary.morning.thirty_day_psalms,
        );

        let primary_evening = observance_view(
            locale,
            current_hour() >= 13,
            (
                controls.use_evening.toggled.stream(),
                controls.use_alternate_observance.toggled.stream(),
            )
                .lift()
                .map(|(evening, alternate)| {
                    evening.map(|evening| evening && !alternate.unwrap_or(false))
                }),
            controls.use_30_day_psalter.toggled.clone(),
            &summary.evening.observed,
            &summary.evening.thirty_day_psalms,
        );

        let alt_morning = observance_view(
            locale,
            false,
            (
                controls.use_evening.toggled.stream(),
                controls.use_alternate_observance.toggled.stream(),
            )
                .lift()
                .map(|(evening, alternate)| match (evening, alternate) {
                    (Some(true), Some(true)) => Some(true),
                    (None, Some(true)) => Some(true),
                    _ => Some(false),
                }),
            controls.use_30_day_psalter.toggled.clone(),
            summary
                .morning
                .alternate
                .as_ref()
                .unwrap_or(&summary.evening.observed),
            &summary.morning.thirty_day_psalms,
        );

        let alt_evening = observance_view(
            locale,
            false,
            (
                controls.use_evening.toggled.stream(),
                controls.use_alternate_observance.toggled.stream(),
            )
                .lift()
                .map(|(evening, alternate)| match (evening, alternate) {
                    (Some(true), Some(true)) => Some(true),
                    _ => Some(false),
                }),
            controls.use_30_day_psalter.toggled.clone(),
            summary
                .evening
                .alternate
                .as_ref()
                .unwrap_or(&summary.evening.observed),
            &summary.evening.thirty_day_psalms,
        );

        let locale = locale.to_string();

        let primary_reading_links = reading_links(
            &summary.morning.observed,
            &summary.evening.observed,
            &summary.morning.thirty_day_psalms,
            &summary.evening.thirty_day_psalms,
        );

        let alternate_reading_links = reading_links(
            summary
                .morning
                .alternate
                .as_ref()
                .unwrap_or(&summary.morning.observed),
            summary
                .evening
                .alternate
                .as_ref()
                .unwrap_or(&summary.evening.observed),
            &summary.morning.thirty_day_psalms,
            &summary.evening.thirty_day_psalms,
        );

        view! {
            <>
                {header(&locale, &t!("toc.daily_readings"))}
                <main>
                    <label class="stacked">
                        {leptos::View::StaticText(t!("date"))}
                        <dyn:input type="date" class="centered"
                            value={summary.date.to_padded_string()}
                            on:change={
                                let locale = locale.clone();
                                move |ev: Event| redirect_to_date(&locale, event_target_value(ev))
                            }
                        />
                    </label>

                    // Controls to select morning/evening, psalm cycle, and alternate observance
                    <dyn:view view={controls.view()}/>

                    // Reading links
                    <dyn:section
                        class="primary reading-link-table"
                        class:hidden={controls.use_alternate_observance.toggled.stream().boxed_local()}
                    >
                        {reading_links_view(primary_reading_links, controls.use_30_day_psalter.toggled.clone())}
                    </dyn:section>
                    <dyn:section
                        class="alternate reading-link-table hidden "
                        class:hidden={controls.use_alternate_observance.toggled.stream().map(|using| !using).boxed_local()}
                    >
                        {reading_links_view(alternate_reading_links, controls.use_30_day_psalter.toggled)}
                    </dyn:section>

                    // The psalms and readings themselves
                    <dyn:view view={primary_morning}/>
                    <dyn:view view={primary_evening}/>
                    <dyn:view view={alt_morning}/>
                    <dyn:view view={alt_evening}/>
                </main>
            </>
        }
    }
    // if no summary, Javascript (see header) will redirect to current date
    else {
        view! {
            <>
                {header(locale, &t!("toc.daily_readings"))}
                <main>
                </main>
            </>
        }
    }
}

fn redirect_to_date(locale: &str, date: String) {
    location()
        .set_href(&format!("/{locale}/daily-readings/{date}"))
        .unwrap_throw();
}

struct ReadingLinks<'a> {
    morning_office_psalms: Vec<&'a str>,
    evening_office_psalms: Vec<&'a str>,
    morning_30_psalms: Vec<&'a str>,
    evening_30_psalms: Vec<&'a str>,
    morning_readings: Vec<&'a str>,
    evening_readings: Vec<&'a str>,
}

fn reading_links<'a>(
    morning: &'a ObservanceSummary,
    evening: &'a ObservanceSummary,
    morning_thirty_day_psalms: &'a [Psalm],
    evening_thiry_day_psalms: &'a [Psalm],
) -> ReadingLinks<'a> {
    ReadingLinks {
        morning_office_psalms: psalm_links(&morning.daily_office_psalms),
        evening_office_psalms: psalm_links(&evening.daily_office_psalms),
        morning_30_psalms: psalm_links(morning_thirty_day_psalms),
        evening_30_psalms: psalm_links(evening_thiry_day_psalms),
        morning_readings: lectionary_reading_links(&morning.daily_office_readings),
        evening_readings: lectionary_reading_links(&evening.daily_office_readings),
    }
}

fn psalm_links(psalms: &[Psalm]) -> Vec<&str> {
    psalms
        .iter()
        .filter_map(|psalm| psalm.citation.as_deref())
        .collect()
}

fn lectionary_reading_links(readings: &[Reading]) -> Vec<&str> {
    readings
        .iter()
        .map(|reading| reading.citation.as_str())
        .collect()
}

fn reading_links_view(links: ReadingLinks, thirty_day: Behavior<bool>) -> View {
    let readings_different = links.morning_readings != links.evening_readings;

    view! {
        <table>
            <tr>
                <th>{t!("daily_readings.morning")}</th>
                <th>{t!("daily_readings.evening")}</th>
            </tr>
            <tr>
                <td>
                    <dyn:view view={
                        psalm_links_view(
                            links.morning_office_psalms,
                            links.morning_30_psalms,
                            thirty_day.clone()
                        )
                    }/>
                </td>
                <td>
                    <dyn:view view={
                        psalm_links_view(
                            links.evening_office_psalms,
                            links.evening_30_psalms,
                            thirty_day
                        )
                    }/>
                </td>
            </tr>
            <tr>
                <td colspan={if readings_different { "1" } else { "2" } }>
                    {reading_links_reading_view(links.morning_readings)}
                </td>
                <td>
                    { if readings_different {
                        reading_links_reading_view(links.evening_readings)
                    } else {
                        View::Empty
                    }}
                </td>
            </tr>
        </table>
    }
}

fn psalm_links_view(
    daily_psalms: Vec<&str>,
    thirty_day_psalms: Vec<&str>,
    use_thirty_day_psalms: Behavior<bool>,
) -> View {
    let daily_psalm_links = View::Fragment(
        daily_psalms
            .iter()
            .map(|citation| {
                view! {
                    <li>
                        <a href={&format!("#{}", citation)}>{citation.to_string()}</a>
                    </li>
                }
            })
            .collect(),
    );
    let thirty_psalm_links = View::Fragment(
        thirty_day_psalms
            .iter()
            .map(|citation| {
                view! {
                    <li>
                        <a href={&format!("#{}", citation)}>{citation.to_string()}</a>
                    </li>
                }
            })
            .collect(),
    );
    view! {
        <>
            <dyn:ul class:hidden={use_thirty_day_psalms.stream().map(|use_30| use_30).boxed_local()}>
                {daily_psalm_links}
            </dyn:ul>
            <dyn:ul
                class="hidden" // TODO allow this to be set by default preference
                class:hidden={use_thirty_day_psalms.stream().map(|use_30| !use_30).boxed_local()}
            >
                {thirty_psalm_links}
            </dyn:ul>
        </>
    }
}

fn reading_links_reading_view(readings: Vec<&str>) -> View {
    let reading_links = View::Fragment(
        readings
            .iter()
            .map(|citation| {
                view! {
                    <li>
                        <a href={&format!("#{}", citation)}>{citation.to_string()}</a>
                    </li>
                }
            })
            .collect(),
    );

    view! {
        <ul>{reading_links}</ul>
    }
}

fn observance_view(
    locale: &str,
    use_default: bool,
    use_this_observance: impl Stream<Item = Option<bool>> + 'static,
    use_thirty_day_psalms: Behavior<bool>,
    observance: &ObservanceSummary,
    thirty_day_psalms: &[Psalm],
) -> View {
    let black_letter_days = View::Fragment(
        observance
            .black_letter_days
            .iter()
            .map(|(feast, name)| {
                let url = format!("/{}/holy-day/{:#?}", locale, feast);
                view! {
                    <li>
                        <a href={url}>{name}</a>
                    </li>
                }
            })
            .collect(),
    );

    view! {
        <dyn:section
            class={if use_default { "" } else { "hidden" }}
            class:hidden={use_this_observance.map(move |yes| !yes.unwrap_or(use_default)).boxed_local()}
        >
            <h1>{&observance.localized_name}</h1>
            <ul class="black-letter-days">{black_letter_days}</ul>

            <dyn:section
                class:hidden={use_thirty_day_psalms.stream().map(|use_30| use_30).boxed_local()}
            >
                <h2>{t!("daily_readings.psalms")}</h2>
                {psalms_view(locale, &observance.daily_office_psalms)}
            </dyn:section>
            <dyn:section
                class="hidden" // TODO allow this to be set by default preference
                class:hidden={use_thirty_day_psalms.stream().map(|use_30| !use_30).boxed_local()}
            >
                <h2>{t!("daily_readings.psalms")}</h2>
                {psalms_view(locale, thirty_day_psalms)}
            </dyn:section>
            <section>
                <h2>{t!("daily_readings.daily_office_readings")}</h2>
                {readings_view(locale, observance)}
            </section>
        </dyn:section>
    }
}

fn psalms_view(locale: &str, psalms: &[Psalm]) -> View {
    View::Fragment(
        psalms
            .iter()
            .map(|psalm| {
                let id = psalm.citation.clone().unwrap_or_default();
                view! {
                    <article class="document" id={id}>
                        {document_view(locale, &Document::from(psalm.clone()))}
                    </article>
                }
            })
            .collect(),
    )
}

fn readings_view(locale: &str, summary: &ObservanceSummary) -> View {
    let readings = &summary.daily_office_readings;

    if readings.is_empty() {
        View::Empty
    } else {
        View::Fragment(
            readings
                .iter()
                .map(|reading| {
                    view! {
                        <>
                            <a id={&reading.citation}></a>
                            {biblical_citation(locale, &BiblicalCitation::from(reading.citation.clone()))}
                        </>
                    }
                })
                .collect(),
        )
    }
}

struct Controls {
    use_evening: Toggle,
    use_30_day_psalter: Toggle,
    use_alternate_observance: ObservancePicker,
}

impl Controls {
    fn new(summary: DailySummary) -> Self {
        let use_evening = Toggle::new(
            current_hour() >= 13,
            "time_of_day",
            t!("daily_readings.morning"),
            t!("daily_readings.evening"),
        );
        let use_30_day_psalter = Toggle::new(
            false, // TODO user preference
            "psalm_cycle",
            t!("daily_readings.daily_office_psalms"),
            t!("daily_readings.thirty_day_psalms"),
        );
        let use_alternate_observance =
            ObservancePicker::new(use_evening.toggled.clone(), Rc::new(summary));
        Self {
            use_evening,
            use_30_day_psalter,
            use_alternate_observance,
        }
    }

    fn view(&self) -> View {
        view! {
            <>
                <dyn:view view={self.use_evening.view()}/>
                <dyn:view view={self.use_30_day_psalter.view()}/>
                <dyn:view view={self.use_alternate_observance.view()}/>
            </>
        }
    }
}
