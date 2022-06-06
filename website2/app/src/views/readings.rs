use crate::views::bible_version_select_options;
use crate::WebView;
use calendar::{Date, LiturgicalDayId};
use lectionary::Reading;
use leptos2::*;
use liturgy::{BiblicalCitation, Document, Psalm, Version};

use super::DocumentView;

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
            <h1>
                <a href={&format!("/{}/holy-day/{:#?}", locale, feast)}>{localized_name}</a>
            </h1>
        },
        LiturgicalDayId::TransferredFeast(feast) => view! {
            <h1>
                <a href={&format!("/{}/holy-day/{:#?}", locale, feast)}>{localized_name}</a>
                <br/>
                {t!("daily_readings.transferred")}
            </h1>
        },
        _ => view! {
            <h1>{localized_name}</h1>
        },
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
