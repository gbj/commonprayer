use std::rc::Rc;

use api::summary::{
    DailySummary, EucharisticLectionarySummary, EucharisticObservanceSummary, FirstLessonAndPsalm,
    ObservanceSummary, TrackedReadings,
};
use calendar::{Date, LiturgicalDayId};
use futures::{Stream, StreamExt};
use lectionary::Reading;
use leptos2::*;
use library::CommonPrayer;
use liturgy::{
    BiblicalCitation, Content, Document, GlobalPref, Lectionaries, PreferenceKey, PreferenceValue,
    Psalm, Version,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::UnwrapThrowExt;

use crate::{
    components::*,
    preferences,
    utils::{language::locale_to_language, time::current_hour},
    views::Header,
};

mod redirect;
use redirect::*;

#[derive(PartialEq)]
pub enum ReadingsPage {
    None,
    DailyOffice(Box<DailySummary>),
    Lectionary(Box<EucharisticLectionarySummary>),
}

#[derive(Deserialize)]
pub struct ReadingsPageParams {
    date: Option<String>,
    alternate_readings: Option<String>,
}

impl Page for ReadingsPage {
    type Params = ReadingsPageParams;

    fn name() -> &'static str {
        "readings"
    }

    fn paths() -> Vec<String> {
        vec![
            format!("{}/{{date}}", RedirectMode::DailyOffice),
            format!("{}/{{date}}", RedirectMode::Eucharist),
            format!(
                "{}/{{date}}/{{alternate_readings}}",
                RedirectMode::Eucharist
            ),
            "{date}".into(),
            "".into(),
        ]
    }

    fn build_state(locale: &str, path: &str, params: Self::Params) -> Option<Self> {
        let language = locale_to_language(locale);
        let date = &params.date;
        if date.is_none() && !path.contains(&format!("{}/", RedirectMode::Eucharist)) {
            Some(ReadingsPage::None)
        } else if date.is_some() && path.contains(&format!("{}/", RedirectMode::Eucharist)) {
            date.as_ref().and_then(|date| {
                Date::parse_from_str(date, "%Y-%m-%d").ok().map(|date| {
                    let alternate = params
                        .alternate_readings
                        .as_ref()
                        .and_then(|feast| serde_json::from_str(&format!(r#""{feast}""#)).ok());
                    let summary =
                        CommonPrayer::eucharistic_lectionary_summary(&date, language, alternate);

                    ReadingsPage::Lectionary(Box::new(summary))
                })
            })
        } else if let Some(date) = date {
            Date::parse_from_str(date, "%Y-%m-%d").ok().map(|date| {
                let summary = CommonPrayer::daily_office_summary(&date, language);

                ReadingsPage::DailyOffice(Box::new(summary))
            })
        } else {
            None
        }
    }

    fn head(&self, locale: &str) -> Vec<Node> {
        let title = format!(
            "{} – {}",
            if matches!(self, ReadingsPage::Lectionary(_)) {
                t!("menu.lectionary")
            } else {
                t!("toc.daily_readings")
            },
            t!("common_prayer")
        );

        // no summary means no date param is present in URL => redirect to client's current day
        let redirect = redirect_script(locale, "readings", self == &ReadingsPage::None);

        view! {
            <>
                <title>{title}</title>
                <link rel="stylesheet" href="/static/vars.css"/>
                <link rel="stylesheet" href="/static/general.css"/>
                <link rel="stylesheet" href="/static/document.css"/>
                <link rel="stylesheet" href="/static/daily-readings.css"/>
                <link rel="stylesheet" href="/static/display-settings.css"/>
                <link rel="stylesheet" href="/static/settings.css"/>
                {redirect}
            </>
        }
    }

    fn body(&self, locale: &str) -> Vec<Node> {
        view! {
            <>
                <Tabs
                    prop:labels={vec![
                        "Maria Ahlefeldt".to_string(),
                        "Carl Andersen".to_string(),
                        "Ida da Fonseca".to_string(),
                        "Peter Lange-Müller".to_string()
                    ]}
                >
                    {Tabs::content(view! {
                        <>
                            <p><a href="https://en.wikipedia.org/wiki/Maria_Theresia_Ahlefeldt">"Maria Theresia Ahlefeldt"</a> " (16 January 1755 – 20 December 1810) was a Danish, (originally German), composer. She is known as the first female composer in Denmark. Maria Theresia composed music for several ballets, operas, and plays of the royal theatre. She was given good critic as a composer and described as a “<span lang='da'>virkelig Tonekunstnerinde</span>” ('a True Artist of Music')."</p>
                            <p><a href="https://en.wikipedia.org/wiki/Joachim_Andersen_(composer)">"Carl Joachim Andersen"</a> " (29 April 1847 – 7 May 1909) was a Danish flutist, conductor and composer born in Copenhagen, son of the flutist Christian Joachim Andersen. Both as a virtuoso and as composer of flute music, he is considered one of the best of his time. He was considered to be a tough leader and teacher and demanded as such a lot from his orchestras but through that style he reached a high level."</p>
                            <p><a href="https://en.wikipedia.org/wiki/Ida_Henriette_da_Fonseca">"Ida Henriette da Fonseca"</a> " (July 27, 1802 – July 6, 1858) was a Danish opera singer and composer.  Ida Henriette da Fonseca was the daughter of Abraham da Fonseca (1776–1849) and Marie Sofie Kiærskou (1784–1863). She and her sister Emilie da Fonseca were students of Giuseppe Siboni, choir master of the Opera in Copenhagen. She was given a place at the royal Opera alongside her sister the same year she debuted in 1827."</p>
                            <p><a href="https://en.wikipedia.org/wiki/Peter_Lange-M%C3%BCller">"Peter Erasmus Lange-Müller"</a>" (1 December 1850 – 26 February 1926) was a Danish composer and pianist. His compositional style was influenced by Danish folk music and by the work of Robert Schumann; Johannes Brahms; and his Danish countrymen, including J.P.E. Hartmann."</p>
                        </>
                    })}
                </Tabs>
            </>
        }
        /*  match self {
            // JavaScript in header will redirect to today's Daily Office page
            ReadingsPage::None => view! {
                <>
                    {Header::new(locale, &t!("toc.daily_readings")).to_node()}
                    <main>
                    </main>
                </>
            },
            ReadingsPage::DailyOffice(summary) => office_body(locale, summary),
            ReadingsPage::Lectionary(summary) => eucharist_body(locale, summary),
        } */
    }
}
/*
fn office_body(locale: &str, summary: &DailySummary) -> Vec<Node> {
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
        controls.use_lff_2018.toggled.clone(),
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
        controls.use_lff_2018.toggled.clone(),
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
        controls.use_lff_2018.toggled.clone(),
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
        controls.use_lff_2018.toggled.clone(),
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

    let display_settings_menu = DisplaySettingsSideMenu::new();

    view! {
        <>
            {header_with_side_menu(&locale, &t!("toc.daily_readings"), display_settings_menu.view())}
            <dyn:main
                class={display_settings_menu.current_settings().stream().map(|settings| settings.to_class()).boxed_local()}
            >
                <label class="stacked">
                    {View::StaticText(t!("date"))}
                    <dyn:input type="date" class="centered"
                        value={summary.date.to_padded_string()}
                        on:change={
                            let locale = locale.clone();
                            move |ev: Event| redirect_to_date(&locale, event_target_value(ev), RedirectMode::DailyOffice)
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
            </dyn:main>
            {preference_status_footer(&display_settings_menu.status)}
        </>
    }
}

fn observance_view(
    locale: &str,
    use_default: bool,
    use_this_observance: impl Stream<Item = Option<bool>> + 'static,
    use_thirty_day_psalms: Behavior<bool>,
    use_lff_2018: Behavior<bool>,
    observance: &ObservanceSummary,
    thirty_day_psalms: &[Psalm],
) -> Vec<Node> {
    todo!()
}

struct Controls {
    use_evening: Toggle,
    use_30_day_psalter: Toggle,
    use_alternate_observance: ObservancePicker,
    use_lff_2018: Toggle,
}

impl Controls {
    fn new(summary: DailySummary) -> Self {
        let use_lff_2018 = Toggle::new(
            preferences::is(
                &PreferenceKey::from(GlobalPref::Calendar),
                &PreferenceValue::Local("lff2018".into()),
            ),
            "calendar",
            t!("bcp_1979"),
            t!("lff_2018"),
            None,
        );
        let use_evening = Toggle::new(
            current_hour() >= 13,
            "time_of_day",
            t!("daily_readings.morning"),
            t!("daily_readings.evening"),
            None,
        );
        let use_30_day_psalter = Toggle::new(
            preferences::is(
                &PreferenceKey::from(GlobalPref::PsalmCycle),
                &PreferenceValue::from(Lectionaries::BCP1979ThirtyDayPsalms),
            ),
            "psalm_cycle",
            t!("daily_readings.daily_office_psalms"),
            t!("daily_readings.thirty_day_psalms"),
            None,
        );
        let use_alternate_observance =
            ObservancePicker::new(use_evening.toggled.clone(), Rc::new(summary));
        Self {
            use_evening,
            use_30_day_psalter,
            use_alternate_observance,
            use_lff_2018,
        }
    }

    fn view(&self) -> View {
        view! {
            <>
                <dyn:view view={self.use_lff_2018.view()}/>
                <dyn:view view={self.use_evening.view()}/>
                <dyn:view view={self.use_30_day_psalter.view()}/>
                <dyn:view view={self.use_alternate_observance.view()}/>
            </>
        }
    }
}
 */
