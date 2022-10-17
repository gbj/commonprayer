use calendar::Date;
use language::Language;
use lectionary::Reading;
use leptos::*;
use leptos_router::*;
use liturgy::Version;
use serde::{Deserialize, Serialize};

use crate::{
    document::{biblical_reading::*, psalm::*},
    i18n::{use_i18n, use_language},
    time::{get_timezone_offset, today},
};

use super::reading_links::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OfficeReadingsData {
    date: Date,
    version: Version,
    evening: bool,
    use_lff: bool,
    use_thirty: bool,
    use_alternate: bool,
    reading_links: ReadingLinks,
    alternates: Option<(String, String)>,
    psalms: Vec<liturgy::Psalm>,
    readings: Vec<Reading>,
}

pub async fn office_readings_data(cx: Scope, _params: ParamsMap, url: Url) -> OfficeReadingsData {
    // Reactive query params
    let tzoffset = get_timezone_offset(cx);

    let q = url.search_params();

    let date = q
        .get("date")
        .and_then(|date| date.parse::<Date>().ok())
        .unwrap_or_else(|| today(&tzoffset));

    let version = q
        .get("version")
        .and_then(|version| version.parse::<Version>().ok())
        .unwrap_or(Version::NRSV);

    let evening = q
        .get("time")
        .map(|value| value == "evening")
        .unwrap_or(false);

    let use_lff = q
        .get("calendar")
        .map(|value| value == "lff")
        .unwrap_or(false);

    let use_thirty = q.get("psalms").map(|value| value == "30").unwrap_or(false);

    let use_alternate = q
        .get("alternate")
        .map(|value| value == "yes")
        .unwrap_or(false);

    // Data
    let language = move || Language::from_locale(&use_language(cx)());
    let summary = library::CommonPrayer::daily_office_summary(&date, language());

    let morning_psalms = if use_thirty {
        summary.morning.thirty_day_psalms.clone()
    } else if use_alternate {
        summary
            .morning
            .alternate
            .as_ref()
            .unwrap_or(&summary.morning.observed)
            .daily_office_psalms
            .clone()
    } else {
        summary.morning.observed.daily_office_psalms.clone()
    };

    let evening_psalms = if use_thirty {
        summary.evening.thirty_day_psalms.clone()
    } else if use_alternate {
        summary
            .evening
            .alternate
            .as_ref()
            .unwrap_or(&summary.evening.observed)
            .daily_office_psalms
            .clone()
    } else {
        summary.evening.observed.daily_office_psalms.clone()
    };

    // TODO
    let alternates = None;

    let morning_readings = if use_alternate {
        summary
            .morning
            .alternate
            .as_ref()
            .unwrap_or(&summary.morning.observed)
            .daily_office_readings
            .clone()
    } else {
        summary.morning.observed.daily_office_readings.clone()
    };

    let evening_readings = if use_alternate {
        summary
            .evening
            .alternate
            .as_ref()
            .unwrap_or(&summary.evening.observed)
            .daily_office_readings
            .clone()
    } else {
        summary.evening.observed.daily_office_readings.clone()
    };

    let reading_links = reading_links(
        &morning_readings,
        &evening_readings,
        &morning_psalms,
        &evening_psalms,
    );

    let readings = if evening {
        evening_readings
    } else {
        morning_readings
    };

    let psalms = if evening {
        evening_psalms
    } else {
        morning_psalms
    };

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
    let data = use_loader::<OfficeReadingsData>(cx);

    let tzoffset = get_timezone_offset(cx);
    let query = use_query_map(cx);
    let date = create_memo(cx, move |_| {
        query.with(|q| {
            q.get("date")
                .and_then(|date| date.parse::<Date>().ok())
                .unwrap_or_else(|| today(&tzoffset))
        })
    });

    let version = create_memo(cx, move |_| {
        query.with(|q| {
            q.get("version")
                .and_then(|version| version.parse::<Version>().ok())
                .unwrap_or(Version::NRSV)
        })
    });

    let evening = create_memo(cx, move |_| {
        query.with(|q| {
            q.get("time")
                .map(|value| value == "evening")
                .unwrap_or(false)
        })
    });

    let use_lff = create_memo(cx, move |_| {
        query.with(|q| {
            q.get("calendar")
                .map(|value| value == "lff")
                .unwrap_or(false)
        })
    });

    let use_thirty = create_memo(cx, move |_| {
        query.with(|q| q.get("psalms").map(|value| value == "30").unwrap_or(false))
    });

    let use_alternate = create_memo(cx, move |_| {
        query.with(|q| {
            q.get("alternate")
                .map(|value| value == "yes")
                .unwrap_or(false)
        })
    });

    let reading_links = create_memo(cx, move |_| {
        data.read()
            .map(|data| data.reading_links)
            .unwrap_or_default()
    });

    let psalms = create_memo(cx, move |_| {
        data.read().map(|d| d.psalms).unwrap_or_default()
    });

    let readings = create_memo(cx, move |_| {
        data.read().map(|d| d.readings).unwrap_or_default()
    });

    view! { cx, 
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
                // TODO alternates
                /* <div>
                    {move || alternates().as_ref().map(|(observed, alternate)| {
                        view! { cx, 
                            <fieldset class="toggle">
                                <input id="observed" type="radio" name="alternate" value="no" checked=move || !use_alternate() onchange="this.form.requestSubmit()"/>
                                <label for="observed">{observed}</label>
                                <input id="alternate" type="radio" name="alternate" value="yes" checked=use_alternate onchange="this.form.requestSubmit()"/>
                                <label for="alternate">{alternate}</label>
                            </fieldset>
                        }
                    })}
                </div> */
            </Form>

            // Reading Links
            <ReadingLinksView reading_links/>

            // Psalms
            <section>
                <h2>{t("daily-readings-psalms")}</h2>
                <For each=psalms key=|psalm| psalm.number>
                {|cx: Scope, psalm: &liturgy::Psalm|  view! { cx, 
                    <Psalm psalm=psalm.clone()/>
                }}
                </For>
            </section>

            // Readings
            <section>
                <h2>{t("daily-readings-readings")}</h2>
                <For each=readings key=|reading| reading.citation.clone()>
                    {|cx: Scope, reading: &Reading| view! { cx, 
                        <div>
                            <a id=&reading.citation></a>
                            <BiblicalCitation citation=reading.citation.to_string()/>
                        </div>
                    }}
                </For>
            </section>
        </div>
    }
}
