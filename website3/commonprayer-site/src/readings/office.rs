use calendar::Date;
use language::Language;
use lectionary::Reading;
use leptos::*;
use liturgy::Version;

use crate::{
    document::*,
    i18n::{use_i18n, use_language},
    settings::use_settings,
    time::{get_timezone_offset, today},
};

use super::reading_links::*;

#[derive(Clone, Debug)]
pub struct OfficeReadingsData {
    date: Memo<Date>,
    version: Memo<Version>,
    evening: Memo<bool>,
    use_lff: Memo<bool>,
    use_thirty: Memo<bool>,
    use_alternate: Memo<bool>,
    reading_links: Memo<ReadingLinks>,
    alternates: Memo<Option<(String, String)>>,
    psalms: Memo<Vec<liturgy::Psalm>>,
    readings: Memo<Vec<Reading>>,
}

pub fn office_readings_data(
    cx: Scope,
    _params: Memo<ParamsMap>,
    location: Location,
) -> OfficeReadingsData {
    // Reactive query params
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

    let evening = create_memo(cx, move |_| {
        location.query.with(|q| {
            q.get("time")
                .map(|value| value == "evening")
                .unwrap_or(false)
        })
    });

    let use_lff = create_memo(cx, move |_| {
        location.query.with(|q| {
            q.get("calendar")
                .map(|value| value == "lff")
                .unwrap_or(false)
        })
    });

    let use_thirty = create_memo(cx, move |_| {
        location
            .query
            .with(|q| q.get("psalms").map(|value| value == "30").unwrap_or(false))
    });

    let use_alternate = create_memo(cx, move |_| {
        location.query.with(|q| {
            q.get("alternate")
                .map(|value| value == "yes")
                .unwrap_or(false)
        })
    });

    // Data
    // TODO: server side for bundle size
    let language = move || Language::from_locale(&use_language(cx)());
    let summary = create_memo(cx, move |_| {
        library::CommonPrayer::daily_office_summary(&date(), language())
    });

    let morning_psalms = move || {
        summary.with(|summary| {
            if use_thirty() {
                summary.morning.thirty_day_psalms.clone()
            } else if use_alternate() {
                summary
                    .morning
                    .alternate
                    .as_ref()
                    .unwrap_or(&summary.morning.observed)
                    .daily_office_psalms
                    .clone()
            } else {
                summary.morning.observed.daily_office_psalms.clone()
            }
        })
    };

    let evening_psalms = move || {
        summary.with(|summary| {
            if use_thirty() {
                summary.evening.thirty_day_psalms.clone()
            } else if use_alternate() {
                summary
                    .evening
                    .alternate
                    .as_ref()
                    .unwrap_or(&summary.evening.observed)
                    .daily_office_psalms
                    .clone()
            } else {
                summary.evening.observed.daily_office_psalms.clone()
            }
        })
    };

    let psalms = create_memo(cx, move |_| {
        // Morning
        if !evening() {
            morning_psalms()
        }
        // Evening
        else {
            evening_psalms()
        }
    });
    let alternates = create_memo(cx, |_| None);

    let morning_readings = move || {
        summary.with(|summary| {
            if use_alternate() {
                summary
                    .morning
                    .alternate
                    .as_ref()
                    .unwrap_or(&summary.morning.observed)
                    .daily_office_readings
                    .clone()
            } else {
                summary.morning.observed.daily_office_readings.clone()
            }
        })
    };

    let evening_readings = move || {
        summary.with(|summary| {
            if use_alternate() {
                summary
                    .evening
                    .alternate
                    .as_ref()
                    .unwrap_or(&summary.evening.observed)
                    .daily_office_readings
                    .clone()
            } else {
                summary.evening.observed.daily_office_readings.clone()
            }
        })
    };

    let reading_links = create_memo(cx, move |_| {
        reading_links(
            &morning_readings(),
            &evening_readings(),
            &morning_psalms(),
            &evening_psalms(),
        )
    });

    let readings = create_memo(cx, move |_| {
        if evening() {
            evening_readings()
        } else {
            morning_readings()
        }
    });

    /* let summary = if evening {
        summary.evening
    } else {
        summary.morning
    };
    let alternates = summary.alternate.as_ref().map(|alternate| {
        (
            summary.observed.localized_name.clone(),
            alternate.localized_name.clone(),
        )
    });
    let summary = if use_alternate {
        summary.alternate.unwrap_or(summary.observed)
    } else {
        summary.observed
    };

    let readings = if evening {
        evening_readings
    } else {
        morning_readings
    }
    .into_iter()
    .map(|reading| ReadingLoader::new(&reading.citation, version, None))
    .collect::<Vec<_>>(); */

    OfficeReadingsData {
        date,
        version,
        evening,
        use_lff,
        use_thirty,
        use_alternate,
        alternates,
        psalms,
        reading_links,
        readings,
    }
}

#[component]
pub fn OfficeReadings(cx: Scope) -> Element {
    let (t, _, _) = use_i18n(cx);
    let OfficeReadingsData {
        date,
        version,
        evening,
        use_lff,
        use_alternate,
        use_thirty,
        alternates,
        psalms,
        reading_links,
        readings,
    } = use_loader(cx);

    view! {
        <div>
            // Controls
            <Form>
                // Date and Bible Version set in form in parent route, so just take them from
                // the parent and stash them in this form
                <input type="hidden" name="date" value=move || date().to_padded_string()/>
                <input type="hidden" name="version" value=move || version().to_string()/>
                <fieldset class="toggle">
                    <input id="bcp" type="radio" name="calendar" value="bcp" checked=move || !use_lff() onchange="this.form.requestSubmit()"/>
                    <label for="bcp">{t("bcp_1979")}</label>
                    <input id="lff" type="radio" name="calendar" value="lff" checked=use_lff onchange="this.form.requestSubmit()"/>
                    <label for="lff">{t("lff_2018")}</label>
                </fieldset>
                <fieldset class="toggle">
                    <input id="morning" type="radio" name="time" value="morning" checked=move || !evening() onchange="this.form.requestSubmit()"/>
                    <label for="morning">{t("daily-readings-morning")}</label>
                    <input id="evening" type="radio" name="time" value="evening" checked=evening onchange="this.form.requestSubmit()"/>
                    <label for="evening">{t("daily-readings-evening")}</label>
                </fieldset>
                <fieldset class="toggle">
                    <input id="office" type="radio" name="psalms" value="daily" checked=move || !use_thirty() onchange="this.form.requestSubmit()"/>
                    <label for="office">{t("daily-readings-daily_office_psalms")}</label>
                    <input id="30" type="radio" name="psalms" value="30" checked=use_thirty onchange="this.form.requestSubmit()"/>
                    <label for="30">{t("daily-readings-thirty_day_psalms")}</label>
                </fieldset>
                <div>
                    {move || alternates().as_ref().map(|(observed, alternate)| {
                        view! {
                            <fieldset class="toggle">
                                <input id="observed" type="radio" name="alternate" value="no" checked=move || !use_alternate() onchange="this.form.requestSubmit()"/>
                                <label for="observed">{observed}</label>
                                <input id="alternate" type="radio" name="alternate" value="yes" checked=use_alternate onchange="this.form.requestSubmit()"/>
                                <label for="alternate">{alternate}</label>
                            </fieldset>
                        }
                    })}
                </div>
            </Form>

            // Reading Links
            <ReadingLinksView reading_links/>

            // Psalms
            <section>
                <h2>{t("daily-readings-psalms")}</h2>
                <For each={psalms} key=|psalm| psalm.number>
                {|cx, psalm: &liturgy::Psalm|  view! {
                    <Psalm psalm=psalm.clone()/>
                }}
                </For>
            </section>

            // Readings
            <section>
                <h2>{t("daily-readings-readings")}</h2>
                <ReadingsView readings />
            </section>
        </div>
    }
}

#[component]
fn ReadingsView(cx: Scope, readings: Memo<Vec<Reading>>) -> Memo<Vec<Element>> {
    let (settings, _) = use_settings(cx);
    let version = move || settings.with(|s| s.bible_version);
    view! {
        <For each=readings key=|reading| reading.citation.clone()>
            {|cx, reading: &Reading| view! {
                <div>
                    <a id=&reading.citation></a>
                    <pre>{&reading.citation}</pre>
                </div>
            }}
        </For>
    }
}
