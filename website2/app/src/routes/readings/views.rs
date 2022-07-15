use crate::components::Tabs;
use crate::routes::readings::reading_loader::ReadingLoader;
use crate::WebView;
use calendar::{Date, LiturgicalDayId};
use language::Language;
use lectionary::Reading;
use leptos2::*;
use liturgy::{BiblicalCitation, Document, Psalm, Version};

use crate::routes::document::views::DocumentView;

pub fn readings_settings_form(
    locale: &str,
    initial_date: Date,
    version: Version,
    additional_fields: Vec<Node>,
) -> Node {
    view! {
        <form>
            <fieldset class="horizontal">
                <label class="stacked">
                    {t!("date")}
                    <input
                        type="date"
                        name="date"
                        value={initial_date.to_padded_string()}
                        onchange="this.form.submit()"
                    />
                </label>
                <label class="stacked">
                    {t!("settings.bible_version")}
                    <select name="version" onchange="this.form.submit()">
                        {bible_version_select_options(locale, version)}
                    </select>
                </label>
            </fieldset>
            {additional_fields}
            <noscript><input type="submit" value={t!("lectionary.go")}/></noscript>
        </form>
    }
}

pub fn title_view(locale: &str, observance: &LiturgicalDayId, localized_name: &str) -> Node {
    match observance {
        LiturgicalDayId::Feast(feast) => view! {
            <a href={&format!("/{}/holy-day/{:#?}", locale, feast)}>{localized_name}</a>
        },
        LiturgicalDayId::TransferredFeast(feast) => view! {
            <span>
                <a href={&format!("/{}/holy-day/{:#?}", locale, feast)}>{localized_name}</a>
                <br/>
                {t!("daily_readings.transferred")}
            </span>
        },
        _ => text(localized_name),
    }
}

pub fn psalms_view(locale: &str, psalms: &[Psalm]) -> Vec<Node> {
    psalms
        .iter()
        .map(|psalm| {
            let id = psalm.citation.clone().unwrap_or_default();
            let doc_view = DocumentView {
                doc: &Document::from(psalm.clone()),
                path: vec![],
                url: "",
            };
            view! {
                <article class="document" id={id}>
                    {doc_view.view(locale)}
                </article>
            }
        })
        .collect()
}

pub fn readings_view(locale: &str, readings: &[Reading], version: Version) -> Vec<Node> {
    if readings.is_empty() {
        vec![]
    } else {
        readings
            .iter()
            .flat_map(|reading| {
                let doc_view = DocumentView {
                    doc: &Document::from(BiblicalCitation::from(reading.citation.clone()))
                        .version(version),
                    path: vec![],
                    url: "",
                };

                view! {
                    <>
                        <a id={&reading.citation}></a>
                        {doc_view.view(locale)}
                    </>
                }
            })
            .collect()
    }
}

pub fn async_readings_view(locale: &str, readings: Vec<ReadingLoader>) -> Vec<Node> {
    if readings.is_empty() {
        vec![]
    } else if readings.len() == 1 {
        readings
            .into_iter()
            .flat_map(|reading| reading.view(locale, vec![]))
            .collect()
    } else {
        // start with anchors, so that navigation to a hidden tab still works
        let mut frag = readings
            .iter()
            .map(ReadingLoader::as_citation)
            .map(|citation| view! { <a id={citation}></a> })
            .collect::<Vec<_>>();
        let labels = readings
            .iter()
            .map(ReadingLoader::as_citation)
            .map(String::from)
            .collect::<Vec<_>>();
        let content = readings
            .into_iter()
            .map(|loader| view! { <div>{loader.view(locale, vec![])}</div> });
        frag.push(view! {
            <Tabs
                prop:labels={labels.clone()}
            >
                {Tabs::content(content)}
            </Tabs>
        });
        frag
    }
}

pub fn async_readings_serial_view(locale: &str, readings: Vec<ReadingLoader>) -> Vec<Node> {
    if readings.is_empty() {
        vec![]
    } else if readings.len() == 1 {
        readings
            .into_iter()
            .flat_map(|reading| reading.view(locale, vec![]))
            .collect()
    } else {
        readings
            .into_iter()
            .map(|loader| view! { <div>{loader.view(locale, vec![])}</div> })
            .collect()
    }
}

pub fn bible_version_select(locale: &str, name: &str, value: Version) -> Node {
    view! {
        <select name={name}>
            {bible_version_select_options(locale, value)}
        </select>
    }
}

pub fn bible_version_select_options(locale: &str, value: Version) -> Vec<Node> {
    let versions = match Language::from_locale(locale) {
        Language::Es => [
            Version::RV09,
            Version::NRSV,
            Version::CEB,
            Version::ESV,
            Version::KJV,
        ],
        _ => [
            Version::NRSV,
            Version::CEB,
            Version::ESV,
            Version::KJV,
            Version::RV09,
        ],
    };

    versions
        .into_iter()
        .map(|version| {
            let value_str: &'static str = version.into();
            view! {
                <option value={value_str} selected={value == version}>{version.to_string()}</option>
            }
        })
        .collect::<Vec<_>>()
}
