use api::summary::ObservanceSummary;
use calendar::Date;
use language::Language;
use lectionary::Reading;
use leptos2::*;
use library::CommonPrayer;
use liturgy::{Psalm, Version};
use serde::Deserialize;

use crate::{
    utils::time::today,
    views::{document::DocumentView, readings::*, Header},
    WebView,
};

#[derive(PartialEq)]
pub struct ReadingsPage {
    pub date: Date,
    pub summary: ObservanceSummary,
    pub version: Version,
    pub use_alternate: bool,
    pub alternates: Option<(String, String)>,
    pub evening: bool,
    pub use_thirty: bool,
    pub use_lff: bool,
    pub reading_links: ReadingLinks,
    pub psalms: Vec<Psalm>,
    pub readings: Vec<Reading>,
}

#[derive(Deserialize)]
pub struct ReadingsPageQuery {
    date: Option<String>,
    alternate: Option<String>,
    version: Option<Version>,
    time: Option<String>,
    psalms: Option<String>,
    calendar: Option<String>,
}

impl Page for ReadingsPage {
    type Params = ();
    type Query = ReadingsPageQuery;

    fn name() -> &'static str {
        "daily-readings"
    }

    fn paths() -> Vec<String> {
        vec!["".into()]
    }

    fn build_state(
        locale: &str,
        _path: &str,
        _params: Self::Params,
        query: Self::Query,
    req: &HttpRequest
    ) -> Option<Self> {
        // Build all params
        let language = Language::from_locale(locale);
        let date = query
            .date
            .as_ref()
            .and_then(|date| Date::parse_from_str(date, "%Y-%m-%d").ok())
            .unwrap_or_else(today);
        let use_alternate = query
            .alternate
            .map(|alternate| alternate == "yes")
            .unwrap_or(false);
        let version = query
            .version
            .filter(Version::is_bible_translation)
            .unwrap_or(Version::NRSV);
        let evening = query.time.map(|time| time == "evening").unwrap_or(false);
        let use_thirty = query.psalms.map(|psalms| psalms == "30").unwrap_or(false);
        let use_lff = query
            .calendar
            .map(|calendar| calendar == "lff")
            .unwrap_or(false);

        // Data
        let summary = CommonPrayer::daily_office_summary(&date, language);
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

        let summary = if evening {
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

        Some(Self {
            date,
            summary,
            version,
            use_alternate,
            alternates,
            evening,
            use_thirty,
            use_lff,
            reading_links,
            psalms: if evening {
                evening_psalms
            } else {
                morning_psalms
            },
            readings: if evening {
                evening_readings
            } else {
                morning_readings
            },
        })
    }

    fn head(&self, locale: &str) -> Vec<Node> {
        let title = format!(
            "{}: {} – {}",
            t!("toc.daily_readings"),
            self.date.to_localized_name(Language::from_locale(locale)),
            t!("common_prayer")
        );

        view! {
            <>
                <title>{title}</title>
                <link rel="stylesheet" href="/static/vars.css"/>
                <link rel="stylesheet" href="/static/general.css"/>
                <link rel="stylesheet" href="/static/document.css"/>
                <link rel="stylesheet" href="/static/daily-readings.css"/>
                <link rel="stylesheet" href="/static/display-settings.css"/>
                <link rel="stylesheet" href="/static/settings.css"/>
            </>
        }
    }

    fn body(&self, locale: &str) -> Vec<Node> {
        view! {
            <>
                {Header::new(locale, &t!("toc.daily_readings")).to_node()}
                <main>
                    // Controls
                    {readings_settings_form(locale, self.date, self.version, view! {
                        <>
                            <fieldset class="toggle">
                                <input id="bcp" type="radio" name="calendar" value="bcp" checked={!self.use_lff} onchange="this.form.submit()"/>
                                <label for="bcp">{t!("bcp_1979")}</label>
                                <input id="lff" type="radio" name="calendar" value="lff" checked={self.use_lff} onchange="this.form.submit()"/>
                                <label for="lff">{t!("lff_2018")}</label>
                            </fieldset>
                            <fieldset class="toggle">
                                <input id="morning" type="radio" name="time" value="morning" checked={!self.evening} onchange="this.form.submit()"/>
                                <label for="morning">{t!("daily_readings.morning")}</label>
                                <input id="evening" type="radio" name="time" value="evening" checked={self.evening} onchange="this.form.submit()"/>
                                <label for="evening">{t!("daily_readings.evening")}</label>
                            </fieldset>
                            <fieldset class="toggle">
                                <input id="office" type="radio" name="psalms" value="daily" checked={!self.use_thirty} onchange="this.form.submit()"/>
                                <label for="office">{t!("daily_readings.daily_office_psalms")}</label>
                                <input id="30" type="radio" name="psalms" value="30" checked={self.use_thirty} onchange="this.form.submit()"/>
                                <label for="30">{t!("daily_readings.thirty_day_psalms")}</label>
                            </fieldset>
                            {self.alternates.as_ref().map(|(observed, alternate)| {
                                view! {
                                    <fieldset class="toggle">
                                        <input id="observed" type="radio" name="alternate" value="no" checked={!self.use_alternate} onchange="this.form.submit()"/>
                                        <label for="observed">{observed}</label>
                                        <input id="alternate" type="radio" name="alternate" value="yes" checked={self.use_alternate} onchange="this.form.submit()"/>
                                        <label for="alternate">{alternate}</label>
                                    </fieldset>
                                }
                            }).unwrap_or_else(|| text(""))}
                        </>
                    })}

                    {self.observance_header_view(locale)}

                    // Psalms
                    <h2>{t!("daily_readings.psalms")}</h2>
                    {psalms_view(locale, &self.psalms)}

                    // Readings
                    <h2>{t!("daily_readings.daily_office_readings")}</h2>
                    {readings_view(locale, &self.readings, self.version)}
                </main>
            </>
        }
    }
}

impl ReadingsPage {
    fn rebuild_query_string(&self, evening: bool) -> String {
        let pairs = [
            Some(("date", self.date.to_padded_string().as_str())),
            if self.use_alternate {
                Some(("alternate", "yes"))
            } else {
                None
            },
            Some(("version", self.version.to_string().as_str())),
            if evening {
                Some(("time", "evening"))
            } else {
                None
            },
            if self.use_thirty {
                Some(("psalms", "30"))
            } else {
                None
            },
            if self.use_lff {
                Some(("calendar", "lff"))
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .map(|(k, v)| format!("{k}={v}"))
        .intersperse_with(|| "&".to_string())
        .collect::<String>();
        format!("?{pairs}")
    }

    fn link_with_citation(&self, citation: &str, evening: bool) -> String {
        format!("{}#{}", self.rebuild_query_string(evening), citation)
    }

    fn observance_header_view(&self, locale: &str) -> Vec<Node> {
        let title = title_view(
            locale,
            &self.summary.observance,
            &self.summary.localized_name,
        );

        let black_letter_days = if self.use_lff {
            &self.summary.lff_black_letter_days
        } else {
            &self.summary.bcp_black_letter_days
        }
        .iter()
        .map(|(feast, name)| {
            let url = format!("/{}/holy-day/{:#?}", locale, feast);
            view! {
                <li>
                    <a href={url}>{name}</a>
                </li>
            }
        })
        .collect::<Vec<_>>();
        let black_letter_days = view! {
            <ul class="black-letter-days">{black_letter_days}</ul>
        };

        let reading_links = self.reading_links_view();

        let collects = self
            .summary
            .collects
            .as_ref()
            .map(|collects| {
                [
                    view! {
                        <h2>{t!("lookup.collect_of_the_day")}</h2>
                    },
                    DocumentView {
                        path: vec![],
                        doc: collects,
                    }
                    .view(locale),
                ]
            })
            .into_iter()
            .flatten();

        [title, black_letter_days, reading_links]
            .into_iter()
            .chain(collects)
            .collect::<Vec<_>>()
    }

    fn reading_links_view(&self) -> Node {
        let readings_different =
            self.reading_links.morning_readings != self.reading_links.evening_readings;

        view! {
            <table class="reading-link-table">
                <tr>
                    <th>{t!("daily_readings.morning")}</th>
                    <th>{t!("daily_readings.evening")}</th>
                </tr>
                <tr>
                    <td>
                        {self.psalm_links_view(&self.reading_links.morning_psalms, false)}
                    </td>
                    <td>
                        {self.psalm_links_view(&self.reading_links.evening_psalms, true)}
                    </td>
                </tr>
                <tr>
                    <td colspan={if readings_different { "1" } else { "2" } }>
                        {self.reading_links_reading_view(&self.reading_links.morning_readings, false)}
                    </td>
                    <td>
                        {if readings_different {
                            Some(self.reading_links_reading_view(&self.reading_links.evening_readings, true))
                        } else {
                            None
                        }}
                    </td>
                </tr>
            </table>
        }
    }

    fn reading_links_reading_view(&self, readings: &[String], evening: bool) -> Node {
        let reading_links = readings
            .iter()
            .map(|citation| {
                view! {
                    <li>
                        <a href={self.link_with_citation(citation, evening)}>{citation}</a>
                    </li>
                }
            })
            .collect::<Vec<_>>();

        view! {
            <ul>{reading_links}</ul>
        }
    }

    fn psalm_links_view(&self, psalms: &[String], evening: bool) -> Node {
        let psalm_links = psalms
            .iter()
            .map(|citation| {
                view! {
                    <li>
                        <a href={self.link_with_citation(citation, evening)}>{citation}</a>
                    </li>
                }
            })
            .collect::<Vec<_>>();

        view! {
            <ul>{psalm_links}</ul>
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReadingLinks {
    morning_psalms: Vec<String>,
    evening_psalms: Vec<String>,
    morning_readings: Vec<String>,
    evening_readings: Vec<String>,
}

pub fn reading_links(
    morning_readings: &[Reading],
    evening_readings: &[Reading],
    morning_psalms: &[Psalm],
    evening_psalms: &[Psalm],
) -> ReadingLinks {
    ReadingLinks {
        morning_psalms: psalm_links(morning_psalms),
        evening_psalms: psalm_links(evening_psalms),
        morning_readings: lectionary_reading_links(morning_readings),
        evening_readings: lectionary_reading_links(evening_readings),
    }
}

fn psalm_links(psalms: &[Psalm]) -> Vec<String> {
    psalms
        .iter()
        .filter_map(|psalm| psalm.citation.clone())
        .collect()
}

fn lectionary_reading_links(readings: &[Reading]) -> Vec<String> {
    readings
        .iter()
        .map(|reading| reading.citation.clone())
        .collect()
}
